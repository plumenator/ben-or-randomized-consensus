use crate::message::Message;

mod byte_channel;
mod message_channel;

pub use byte_channel::ByteChannel;
pub use message_channel::MessageChannel;

pub trait Transport: Send {
    fn num_senders(&self) -> usize;
    fn send(&self, message: Message);
    fn send_to_self(&self, message: Message);
    fn receive(&self) -> Message;
}
