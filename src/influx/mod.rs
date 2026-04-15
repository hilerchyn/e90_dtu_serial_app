mod influx_v2;
mod influx_v3;

use async_trait::async_trait;

use crate::args;

#[async_trait]
pub trait InfluxWriter: Send + Sync {
    async fn write(&self, cfg: &args::Args);
}

pub struct Influx<'a> {
    pub cfg: &'a args::Args,
    writer: Box<dyn InfluxWriter>,
}

impl<'a> Influx<'a> {
    pub fn init(args: &'a args::Args) -> Self {
        let writer: Box<dyn InfluxWriter> = if args.influx_version == 2 {
            Box::new(influx_v2::Client::from_args(args))
        } else {
            Box::new(influx_v3::Client::from_args(args))
        };

        Self { cfg: args, writer }
    }

    // 写入数据
    pub async fn write(&self) {
        if !self.cfg.influx_enable {
            return;
        }

        self.writer.write(self.cfg).await;
    }
}
