use actix::*;
use mqttc::Client;
use mqttc::{PubOpt, PubSub};
use settings::Mqtt;
use std::time;

pub fn new_client(cfg: Mqtt) -> Client {
    use mqttc::{ClientOptions, ReconnectMethod};
    use netopt::{NetworkOptions, SslContext};

    // Using ssl network connection
    let mut netopt = NetworkOptions::new();
    if cfg.tls {
        netopt.tls(SslContext::default());
    }

    // Using credentials for client
    let mut opts = ClientOptions::new();
    opts.set_username(cfg.username.to_string());
    opts.set_password(cfg.password.to_string());
    let timeout = time::Duration::from_secs(1_0000);
    opts.set_reconnect(ReconnectMethod::ReconnectAfter(timeout));

    let x = opts.connect(cfg.url, netopt).unwrap();
    x
}

pub struct Publisher {
    pub client: Client,
    pub topic: String,
}
pub struct Report {
    pub json: String,
}
impl Message for Report {
    type Result = String;
}

impl Actor for Publisher {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("{}", "publisher started");
    }
}

impl Handler<Report> for Publisher {
    type Result = String; // <- Message response type

    fn handle(&mut self, msg: Report, _ctx: &mut Context<Self>) -> Self::Result {
        self.client
            .publish(self.topic.clone(), msg.json, PubOpt::at_least_once())
            .ok();

        String::from("ok")
    }
}
