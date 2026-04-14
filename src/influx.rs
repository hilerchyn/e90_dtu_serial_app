use chrono::{Local, Utc};
use futures::prelude::*;

use influxdb3::InfluxDbClientBuilder;
use influxdb3::http_client::InfluxDbClient;
use influxdb3::{DataPointBuilder, FieldDataType};

use influxdb2::Client;
use influxdb2::models::DataPoint;

use crate::args;

pub struct Influx<'a> {
    pub cfg: &'a args::Args,

    client_v3: InfluxDbClient,
    client_v2: Client,
}

impl<'a> Influx<'a> {
    pub fn init(args: &'a args::Args) -> Self {
        // CONNECTING
        let influxdb_client = InfluxDbClientBuilder::new()
            .server_endpoint(&args.influx_endpoint)
            .token(&args.influx_token)
            .database(&args.influx_database)
            .build()
            .unwrap();

        let influxdb2_client = Client::new(
            args.influx_endpoint.clone(),
            args.influx_org.clone(),
            args.influx_token.clone(),
        );

        Self {
            cfg: args,
            client_v3: influxdb_client,
            client_v2: influxdb2_client,
        }
    }

    // 写入数据
    pub async fn write(&self) {
        if !self.cfg.influx_enable {
            return;
        }

        let now = Local::now();
        let formatted_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

        if self.cfg.influx_version == 2 {
            let points = vec![
                DataPoint::builder("rx")
                    //.tag("host", "server01")
                    //.tag("region", "us-west")
                    .field("value", 99)
                    .build()
                    .unwrap(),
            ];

            self.client_v2
                .write(&self.cfg.influx_bucket, stream::iter(points))
                .await
                .unwrap();

            return;
        }

        // WRITING
        let data_point = DataPointBuilder::new()
            .table("signle")
            .field("point", FieldDataType::Integer(99))
            .datetime(Utc::now())
            .build()
            .unwrap();

        match self.client_v3.write_one(data_point).await {
            Ok(cluster_uuid_opt) => {
                if self.cfg.debug {
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
