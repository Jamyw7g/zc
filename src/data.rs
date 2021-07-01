use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::slice::Iter;
use std::string::ToString;

use bincode::{deserialize_from, serialize_into, Result as BResult};

#[derive(Debug, Clone)]
pub struct Data {
    list: HashMap<String, u64>,
}

pub struct Sorted<'a> {
    inner: Vec<(&'a String, &'a u64)>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    pub fn deserialize_from<R: Read>(reader: R) -> BResult<Self> {
        let list = deserialize_from(reader)?;
        Ok(Self { list })
    }

    pub fn increase(&mut self, path: String) {
        let entry = self.list.entry(path).or_insert(0);
        *entry = (entry.saturating_pow(2).saturating_add(10_000) as f32).sqrt() as u64;
    }

    pub fn decrease(&mut self, path: String) {
        let entry = self.list.entry(path).or_insert(0);
        *entry = entry.saturating_sub(100);
    }

    pub fn sorted(&self) -> Sorted {
        let mut sorted_vec: Vec<_> = self.list.iter().collect();
        sorted_vec.sort_by(|a, b| b.1.cmp(a.1));
        Sorted { inner: sorted_vec }
    }

    pub fn purge(&mut self) -> usize {
        let mut removed = Vec::new();
        for (p, w) in &self.list {
            let pb = PathBuf::from(p);
            if !pb.exists() || *w == 0 {
                removed.push(p.clone());
            }
        }
        let num = removed.len();
        for k in removed {
            self.list.remove(&k);
        }
        num
    }

    pub fn serialize_into<W: Write>(&self, writer: W) -> BResult<()> {
        serialize_into(writer, &self.list)
    }
}

impl<'a> Sorted<'a> {
    pub fn iter(&self) -> Iter<(&String, &u64)> {
        self.inner.iter()
    }
}

impl ToString for Data {
    fn to_string(&self) -> String {
        let mut sb = String::new();
        for (p, w) in self.sorted().iter().rev() {
            sb.push_str(&format!("weight={},\tpath={}\n", w, p));
        }
        sb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Seek, SeekFrom};

    #[test]
    fn test_data() {
        let mut data = Data::new();
        data.increase("o23ne".to_string());
        data.increase("t34wo".to_string());
        data.increase("t34wo".to_string());

        let mut cur = Cursor::new(Vec::new());
        data.serialize_into(&mut cur).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        let read_data = Data::deserialize_from(&mut cur).unwrap();
        assert_eq!(read_data.to_string(), data.to_string());

        assert_eq!(data.purge(), 2);
    }
}
