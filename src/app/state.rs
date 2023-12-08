use super::{
    database::{blob_store::BlobStore, data_store::DataStore},
    queue::message_queue::Queue,
};

pub struct AppState<'a> {
    pub message_queue: Queue<'a>,
    pub blob_store: BlobStore,
    pub data_store: DataStore,
}
