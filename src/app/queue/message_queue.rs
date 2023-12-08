use std::{sync::RwLock, vec};

use crate::{
    app::models::image_upload_event::ImageUploadEvent,
    interfaces::message_queue::{MessageQueue, MessageQueueError},
};

pub struct Queue<'a> {
    data: RwLock<Vec<ImageUploadEvent<'a>>>,
}

impl<'a> Queue<'a> {
    pub fn new() -> Self {
        let data = RwLock::new(Vec::<ImageUploadEvent<'a>>::new());
        Queue { data }
    }
}

impl<'a> MessageQueue<'a, ImageUploadEvent<'a>> for Queue<'a> {
    fn send(
        &'a self,
        payload: ImageUploadEvent<'a>,
    ) -> std::prelude::v1::Result<(), crate::interfaces::message_queue::MessageQueueError> {
        self.data
            .write()
            .expect("could not unwrap arc")
            .push(payload);
        Ok(())
    }

    fn receive(
        &self,
    ) -> std::prelude::v1::Result<
        ImageUploadEvent<'a>,
        crate::interfaces::message_queue::MessageQueueError,
    > {
        match self.data.write().expect("could not unwrap data").pop() {
            Some(val) => Ok(val),
            None => Err(MessageQueueError::MessageQueueReceiveError(
                "Nothing to receive".into(),
            )),
        }
    }
}
