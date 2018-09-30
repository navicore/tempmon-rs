use actix::*;
use futures::{future, Future};
use publisher::Report;
use settings::Settings;
use std::env;

use mraa::bindings::aio::*;

pub fn monitor(addr: Recipient<MonitorCmd>) {
    let res = addr.send(MonitorCmd());
    Arbiter::spawn(res.then(|res| {
        match res {
            Ok(result) => println!("MonitorCmd: {}", result),
            Err(err) => panic!("Bad beat: {}", err),
        }

        future::result(Ok(()))
    }));
}

fn node_name() -> String {
    match env::var("RESIN_DEVICE_NAME_AT_INIT") {
        Ok(val) => val,
        Err(_) => String::from("me!"),
    }
}

fn temp() -> i32 {
    let adc_a0: mraa_aio_context = unsafe { mraa_aio_init(0) };
    if adc_a0.is_null() {
        panic!("Failed to initialise aio context.");
    }
    //
    // //loop {
    // let adc_value: u32 = unsafe { mraa_aio_read(adc_a0) };
    // let adc_value_float: f32 = unsafe { mraa_aio_read_float(adc_a0) };
    //
    // println!("ADC A0 read {}", adc_value);
    // println!("ADC A0 read float - {:.5}", adc_value_float);
    // //}
    //
    unsafe { mraa_aio_close(adc_a0) };

    40
}

pub struct TempMon {
    pub publisher: Recipient<Report>,
}
pub struct MonitorCmd();
impl Message for MonitorCmd {
    type Result = String;
}

impl Actor for TempMon {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("{}", "tempmon started");
    }
}

impl Handler<MonitorCmd> for TempMon {
    type Result = String; // <- Message response type

    fn handle(&mut self, _msg: MonitorCmd, _ctx: &mut Context<Self>) -> Self::Result {
        let settings = Settings::new().unwrap();
        let json = format!(
            r#"{{"tempmon": "{} {} is {}"}}"#,
            settings.tempmon_template,
            node_name(),
            temp()
        );
        let res = self.publisher.send(Report { json });
        Arbiter::spawn(res.then(|res| {
            match res {
                Ok(result) => println!("TempMon Report: {}", result),
                Err(err) => panic!("Bad report: {}", err),
            }
            future::result(Ok(()))
        }));
        String::from("ok")
    }
}
