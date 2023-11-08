use dist_sys_challenges::{Message, Body, Handler, main_loop};
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
        let reply = match msg.body.msg_type {
            Generate::Generate => Message {
                src: msg.dst.clone(),
                dst: msg.src,
                body: Body {
                    msg_id: Some(self.msg_count),
                    in_reply_to: Some(msg.body.msg_id.unwrap()),
                    msg_type: Generate::GenerateOk {id: format!("{0} - {1}", msg.dst, self.msg_count.clone())}
                }
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

