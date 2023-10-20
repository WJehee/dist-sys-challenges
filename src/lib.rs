use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Message {
    source: String,
    destination: String,
    body: Body,
}

#[derive(Deserialize, Serialize)]
pub struct Body {
    msg_type: String,
    msg_id: Option<isize>,
    in_reply_to: Option<isize>,
}

