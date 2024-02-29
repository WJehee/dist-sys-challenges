use std::{io::Write, sync::mpsc::Sender};

use dist_sys_challenges::{main_loop, Event, Handler};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Generate {
    Generate,
    GenerateOk{id: String},
}

struct UniqueIDSolution {
    msg_count: usize,
}

impl Handler<Generate> for UniqueIDSolution {
    fn initialize(&mut self, _node_id: String, _sender: Sender<Event<Generate>>) {}

    fn handle_event(&mut self, msg: Event<Generate>, writer: &mut impl Write) {
        if let Event::Message(msg) = msg {
            let mut reply = msg.from_msg(&mut self.msg_count);
            reply.body.msg_type = match reply.body.msg_type {
                Generate::Generate => {
                    Generate::GenerateOk {id: format!("{0} - {1}", reply.src, self.msg_count.clone())}
                },
                _ => panic!("unexpected message type")
            };
            reply.send(writer);
        }
    }
}

fn main() {
    let handler = UniqueIDSolution {msg_count: 1};
    main_loop(handler);
}

