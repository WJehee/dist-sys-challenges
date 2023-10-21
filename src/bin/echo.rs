use std::io::{BufRead, Write};

use Dist_sys_challenges::{Message, HandleMessage, Init, Body, handle_init};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Echo { echo: String },
    EchoOk {echo: String},
}

type EchoMessage = Message<Type>;

fn main() {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    handle_init(stdin, stdout);
    
    // TODO: process it
    // TODO: write msg to std::out
}

