use anyhow::Result;
use uuid::Uuid;

use crate::{
    app::models::image_upload_event::ImageUploadEvent,
    interfaces::{message_queue::MessageQueue, store::Store},
};

pub async fn create_image<'a>(
    queue: &'a dyn MessageQueue<'a, ImageUploadEvent<'a>>,
    blob_store: &dyn Store<u8, &str, Vec<u8>>,
    user_id: Uuid,
    data: u8,
) -> Result<()> {
    let source = blob_store.insert(data)?;

    let payload = ImageUploadEvent {
        id: Uuid::new_v4(),
        source,
        user_id,
    };

    queue.send(payload)?;
    Ok(())
}

mod test {
    use std::sync::RwLock;

    use crate::interfaces::message_queue::MessageQueueError;

    use super::*;

    struct MockBlockStore {
        data: RwLock<Vec<u8>>,
    }

    #[derive(Debug)]
    struct MockMessageQueue<'a> {
        data: RwLock<Vec<ImageUploadEvent<'a>>>,
    }

    impl Store<u8, &str, Vec<u8>> for MockBlockStore {
        fn insert<'a>(
            &self,
            data: u8,
        ) -> std::prelude::v1::Result<&'a str, crate::interfaces::store::StoreError> {
            self.data.write().unwrap().push(data);
            Ok("mock")
        }

        fn find(
            &self,
            id: &str,
        ) -> std::prelude::v1::Result<Vec<u8>, crate::interfaces::store::StoreError> {
            println!("{:?}", id);
            Ok(vec![])
        }
    }

    impl<'a> MessageQueue<'a, ImageUploadEvent<'a>> for MockMessageQueue<'a> {
        fn send(
            &'a self,
            payload: ImageUploadEvent<'a>,
        ) -> std::prelude::v1::Result<(), crate::interfaces::message_queue::MessageQueueError>
        {
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

    #[tokio::test]
    async fn test_create_image() -> Result<()> {
        let mock_queue = MockMessageQueue {
            data: RwLock::new(vec![]),
        };
        let mock_blob_store = MockBlockStore {
            data: RwLock::new(vec![]),
        };
        let mock_user_id = Uuid::new_v4();
        let mock_data: u8 = 10;

        create_image(&mock_queue, &mock_blob_store, mock_user_id, mock_data).await?;
        assert_eq!(
            mock_user_id,
            mock_queue.data.read().unwrap().first().unwrap().user_id
        );
        assert_eq!(
            "mock",
            mock_queue.data.read().unwrap().first().unwrap().source
        );

        Ok(())
    }
}
