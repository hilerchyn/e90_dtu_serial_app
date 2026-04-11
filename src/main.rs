use chrono::Local;
use serialport;
use std::io::Read;
use std::thread;
use std::time::Duration;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

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
                println!("[{formatted_time_str}] Received: {:?}", &serial_buf[..t]);

                println!(
                    "[{formatted_time_str}] Received: {:?}",
                    String::from_utf8(serial_buf.clone())
                );
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("[{formatted_time_str}] {:?}", e),
        }

        thread::sleep(Duration::from_secs(3));
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
