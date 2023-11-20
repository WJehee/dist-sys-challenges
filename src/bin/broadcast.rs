use std::collections::HashMap;

use dist_sys_challenges::{Message, Handler, main_loop};
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
        let mut reply = msg.from_msg(self.msg_count);
        reply.body.msg_type = match reply.body.msg_type {
            Broadcast::Broadcast{message} => {
                self.messages_seen.push(message);
                Broadcast::BroadcastOk
            },
            Broadcast::Read => {
                Broadcast::ReadOk { messages: self.messages_seen.clone() }
            },
            Broadcast::Topology{topology: _topology} => {
                Broadcast::TopologyOk
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

