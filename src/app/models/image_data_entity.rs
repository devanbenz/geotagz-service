use uuid::Uuid;

pub struct ImageData<'a> {
    pub id: Uuid,
    pub user_id: i32,
    pub coordinates: Geography,
    // location in blob storage
    pub blob: &'a str
}

pub struct Geography {
    pub longitude: f32,
    pub latitude: f32
}