use actix::*;
use futures::{future, Future};
use publisher::Report;
use settings::Settings;
use std::env;

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
            r#"{{"tempmon": "{} {}"}}"#,
            settings.tempmon_template,
            node_name()
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
