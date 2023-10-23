use std::io::{BufRead, Write};

use dist_sys_challenges::{Message, Body, handle_init, parse_message};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Generate {
    Generate,
    GenerateOk{id: String},
}

fn main() {
    let stdin = std::io::stdin().lock();
    let mut stdin = stdin.lines();
    let mut stdout = std::io::stdout().lock();

    let my_id = handle_init(&mut stdin, &mut stdout);
    let mut msg_count = 1;

    loop {
        let msg: Message<Generate> = match parse_message(&mut stdin) {
            Some(msg) => msg,
            None => panic!("failed to parse message")
        };
        let reply = match msg.body.msg_type {
            Generate::Generate => Message {
                src: msg.dst,
                dst: msg.src,
                body: Body {
                    msg_id: Some(msg_count),
                    in_reply_to: Some(msg.body.msg_id.unwrap()),
                    msg_type: Generate::GenerateOk {id: format!("{0} - {msg_count}", my_id.clone())}
                }
            },
            _ => panic!("unexpected message type")
        };
        msg_count += 1;

        serde_json::to_writer(&mut stdout, &reply).unwrap();
        stdout.write_all(b"\n").unwrap();
    }
}
