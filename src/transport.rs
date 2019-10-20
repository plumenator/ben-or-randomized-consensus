use std::sync::mpsc::{Receiver, Sender};

use crate::message::Message;

pub(crate) struct Transport {
    self_sender: Sender<Message>,
    senders: Vec<Sender<Message>>,
    receiver: Receiver<Message>,
}

impl Transport {
    pub(crate) fn new(num_processes: usize) -> Vec<Self> {
        let mut senders = vec![];
        let mut receivers = vec![];
        for _ in 0..num_processes {
            let (sender, receiver) = std::sync::mpsc::channel();
            senders.push(sender);
            receivers.push(receiver);
        }
        let mut transports = vec![];
        for (i, receiver) in receivers.into_iter().enumerate() {
            transports.push(Transport {
                self_sender: senders[i].clone(),
                senders: senders.clone(),
                receiver: receiver,
            })
        }
        transports
    }

    pub(crate) fn num_senders(&self) -> usize {
        self.senders.len()
    }

    pub(crate) fn send(&self, message: Message) {
        for sender in &self.senders {
            let _ = sender
                .send(message.clone())
                .map_err(|e| eprintln!("Failed to send {:?}", e.0));
        }
    }

    pub(crate) fn send_to_self(&self, message: Message) {
        let _ = self
            .self_sender
            .send(message.clone())
            .map_err(|e| eprintln!("Failed to send to self {:?}", e.0));
    }

    pub(crate) fn receive(&self) -> Message {
        self.receiver.recv().expect("recv")
    }
}
