//
// strongly typed json parser

extern crate ws;
extern crate reqwest;
use std::collections::HashMap;
/*
struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake -> Result<()>) {
        println!("on_open")
    }
    fn on_message(&mut self, msg: Message -> Result<()>){
        println!("Got message: {}", msg);
        //self.out.close(CloseCode::Normal)
    }
    fn on_close(&mut self, code: CloseCode, reason: &str){
        println!("Slack WebSocket connection closing: {}", reason);
        panic!("I should learn to recover from thiiiiisss")
    }
}

fn ws_connect(url) {
    connect(url, |out| Client { out: out }).unwrap()
}
 */

// TODO make into result
fn connect(token: &str) {
    let mut params = HashMap::new();
    params.insert("token", token);
    let client = reqwest::Client::new();
    let mut resp = client.post("https://slack.com/api/rtm.connect")
        .form(&params)
        .send()?;
    if resp.status().is_success() {
        println!("Sucess");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect(){
        connect("xoxb-4823510954-495275741654-B6uS97d59DnhFlDrOl1OxIIL")
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
