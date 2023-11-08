use dist_sys_challenges::{Message, Body, Handler, main_loop};
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
    fn handle_message(&mut self, msg: Message<Echo>) -> Message<Echo> {
        match msg.body.msg_type {
            Echo::Echo{echo} => Message {
                src: msg.dst,
                dst: msg.src,
                body: Body {
                    msg_id: Some(1),
                    in_reply_to: Some(msg.body.msg_id.unwrap()),
                    msg_type: Echo::EchoOk{echo},
                }
            },
            _ => panic!("unexpected message type")
        }
    }
}

fn main() {
    let handler = EchoSolution;
    main_loop(handler);
}

