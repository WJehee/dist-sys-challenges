use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::time::{Duration, SystemTime};

use dist_sys_challenges::{Message, Handler, main_loop};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Broadcast {
    Broadcast{message: usize},
    BroadcastOk,
    Read,
    ReadOk{messages: HashSet<usize>},
    Topology{topology: HashMap<String, Vec<String>>},
    TopologyOk,
    Gossip{messages_seen: HashSet<usize>},
}

struct BroadcastSolution {
    node_id: String,
    msg_count: usize,
    prev_time: SystemTime,
    messages_seen: HashSet<usize>,
    // propagated: HashMap<usize, Vec<String>>,
    topology: Option<Vec<String>>,
}

impl BroadcastSolution {
    fn gossip(&self, writer: &mut impl Writer) {
        if let Some(topology) = &self.topology {
            for node in topology {
                
            }
        }
    }
}

impl Handler<Broadcast> for BroadcastSolution {
    fn initialize(&mut self, node_id: String) {
        self.node_id = node_id;
    }

    fn handle_message(&mut self, msg: Message<Broadcast>, writer: &mut impl Write) {
        if SystemTime::now().duration_since(self.prev_time).expect("clock to not go backwards") <= Duration::from_millis(500) {
            self.gossip(writer);
        }

        let mut reply = msg.from_msg(&mut self.msg_count);
        match reply.body.msg_type {
            Broadcast::Broadcast{message} => {
                self.messages_seen.insert(message);
                reply.body.msg_type = Broadcast::BroadcastOk;
                reply.send(writer);
            },
            Broadcast::Read => {
                reply.body.msg_type = Broadcast::ReadOk { messages: self.messages_seen.clone() };
                reply.send(writer);
            },
            Broadcast::Topology{mut topology} => {
                self.topology = topology.remove(&self.node_id);
                reply.body.msg_type = Broadcast::TopologyOk;
                reply.send(writer);
            },
            Broadcast::Gossip { messages_seen } => {
                self.messages_seen.extend(messages_seen);
            },
            Broadcast::BroadcastOk | Broadcast::TopologyOk | Broadcast::ReadOk{..} => { panic!{"Unexpected message type"} }
        };
    }
}

fn main() {
    let handler = BroadcastSolution {
        node_id: String::new(),
        msg_count: 0,
        prev_time: SystemTime::now(),
        messages_seen: HashSet::new(),
        // propagated: HashMap::new(),
        topology: None,
    };
    main_loop(handler);
}

