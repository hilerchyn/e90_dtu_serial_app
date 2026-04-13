use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // InfluxDB
    #[arg(short = 'e', long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub enable_influx: bool,
    #[arg(short = 'p', long, default_value_t = String::from("http://127.0.0.1:8181"))]
    pub influx_endpoint: String,
    #[arg(short = 't', long, default_value_t = String::from("apiv3_r04ea4SSIafZ9enFYijF4uRsLBh_1rHsIKdyyEy5jwfXQkcrdUI0sNo8MGgymxnFcJBwZxHeR6aBIIsFiPv7Gw"))]
    pub influx_token: String,
    #[arg(short = 'd', long, default_value_t = String::from("lora"))]
    pub influx_database: String,
    #[arg(long, default_value_t = 1000)]
    pub infflux_timeout: u64, // 毫秒

    // 接收消息
    #[arg(short = 'r', long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub enable_rx: bool,
    #[arg(short = 's', long, default_value_t = 100)]
    pub receive_sleep: u64, // 毫秒

    // 发送消息
    #[arg(short = 'w', long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub enable_tx: bool,
    #[arg(short = 'l', long, default_value_t = 100)]
    pub write_sleep: u64, // 毫秒

    #[arg(short = 'n', long, default_value_t = String::from("ttyUSB"))]
    pub serial_name_part: String,
    #[arg(short = 'b', long, default_value_t = 9600)]
    pub baud_rate: u32,
}
