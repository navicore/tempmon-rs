use actix::*;
use futures::{future, Future};
use publisher::Report;
use settings::Settings;
use std::env;

fn node_name() -> String {
    match env::var("RESIN_DEVICE_NAME_AT_INIT") {
        Ok(val) => val,
        Err(_) => String::from("me!"),
    }
}

pub struct Heartbeater {
    pub publisher: Recipient<Report>,
}
pub struct Beat();
impl Message for Beat {
    type Result = String;
}

impl Actor for Heartbeater {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("{}", "beater started");
    }
}

impl Handler<Beat> for Heartbeater {
    type Result = String; // <- Message response type

    fn handle(&mut self, _msg: Beat, _ctx: &mut Context<Self>) -> Self::Result {
        let settings = Settings::new().unwrap();
        let json = format!(
            r#"{{"heartbeat": "{} {}"}}"#,
            settings.heartbeat_template,
            node_name()
        );
        let res = self.publisher.send(Report { json });
        Arbiter::spawn(res.then(|res| {
            match res {
                Ok(result) => println!("Heartbeat Report: {}", result),
                Err(err) => panic!("Bad report: {}", err),
            }
            future::result(Ok(()))
        }));
        String::from("ok")
    }
}
