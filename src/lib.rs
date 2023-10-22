use std::io::{Write, Lines, StdinLock};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

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

pub fn handle_init(stdin: &mut Lines<StdinLock<'_>>, stdout: &mut impl Write) {
    let init_msg: Message<Init> = match parse_message(&mut *stdin) {
        Some(msg) => msg,
        None => panic!("did not receive init message first")
    };
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
    serde_json::to_writer(&mut *stdout, &init_reply).unwrap();
    stdout.write_all(b"\n").unwrap();
}

pub fn parse_message<T: DeserializeOwned> (stdin: &mut Lines<StdinLock<'_>>) -> Option<Message<T>> {
    serde_json::from_str(
        &stdin.next()
              .unwrap()
              .expect("failed to read line")
    ).unwrap_or(None)
}

