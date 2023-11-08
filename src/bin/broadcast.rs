use std::collections::HashMap;

use dist_sys_challenges::{Message, Body, Handler, main_loop};
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

struct BroadcastSolution {
    msg_count: usize,
    messages_seen: Vec<usize>,
}

impl Handler<Broadcast> for BroadcastSolution {
    fn handle_message(&mut self, msg: Message<Broadcast>) -> Message<Broadcast> {
        let reply = match msg.body.msg_type {
            Broadcast::Broadcast{message} => {
                self.messages_seen.push(message);
                Message {
                    src: msg.dst,
                    dst: msg.src,
                    body: Body {
                        msg_id: Some(self.msg_count),
                        in_reply_to: Some(msg.body.msg_id.unwrap()),
                        msg_type: Broadcast::BroadcastOk
                    }
                }
            },
            Broadcast::Read => Message {
                src: msg.dst,
                dst: msg.src,
                body: Body {
                    msg_id: Some(self.msg_count),
                    in_reply_to: Some(msg.body.msg_id.unwrap()),
                    msg_type: Broadcast::ReadOk { messages: self.messages_seen.clone() }
                }
            },
            Broadcast::Topology{topology: _topology} => Message {
                src: msg.dst,
                dst: msg.src,
                body: Body {
                    msg_id: Some(self.msg_count),
                    in_reply_to: Some(msg.body.msg_id.unwrap()),
                    msg_type: Broadcast::TopologyOk
                }
            },
            _ => panic!("Unexpected message type")
        };
        self.msg_count += 1;
        reply
    }
}

fn main() {
    let handler = BroadcastSolution {
        msg_count: 1,
        messages_seen: Vec::new(),
    };
    main_loop(handler);
}

