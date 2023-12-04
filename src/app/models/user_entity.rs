use uuid::Uuid;

pub struct User<'a> {
    pub id: Uuid,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub email: &'a str
}