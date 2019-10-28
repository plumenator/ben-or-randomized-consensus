use std::sync::mpsc::{Receiver, Sender};

use crate::{message::Message, transport::Transport};

pub struct MessageChannel {
    self_sender: Sender<Message>,
    senders: Vec<Sender<Message>>,
    receiver: Receiver<Message>,
}

impl MessageChannel {
    pub fn new(num_processes: usize) -> Vec<Box<dyn Transport>> {
        let mut senders = vec![];
        let mut receivers = vec![];
        for _ in 0..num_processes {
            let (sender, receiver) = std::sync::mpsc::channel();
            senders.push(sender);
            receivers.push(receiver);
        }
        let mut boxes = vec![];
        for receiver in receivers
            .into_iter()
            .enumerate()
            .map(|(i, receiver)| MessageChannel {
                self_sender: senders[i].clone(),
                senders: senders.clone(),
                receiver,
            })
        {
            let b: Box<dyn Transport> = Box::new(receiver);
            boxes.push(b);
        }
        boxes
    }
}

impl Transport for MessageChannel {
    fn num_senders(&self) -> usize {
        self.senders.len()
    }

    fn send(&self, message: Message) {
        for sender in &self.senders {
            let _ = sender
                .send(Message::from(message.clone()))
                .map_err(|e| eprintln!("Failed to send {:?}", e.0));
        }
    }

    fn send_to_self(&self, message: Message) {
        let _ = self
            .self_sender
            .send(Message::from(message.clone()))
            .map_err(|e| eprintln!("Failed to send to self {:?}", e.0));
    }

    fn receive(&self) -> Message {
        self.receiver.recv().expect("recv").into()
    }
}
