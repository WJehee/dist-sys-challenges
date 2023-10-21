use std::io::{BufRead, Write};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message<T> {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Body<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Body<T> {
    #[serde(flatten)]
    pub msg_type: T,
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Init {
    Init { node_id: String, node_ids: Vec<String> },
    InitOk,
}

pub fn handle_init(stdin: impl BufRead, mut stdout: impl Write) {
    let mut stdin = stdin.lines();

    let init_msg: Message<Init>= serde_json::from_str(
        &stdin.next()
            .unwrap()
            .expect("failed to read line")
    ).expect("unable to parse message");

    let node_id = match init_msg.body.msg_type {
        Init::Init{node_id, node_ids: _} => node_id,
        _ => panic!("did not receive init message first"),
    };

    let init_reply = Message {
        src: node_id,
        dst: init_msg.src,
        body: Body {
            msg_id: Some(0),
            in_reply_to: init_msg.body.msg_id,
            msg_type: Init::InitOk,
        }
    };
    serde_json::to_writer(&mut stdout, &init_reply).unwrap();
    stdout.write_all(b"\n").unwrap();
}

