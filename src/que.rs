use std::collections::HashMap;
use uuid;
use crate::error::InternalError;
use crate::types::QueObject;

pub struct Que {
    pub main_que: QueMap,
    pub return_que: QueMap,
}
impl Que {
    pub fn new() -> Que {
        Que {
            main_que: QueMap::new(),
            return_que: QueMap::new(),
        }
    }
}

pub struct QueMap {
    map: HashMap<String, String>
}

impl QueMap {
    pub fn new() -> QueMap {
        let main_que: HashMap<String, String> = HashMap::new();
        QueMap {
            map: main_que,
        }
    }

    pub fn add_to_map_and_generate_key(&mut self, item: String) -> String {
        let key = uuid::Uuid::new_v4().to_string();
        self.map.insert(key.clone(), item);
        key
    }

    pub fn add_to_map(&mut self, key: String, item: String) -> String {
        self.map.insert(key.clone(), item);
        key
    }

    pub fn retrieve_from_map(&mut self, key: String) -> Result<(String, String), InternalError> {
        let item_result = self.map.get(&key).cloned();
        if item_result.is_none() {
            return Err(InternalError::new_basic_error())
        }
        let item = item_result.unwrap();
        self.map.remove(&key);
        Ok((key, item))
    }

    pub fn retrieve_all_from_map(&mut self) -> Vec<QueObject> {
        let mut que_items: Vec<QueObject> = Vec::new();
        let mut key_list: Vec<String> = Vec::new();
        for key in self.map.keys() {
            key_list.push(key.clone());
        }
        for key in key_list {
            let key_value_result = self.retrieve_from_map(key);
            let mut key_value = ("".to_string(), "".to_string());
            if key_value_result.is_ok() {
                key_value = key_value_result.unwrap();
            }
            let que_object = QueObject {key: key_value.0 , message: key_value.1 };
            que_items.push(que_object);
        }
        que_items
    }
}