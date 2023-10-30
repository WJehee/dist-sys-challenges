use std::{io::{BufRead, Write}, collections::HashMap};

use dist_sys_challenges::{Message, Body, handle_init, parse_message};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Broadcast {
    Broadcast{message: usize},
    BroadcastOk,
    Read,
    ReadOk{messages: Vec<usize>},
    Topology{topology: HashMap<String, Vec<String>>},
    TopologyOk,
}

fn main() {
    let stdin = std::io::stdin().lock();
    let mut stdin = stdin.lines();
    let mut stdout = std::io::stdout().lock();

    let _my_id = handle_init(&mut stdin, &mut stdout);

    let mut msg_count = 1;
    let mut messages_seen: Vec<usize> = Vec::new();
    loop {
        let msg: Message<Broadcast> = match parse_message(&mut stdin) {
            Some(msg) => msg,
            None => panic!("failed to parse message")
        };
        let reply = match msg.body.msg_type {
            Broadcast::Broadcast{message} => {
                messages_seen.push(message);
                Message {
                    src: msg.dst,
                    dst: msg.src,
                    body: Body {
                        msg_id: Some(msg_count),
                        in_reply_to: Some(msg.body.msg_id.unwrap()),
                        msg_type: Broadcast::BroadcastOk
                    }
                }
            },
            Broadcast::Read => Message {
                src: msg.dst,
                dst: msg.src,
                body: Body {
                    msg_id: Some(msg_count),
                    in_reply_to: Some(msg.body.msg_id.unwrap()),
                    msg_type: Broadcast::ReadOk { messages: messages_seen.clone() }
                }
            },
            Broadcast::Topology{topology: _topology} => Message {
                src: msg.dst,
                dst: msg.src,
                body: Body {
                    msg_id: Some(msg_count),
                    in_reply_to: Some(msg.body.msg_id.unwrap()),
                    msg_type: Broadcast::TopologyOk
                }
            },
            _ => panic!("Unexpected message type")
        };

        serde_json::to_writer(&mut stdout, &reply).unwrap();
        stdout.write_all(b"\n").unwrap();
        msg_count += 1;
    }
}
