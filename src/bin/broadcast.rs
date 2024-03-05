use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::sync::mpsc::Sender;
use std::time::Duration;

use dist_sys_challenges::{main_loop, Body, Event, Handler, Message};
use rand::Rng;
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
    GossipSend,
}

struct BroadcastSolution {
    node_id: String,
    msg_count: usize,
    messages_seen: HashSet<usize>,
    known_by_others: HashMap<String, HashSet<usize>>,
    topology: Option<Vec<String>>,
}

impl Handler<Broadcast> for BroadcastSolution {
    fn initialize(&mut self, node_id: String, sender: Sender<Event<Broadcast>>) {
        self.node_id = node_id;
        // Send gossip events every so often
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_millis(150));
                if let Err(_) = sender.send(Event::Body(Broadcast::GossipSend)) {
                    break;
                }
            }
        });
    }

    fn handle_event(&mut self, event: Event<Broadcast>, writer: &mut impl Write) {
        match event {
            Event::Message(msg) => {
                let source = msg.src.clone();
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
                        self.messages_seen.extend(messages_seen.clone());
                        // Keep track of what messages the sender knows
                        self.known_by_others
                            .entry(source)
                            .and_modify(|known| known.extend(messages_seen.clone()))
                            .or_insert(messages_seen);
                    },
                    Broadcast::BroadcastOk | Broadcast::TopologyOk | Broadcast::GossipSend | Broadcast::ReadOk{..} => { panic!{"Unexpected message type"} }
                };
            },
            Event::Body(Broadcast::GossipSend) => {
                if let Some(topology) = &self.topology {
                    for node in topology {
                        let msgs = match self.known_by_others.get(node) {
                            Some(known) => {
                                let mut diff = &self.messages_seen - &known;

                                // Add random other elements to the list
                                // to let other nodes know we know that they know
                                let mut rng = rand::thread_rng();
                                let additional_cap = (10 * diff.len() / 100) as u32;
                                diff.extend(known.iter().filter(|_| {
                                    rng.gen_ratio(
                                        additional_cap.min(known.len() as u32),
                                        known.len() as u32,
                                        )
                                }));
                                diff
                            },
                            None => self.messages_seen.clone(),
                        };
                        let msg = Message {
                            src: self.node_id.clone(),
                            dst: node.to_string(),
                            body: Body {
                                msg_id: Some(self.msg_count),
                                msg_type: Broadcast::Gossip {
                                    messages_seen: msgs,
                                },
                                in_reply_to: None,
                            }
                        };
                        msg.send(writer);
                        self.msg_count += 1;
                    }
                }
            },
            _ => panic!("Unexpected event"),
        }
    }
}

fn main() {
    let handler = BroadcastSolution {
        node_id: String::new(),
        msg_count: 0,
        messages_seen: HashSet::new(),
        known_by_others: HashMap::new(),
        topology: None,
    };
    main_loop(handler);
}

