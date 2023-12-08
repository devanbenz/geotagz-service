use std::env;

use aws_sdk_s3::Client;

use crate::interfaces::store::Store;

pub struct BlobStore {
    client: Client,
}

impl BlobStore {
    pub async fn new() -> Self {
        let mut config = aws_config::defaults(aws_config::BehaviorVersion::latest());
        if env::var("USE_IN_MEMORY_AWS").unwrap() == "true" {
            log::info!("using in memory AWS");
            config = config.endpoint_url(env::var("LOCALSTACK_S3_URL").unwrap());
            log::info!(
                "using localstack S3 URL: {}",
                env::var("LOCALSTACK_S3_URL").unwrap()
            )
        }
        config = config.region("us-east-1");

        let config = config.load().await;
        let client = aws_sdk_s3::Client::new(&config);
        BlobStore { client }
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
}

impl<'a> Store<u8, &'a str, &'a str, Vec<u8>> for BlobStore {
    fn insert(
        &self,
        data: u8,
    ) -> std::prelude::v1::Result<&'a str, crate::interfaces::store::StoreError> {
        todo!()
    }

    fn find(
        &self,
        id: &str,
    ) -> std::prelude::v1::Result<Vec<u8>, crate::interfaces::store::StoreError> {
        todo!()
    }
}
