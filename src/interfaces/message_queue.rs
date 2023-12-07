use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum MessageQueueError {
    MessageQueueSendError(String),
    MessageQueueReceiveError(String),
}

pub trait MessageQueue<'a, T: 'a> {
    fn send<'b>(&'a self, payload: T) -> Result<(), MessageQueueError>;
    fn receive<'b>(&'a self) -> Result<T, MessageQueueError>;
}

impl fmt::Display for MessageQueueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageQueueError::MessageQueueSendError(msg) => write!(f, "Send error: {}", msg),
            MessageQueueError::MessageQueueReceiveError(msg) => write!(f, "Consume error: {}", msg),
        }
    }
}

impl Error for MessageQueueError {}
