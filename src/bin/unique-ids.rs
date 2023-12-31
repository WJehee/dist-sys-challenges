use dist_sys_challenges::{Message, Handler, main_loop};
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
    fn handle_message(&mut self, msg: Message<Generate>) -> Message<Generate> {
        let mut reply = msg.from_msg(self.msg_count);
        reply.body.msg_type = match reply.body.msg_type {
            Generate::Generate => {
                Generate::GenerateOk {id: format!("{0} - {1}", reply.src, self.msg_count.clone())}
            },
            _ => panic!("unexpected message type")
        };
        self.msg_count += 1;
        reply
    }
}

fn main() {
    let handler = UniqueIDSolution {msg_count: 1};
    main_loop(handler);
}

