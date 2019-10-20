use std::sync::mpsc::{Receiver, Sender};

use crate::message::Message;

pub trait Transport: Send + Sized {
    type Wire;
    fn new(num_processes: usize) -> Vec<Self>;
    fn num_senders(&self) -> usize;
    fn send(&self, message: Message);
    fn send_to_self(&self, message: Message);
    fn receive(&self) -> Message;
}

pub struct MessageChannel {
    self_sender: Sender<Message>,
    senders: Vec<Sender<Message>>,
    receiver: Receiver<Message>,
}

impl Transport for MessageChannel {
    type Wire = Message;

    fn new(num_processes: usize) -> Vec<Self> {
        let mut senders = vec![];
        let mut receivers = vec![];
        for _ in 0..num_processes {
            let (sender, receiver) = std::sync::mpsc::channel();
            senders.push(sender);
            receivers.push(receiver);
        }
        receivers
            .into_iter()
            .enumerate()
            .map(|(i, receiver)| MessageChannel {
                self_sender: senders[i].clone(),
                senders: senders.clone(),
                receiver: receiver,
            })
            .collect()
    }

    fn num_senders(&self) -> usize {
        self.senders.len()
    }

    fn send(&self, message: Message) {
        for sender in &self.senders {
            let _ = sender
                .send(Self::Wire::from(message.clone()))
                .map_err(|e| eprintln!("Failed to send {:?}", e.0));
        }
    }

    fn send_to_self(&self, message: Message) {
        let _ = self
            .self_sender
            .send(Self::Wire::from(message.clone()))
            .map_err(|e| eprintln!("Failed to send to self {:?}", e.0));
    }

    fn receive(&self) -> Message {
        self.receiver.recv().expect("recv").into()
    }
}
