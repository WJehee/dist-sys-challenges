use std::{io::{BufRead, Lines, StdinLock, Write}, sync::mpsc::{channel, Sender}};

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

#[derive(Debug)]
pub enum Event<T> {
    Message(Message<T>),
    Body(Body<T>),
}

impl<T: Serialize> Message<T> {
    pub fn from_msg(self, msg_id: &mut usize) -> Self {
        *msg_id += 1;
        Self {
            src: self.dst,
            dst: self.src,
            body: Body {
                msg_id: Some(*msg_id),
                in_reply_to: self.body.msg_id,
                msg_type: self.body.msg_type,
            }
        }
    }

    pub fn send(self, writer: &mut impl Write) {
        serde_json::to_writer(&mut *writer, &self).unwrap();
        writer.write_all(b"\n").unwrap();
    }
}

pub trait Handler<T> {
    fn initialize(&mut self, node_id: String, sender: Sender<Event<T>>);
    fn handle_event(&mut self, msg: Event<T>, writer: &mut impl Write);
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Init {
    Init { node_id: String, node_ids: Vec<String> },
    InitOk,
}

pub fn main_loop<T, H>(mut handler: H) where
T: DeserializeOwned + Serialize + Send + 'static,
H: Handler<T>
{
    let node_id = handle_init();
    let (send_channel, recv_channel) = channel();
    handler.initialize(node_id, send_channel.clone());

    std::thread::spawn(move || {
        let stdin = std::io::stdin().lock();
        let mut stdin = stdin.lines();
        loop {
            match parse_message(&mut stdin) {
                Some(msg) => send_channel.send(Event::Message(msg)).expect("Failed sending over channel"),
                None => {},
            };
        }
    });

    let mut stdout = std::io::stdout().lock();
    loop {
        for event in &recv_channel {
            handler.handle_event(event, &mut stdout);
        }
    }
}

pub fn handle_init() -> String {
    let stdin = std::io::stdin().lock();
    let mut stdin = stdin.lines();
    let mut stdout = std::io::stdout().lock();

    let init_msg: Message<Init> = match parse_message(&mut stdin) {
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
    init_reply.send(&mut stdout);
    node_id
}

pub fn parse_message<T: DeserializeOwned> (stdin: &mut Lines<StdinLock<'_>>) -> Option<Message<T>> {
    serde_json::from_str(
        &stdin.next()
        .unwrap()
        .expect("failed to read line")
        ).unwrap_or(None)
}



