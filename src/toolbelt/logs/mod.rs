pub use slog::*;
use std::{fmt, result};

pub struct PrintlnSerializer;

impl Serializer for PrintlnSerializer {
    fn emit_arguments(&mut self, key: Key, val: &fmt::Arguments) -> Result {
        print!(", {}={}", key, val);
        Ok(())
    }
}

pub struct RootLog;

impl RootLog {
    pub fn get_logger(enable_debug: bool) -> Logger {
        if enable_debug {
            Logger::root(
                LevelFilter::new(RootLog, Level::Debug).fuse(),
                o!("app" => option_env!("CARGO_PKG_NAME"), "version" => option_env!("CARGO_PKG_VERSION"))
            )
        } else {
            Logger::root(
                LevelFilter::new(RootLog, Level::Info).fuse(),
                o!("app" => option_env!("CARGO_PKG_NAME"), "version" => option_env!("CARGO_PKG_VERSION"))
            )
        }
    }
}

impl Drain for RootLog {
    type Ok = ();
    type Err = ();

    fn log(
        &self,
        record: &Record,
        values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {

        print!("{}", record.msg());

        record
            .kv()
            .serialize(record, &mut PrintlnSerializer)
            .unwrap();
        values.serialize(record, &mut PrintlnSerializer).unwrap();

        println!(" ");
        Ok(())
    }
}
