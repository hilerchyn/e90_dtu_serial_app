use async_trait::async_trait;
use influxdb2::models::DataPoint;

pub struct Client {
    inner: influxdb2::Client,
}

impl Client {
    pub fn from_args(args: &crate::args::Args) -> Self {
        let inner = influxdb2::Client::new(
            args.influx_endpoint.clone(),
            args.influx_org.clone(),
            args.influx_token.clone(),
        );
        Self { inner }
    }
}

#[async_trait]
impl super::InfluxWriter for Client {
    async fn write(&self, cfg: &crate::args::Args) {
        let points = vec![
            DataPoint::builder("rx")
                //.tag("host", "server01")
                //.tag("region", "us-west")
                .field("value", 99)
                .build()
                .unwrap(),
        ];

        self.inner
            .write(&cfg.influx_bucket, futures::stream::iter(points))
            .await
            .unwrap();
    }
}
