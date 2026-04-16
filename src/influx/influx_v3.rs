use async_trait::async_trait;
use chrono::{Local, Utc};
use influxdb3::InfluxDbClientBuilder;
use influxdb3::http_client::InfluxDbClient;
use influxdb3::{DataPointBuilder, FieldDataType};
use crate::args;

pub struct Client {
    inner: InfluxDbClient,
}

impl Client {
    pub fn from_args(args: &args::Args) -> Self {
        let influxdb_client = InfluxDbClientBuilder::new()
            .server_endpoint(&args.influx_endpoint)
            .token(&args.influx_token)
            .database(&args.influx_database)
            .build()
            .unwrap();

        Self {
            inner: influxdb_client,
        }
    }
}

#[async_trait]
impl super::InfluxWriter for Client {
    async fn write(&self, cfg: &args::Args) {
        let now = Local::now();
        let formatted_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

        // WRITING
        let data_point = DataPointBuilder::new()
            .table("signle")
            .field("point", FieldDataType::Integer(99))
            .datetime(Utc::now())
            .build()
            .unwrap();

        match self.inner.write_one(data_point).await {
            Ok(cluster_uuid_opt) => {
                if cfg.debug {
                    println!(
                        "[{formatted_time_str}] writing db successful : cluster_uuid = {:?}",
                        cluster_uuid_opt
                    );
                }
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
