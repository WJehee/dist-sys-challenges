use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::sync::mpsc::Sender;

use dist_sys_challenges::{main_loop, Event, Handler};
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
    messages_seen: HashSet<usize>,
    // propagated: HashMap<usize, Vec<String>>,
    topology: Option<Vec<String>>,
}

impl BroadcastSolution {
    fn gossip(&self, writer: &mut impl Write) {
        if let Some(topology) = &self.topology {
            for node in topology {
                
            }
        }
    }
}

impl Handler<Broadcast> for BroadcastSolution {
    fn initialize(&mut self, node_id: String, sender: Sender<Event<Broadcast>>) {
        self.node_id = node_id;
        // TODO: send gossip events every so often
    }

    fn handle_event(&mut self, event: Event<Broadcast>, writer: &mut impl Write) {
        match event {
            Event::Message(msg) => {
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
            },
            Event::Body(body) => {

            }
        }

    }
}

fn main() {
    let handler = BroadcastSolution {
        node_id: String::new(),
        msg_count: 0,
        messages_seen: HashSet::new(),
        // propagated: HashMap::new(),
        topology: None,
    };
    main_loop(handler);
}

