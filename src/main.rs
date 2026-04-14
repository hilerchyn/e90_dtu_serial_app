mod args;

use chrono::{Local, Utc};
use clap::Parser;
use influxdb3::InfluxDbClientBuilder;
use influxdb3::http_client::InfluxDbClient;
use influxdb3::{DataPointBuilder, FieldDataType};
use serialport::{self, SerialPort};
use std::io::Read;
use std::thread;
use std::time::Duration;

// 串口接收函数
async fn rx_function<F>(
    args: &args::Args,
    serial_port: &mut Box<dyn SerialPort>,
    influxdb_client: &InfluxDbClient,
    callback: Option<F>,
) where
    F: Fn(&args::Args, &mut Box<dyn SerialPort>),
{
    if !args.rx_enable {
        return;
    }

    if args.rx_sleep > 0 {
        thread::sleep(Duration::from_millis(args.rx_sleep));
    }

    let mut serial_buf: Vec<u8> = vec![0; 256];
    let now = Local::now();
    let formatted_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    match serial_port.read(serial_buf.as_mut_slice()) {
        Ok(t) => {
            // 't' is the number of bytes read
            if args.debug {
                println!("[{formatted_time_str}] Received: {:?}", &serial_buf[..t]);
            }

            println!(
                "[{formatted_time_str}] Received: {:?}",
                String::from_utf8(serial_buf.clone())
            );

            if args.influx_enable {
                // WRITING
                let data_point = DataPointBuilder::new()
                    .table("signle")
                    .field("point", FieldDataType::Integer(99))
                    .datetime(Utc::now())
                    .build()
                    .unwrap();

                match influxdb_client.write_one(data_point).await {
                    Ok(cluster_uuid_opt) => {
                        println!(
                            "[{formatted_time_str}] writing db successful : cluster_uuid = {:?}",
                            cluster_uuid_opt
                        );
                    }
                    Err(error_detail) => {
                        println!(
                            "[{formatted_time_str}] write db failure : {:?}",
                            error_detail
                        );
                    }
                }
            }

            if let Some(tx_fn) = callback {
                tx_fn(args, serial_port);
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
        Err(e) => {
            eprintln!("[{formatted_time_str}] {:?}", e);

            if args.influx_enable {
                // WRITING
                let data_point = DataPointBuilder::new()
                    .table("signle")
                    .field("point", FieldDataType::Integer(199))
                    .datetime(Utc::now())
                    .build()
                    .unwrap();

                match influxdb_client.write_one(data_point).await {
                    Ok(cluster_uuid_opt) => {
                        println!(
                            "[{formatted_time_str}] writing db successful : cluster_uuid = {:?}",
                            cluster_uuid_opt
                        );
                    }
                    Err(error_detail) => {
                        println!(
                            "[{formatted_time_str}] write db failure : {:?}",
                            error_detail
                        );
                    }
                }
            }
        }
    }
}

// 串口发送函数
fn tx_callback(args: &args::Args, serial_port: &mut Box<dyn SerialPort>) {
    // 收到消息以后再写入消息
    if !args.tx_enable {
        return;
    }

    if args.tx_sleep > 0 {
        thread::sleep(Duration::from_millis(args.tx_sleep));
    }

    let now = Local::now();
    let formatted_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let buf = "from_mac_mini".as_bytes();
    match serial_port.write(buf) {
        Ok(_) => {
            println!("[{formatted_time_str}] write success");
        }
        Err(e) => {
            println!("[{formatted_time_str}] write error: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    println!("arguments: {:#?}", args);

    let ports = serialport::available_ports().expect("No ports found!");
    let mut serial_port = String::from("/dev/ttyUSB0");
    for p in ports {
        println!("{}", p.port_name);
        if p.port_name.contains(args.serial_name_part.as_str()) {
            serial_port = p.port_name;
        }
    }

    // CONNECTING
    let influxdb_client = InfluxDbClientBuilder::new()
        .server_endpoint(&args.influx_endpoint)
        .token(&args.influx_token)
        .database(&args.influx_database)
        .build()
        .unwrap();

    // 1. Configure and open the port
    let mut port = serialport::new(serial_port, args.serial_baud_rate) // Replace with your port
        .timeout(Duration::from_millis(args.infflux_timeout))
        .open()
        .expect("Failed to open port");

    // 2. Create a buffer to hold the incoming data
    // WARN: 全局缓存
    //let mut serial_buf: Vec<u8> = vec![0; 256];

    // 3. Continuously read in a loop
    loop {
        // 先接收(rx)再发送(tx)
        if args.direction == "rx" {
            rx_function(&args, port.by_ref(), &influxdb_client, Some(tx_callback)).await;
        }
        // 先发送(tx)再接收(rx)
        if args.direction == "tx" {
            // 收到消息以后再写入消息
            if !args.tx_enable {
                continue;
            }

            if args.tx_sleep > 0 {
                thread::sleep(Duration::from_millis(args.tx_sleep));
            }

            let now = Local::now();
            let formatted_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let buf = "from_mac_mini".as_bytes();
            match port.write(buf) {
                Ok(_) => {
                    println!("[{formatted_time_str}] write success");
                    rx_function(
                        &args,
                        port.by_ref(),
                        &influxdb_client,
                        None::<Box<dyn Fn(&args::Args, &mut Box<dyn SerialPort>)>>,
                    )
                    .await;
                }
                Err(e) => {
                    println!("[{formatted_time_str}] write error: {}", e);
                }
            }
        }
    }
}
