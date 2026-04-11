use chrono::{Local, Utc};
use influxdb3::InfluxDbClientBuilder;
use influxdb3::{DataPointBuilder, FieldDataType};
use serialport;
use std::io::Read;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    // CONNECTING
    let influxdb_client = InfluxDbClientBuilder::new()
        .server_endpoint("http://127.0.0.1:8181")
        .token("apiv3_r04ea4SSIafZ9enFYijF4uRsLBh_1rHsIKdyyEy5jwfXQkcrdUI0sNo8MGgymxnFcJBwZxHeR6aBIIsFiPv7Gw")
        .database("lora")
        .build().unwrap();

    // 1. Configure and open the port
    let mut port = serialport::new("/dev/ttyUSB0", 9600) // Replace with your port
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    // 2. Create a buffer to hold the incoming data
    let mut serial_buf: Vec<u8> = vec![0; 256];

    // 3. Continuously read in a loop
    loop {
        let now = Local::now();
        let formatted_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                // 't' is the number of bytes read
                //println!("[{formatted_time_str}] Received: {:?}", &serial_buf[..t]);

                println!(
                    "[{formatted_time_str}] Received: {:?}",
                    String::from_utf8(serial_buf.clone())
                );

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
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => {
                eprintln!("[{formatted_time_str}] {:?}", e);

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

        thread::sleep(Duration::from_millis(50));
        let buf = "from_mac_mini".as_bytes();
        match port.write(buf) {
            Ok(_) => {
                println!("[{formatted_time_str}] write success");
            }
            Err(e) => {
                println!("[{formatted_time_str}] write error: {}", e);
            }
        }
    }
}
