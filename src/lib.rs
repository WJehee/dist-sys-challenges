use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Deserialize, Serialize)]
pub struct Body {
    #[serde(flatten)]
    msg_type: Type,
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Echo { echo: String },
    EchoOk {echo: String},
}

