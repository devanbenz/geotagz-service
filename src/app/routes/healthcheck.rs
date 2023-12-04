pub async fn healthcheck() -> String {
    "Ok".into()
}

#[tokio::test]
async fn test_healthcheck() {
    assert_eq!("Ok".to_owned(), healthcheck().await)
}