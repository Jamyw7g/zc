use std::{io::{Read, Write}, usize};
use bincode::{deserialize_from, serialize_into};
use std::path::Path;
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct DataList(HashMap<String, usize>);


#[allow(dead_code)]
impl DataList {
    pub fn new() -> Self {
        DataList(HashMap::new())
    }

    pub fn with_capacity(cap: usize) -> Self {
        DataList(HashMap::with_capacity(cap))
    }

    pub fn add(&mut self, path: &str) {
        *self.0.entry(path.into()).or_insert(0) += 10;
    }

    pub fn increase(&mut self, path: &str, weight: usize) {
        if let Some(v) = self.0.get_mut(path) {
            *v += weight;
        }
    }

    pub fn decrease(&mut self, path: &str, weight: usize) {
        if let Some(v) = self.0.get_mut(path) {
            *v -= weight;
        }
    }

    pub fn deserialize_from<R: Read>(reader: R) -> Self {
        let list = deserialize_from(reader).unwrap();
        DataList(list)
    }

    pub fn serialize_into<W: Write>(&self, writer: W) {
        serialize_into(writer, &self.0).unwrap();
    }

    pub fn sort(&self) -> Vec<(String, usize)> {
        let mut res: Vec<_> = self.0.iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();
        res.sort_by(|a, b| a.1.cmp(&b.1));
        res
    }

    pub fn to_string(&self) -> String {
        let mut str_buffer = String::new();
        for (p, w) in self.sort() {
            str_buffer.push_str(&format!("weigth={}, path={}\n", w, p));
        }
        str_buffer
    }

    pub fn clean(&mut self) -> usize {
        let keys: Vec<_> = self.0.keys().cloned().collect();
        let mut count = 0;
        for key in keys.iter() {
            if !Path::new(key.as_str()).exists() {
                self.0.remove(key);
                count += 1;
            }
        }
        count
    }
}
