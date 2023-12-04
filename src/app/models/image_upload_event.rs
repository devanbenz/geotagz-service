use uuid::Uuid;

pub struct ImageUploadEvent<'a> {
    pub id: Uuid,
    // blob storage source location
    pub source: &'a str,
    // user_id that uploaded the image
    pub user_id: Uuid
}