use std::io::{BufRead, Write};

use dist_sys_challenges::{Message, Body, handle_init, parse_message};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Echo {
    Echo { echo: String },
    EchoOk {echo: String},
}

fn main() {
    let stdin = std::io::stdin().lock();
    let mut stdin = stdin.lines();
    let mut stdout = std::io::stdout().lock();

    handle_init(&mut stdin, &mut stdout);

    loop {
        let msg: Message<Echo> = match parse_message(&mut stdin) {
            Some(msg) => msg,
            None => panic!("failed to parse message")
        };
        let reply = Message {
           src: msg.dst,
           dst: msg.src,
           body: Body {
               msg_id: Some(1),
               in_reply_to: Some(msg.body.msg_id.unwrap()),
               msg_type: match msg.body.msg_type {
                    Echo::Echo{echo} => Echo::EchoOk{echo},
                    _ => panic!("wrong message type")
               }
           }
        };
        serde_json::to_writer(&mut stdout, &reply).unwrap();
        stdout.write_all(b"\n").unwrap();
    }
}

