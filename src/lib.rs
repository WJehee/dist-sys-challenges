use std::io::{Write, Lines, StdinLock, BufRead};

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

impl<T> Message<T> {
    pub fn from_msg(self, msg_id: usize) -> Self {
        Self {
            src: self.dst,
            dst: self.src,
            body: Body {
                msg_id: Some(msg_id),
                in_reply_to: self.body.msg_id,
                msg_type: self.body.msg_type,
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Init {
    Init { node_id: String, node_ids: Vec<String> },
    InitOk,
}

pub fn handle_init(stdin: &mut Lines<StdinLock<'_>>, stdout: &mut impl Write) -> String {
    let init_msg: Message<Init> = match parse_message(&mut *stdin) {
        Some(msg) => msg,
        None => panic!("did not receive init message first")
    };
    let node_id = match init_msg.body.msg_type {
        Init::Init{node_id, node_ids: _} => node_id,
        _ => panic!("did not receive init message first"),
    };

    let init_reply = Message {
        src: node_id.clone(),
        dst: init_msg.src,
        body: Body {
            msg_id: Some(0),
            in_reply_to: init_msg.body.msg_id,
            msg_type: Init::InitOk,
        }
    };
    serde_json::to_writer(&mut *stdout, &init_reply).unwrap();
    stdout.write_all(b"\n").unwrap();
    node_id
}

pub fn parse_message<T: DeserializeOwned> (stdin: &mut Lines<StdinLock<'_>>) -> Option<Message<T>> {
    serde_json::from_str(
        &stdin.next()
        .unwrap()
        .expect("failed to read line")
        ).unwrap_or(None)
}

pub trait Handler<T> {
    fn handle_message(&mut self, msg: Message<T>) -> Message<T>;
}

pub fn main_loop<T, H>(mut handler: H) where
T: DeserializeOwned + Serialize,
H: Handler<T>
{
    let stdin = std::io::stdin().lock();
    let mut stdin = stdin.lines();
    let mut stdout = std::io::stdout().lock();

    handle_init(&mut stdin, &mut stdout);

    loop {
        let msg: Message<T> = match parse_message(&mut stdin) {
            Some(msg) => msg,
            None => panic!("failed to parse message")
        };
        let reply = handler.handle_message(msg);

        serde_json::to_writer(&mut stdout, &reply).unwrap();
        stdout.write_all(b"\n").unwrap();
    }
}

