use std::sync::RwLock;

use crate::interfaces::store::Store;

pub struct DataStore {
    data: RwLock<Vec<u8>>,
}

impl DataStore {
    pub fn new() -> Self {
        let data = RwLock::new(vec![]);
        DataStore { data }
    }
}

impl<'a> Store<u8, &'a str, &'a str, Vec<u8>> for DataStore {
    fn insert(
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
