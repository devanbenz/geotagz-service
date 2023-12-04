use anyhow::Result;
use uuid::Uuid;

use crate::{
    app::models::image_upload_event::ImageUploadEvent,
    interfaces::{
        message_queue::{MessageQueue, MockMessageQueue},
        store::{MockStore, Store},
    },
};

pub async fn create_image<'a>(
    queue: Box<dyn MessageQueue<ImageUploadEvent<'a>>>,
    blob_store: Box<dyn Store<Vec<u8>, &str, Vec<u8>>>,
    user_id: Uuid,
    data: Vec<u8>,
) -> Result<()> {
    blob_store.insert(data)?;

    let payload = ImageUploadEvent {
        id: Uuid::new_v4(),
        source: "",
        user_id,
    };

    queue.send(payload)?;
    Ok(())
}

#[tokio::test]
async fn test_create_image() -> Result<()> {
    let mock_user_id = Uuid::new_v4();
    let mut mock_message_queue = MockMessageQueue::<ImageUploadEvent<'static>>::new();
    let mut mock_blob_store = MockStore::<Vec<u8>, &str, Vec<u8>>::new();

    mock_message_queue
        .expect_send()
        .times(1)
        .return_const(Ok(()));

    mock_blob_store
        .expect_insert()
        .times(1)
        .return_const(Ok(()));

    create_image(
        Box::new(mock_message_queue),
        Box::new(mock_blob_store),
        mock_user_id,
        Vec::new().into(),
    )
    .await?;
    Ok(())
}
