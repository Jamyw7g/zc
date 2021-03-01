use std::{cmp::Ordering, io::{Read, Write}};
use std::path::Path;
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct DataList(HashMap<String, f32>);


#[allow(dead_code)]
impl DataList {
    pub fn new() -> Self {
        DataList(HashMap::new())
    }

    pub fn with_capacity(cap: usize) -> Self {
        DataList(HashMap::with_capacity(cap))
    }

    pub fn add(&mut self, path: &str) {
        if self.0.contains_key(path) {
            self.increase(path, 10.0);
        } else {
            self.0.insert(String::from(path), 10.0);
        }
    }

    pub fn increase(&mut self, path: &str, weight: f32) {
        if let Some(v) = self.0.get_mut(path) {
            let val = *v;
            *v = f32::sqrt(val.powi(2) + weight.powi(2));
        }
    }

    pub fn decrease(&mut self, path: &str, weight: f32) {
        if let Some(v) = self.0.get_mut(path) {
            let val = *v - weight;
            *v = val.max(0.0);
        }
    }

    pub fn deserialize_from<R: Read>(mut reader: R) -> Self {
        let list = deserialize_from(&mut reader).unwrap();
        DataList(list)
    }

    pub fn serialize_into<W: Write>(&self, mut writer: W) {
        serilized_into(&mut writer, self).unwrap();
    }

    pub fn sort(&self) -> Vec<(String, f32)> {
        let mut res: Vec<_> = self.0.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        res.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Less));
        res
    }

    pub fn to_string(&self) -> String {
        let mut str_buffer = String::new();
        for (p, w) in self.sort() {
            str_buffer.push_str(&format!("weigth={:.02}, path={}\n", w, p));
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

fn serilized_into<W: Write>(writer: &mut W, data: &DataList) -> std::io::Result<()> {
    let mut str_buffer = String::new();
    for (p, w) in data.sort().drain(..) {
        str_buffer.push_str(&format!("{:.02},{}\n", w, p));
    }

    writer.write_all(str_buffer.as_bytes())
}

fn deserialize_from<R: Read>(reader: &mut R) -> std::io::Result<HashMap<String, f32>> {
    let mut map = HashMap::new();

    let mut str_buffer = String::new();
    reader.read_to_string(&mut str_buffer)?;
    for line in str_buffer.trim().split('\n') {
        let fields: Box<[_]> = line.split(',').collect();
        let path = String::from(fields[1]);
        let weight = fields[0].parse().unwrap();
        map.insert(path, weight);
    }

    Ok(map)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Seek, SeekFrom};

    #[test]
    fn test_se_de() {
        let mut map = HashMap::new();
        map.insert("hello".to_string(), 89.9);
        map.insert("huhus".to_string(), 90.98);
        map.insert("jksdj".to_string(), 78.989);

        let dat = DataList(map);

        let mut cur = Cursor::new(Vec::new());
        let mut str_buffer = String::new();
        serilized_into(&mut cur, &dat).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        cur.read_to_string(&mut str_buffer).unwrap();
        println!("{}", str_buffer);
        cur.seek(SeekFrom::Start(0)).unwrap();
        let res = deserialize_from(&mut cur).unwrap();
        println!("{:?}", res);
    }
}