use std::{io::Write, sync::mpsc::Sender};

use dist_sys_challenges::{main_loop, Event, Handler};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Echo {
    Echo { echo: String },
    EchoOk {echo: String},
}

struct EchoSolution;

impl Handler<Echo> for EchoSolution {
    fn initialize(&mut self, _node_id: String, _sender: Sender<Event<Echo>>) {}

    fn handle_event(&mut self, event: Event<Echo>, writer: &mut impl Write) {
        if let Event::Message(msg) = event {
            let mut reply = msg.from_msg(&mut 0);
            reply.body.msg_type = match reply.body.msg_type {
                Echo::Echo{echo} => {
                    Echo::EchoOk{echo}
                },
                _ => panic!("unexpected message type")
            };
            reply.send(writer);
        }
    }
}

fn main() {
    let handler = EchoSolution;
    main_loop(handler);
}

