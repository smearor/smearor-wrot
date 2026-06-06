use crate::SmearorCompositor;
use crate::message::compositor_message::CompositorMessage;
use std::sync::mpsc::Sender;

pub trait CompositorMessageSender {
    /// Set the message sender for communicating with GTK wrapper
    fn set_message_sender(&self, sender: Sender<CompositorMessage>);

    /// Send a message to the GTK wrapper
    fn send_message(&self, message: CompositorMessage);
}

impl CompositorMessageSender for SmearorCompositor {
    fn set_message_sender(&self, sender: Sender<CompositorMessage>) {
        if let Ok(mut sender_guard) = self.message_sender.lock() {
            *sender_guard = Some(sender);
        }
    }

    fn send_message(&self, message: CompositorMessage) {
        if let Ok(sender_guard) = self.message_sender.lock() {
            if let Some(sender) = sender_guard.as_ref() {
                let _ = sender.send(message);
            }
        }
    }
}
