use std::{error::Error, fmt};

use mockall::automock;

#[derive(Debug, Clone)]
pub enum MessageQueueError {
    MessageQueueSendError(String),
    MessageQueueReceiveError(String),
}

#[automock]
pub trait MessageQueue<T> {
    fn send(&self, payload: T) -> Result<(), MessageQueueError>;
    fn receive(&self) -> Result<T, MessageQueueError>;
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
