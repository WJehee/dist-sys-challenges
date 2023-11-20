use dist_sys_challenges::{Message, Handler, main_loop};
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
        let mut reply = msg.from_msg(1);
        reply.body.msg_type = match reply.body.msg_type {
            Echo::Echo{echo} => {
                Echo::EchoOk{echo}
            },
            _ => panic!("unexpected message type")
        };
        reply
    }
}

fn main() {
    let handler = EchoSolution;
    main_loop(handler);
}

