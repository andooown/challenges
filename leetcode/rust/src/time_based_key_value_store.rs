use std::collections::HashMap;

struct Entry(i32, String);

struct TimeMap {
    map: HashMap<String, Vec<Entry>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map
            .entry(key)
            .or_insert(vec![])
            .push(Entry(timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.map.get(&key) {
            None => "".to_string(),
            Some(entries) => match entries.binary_search_by(|e| e.0.cmp(&timestamp)) {
                Ok(i) => entries[i].1.to_string(),
                Err(i) => {
                    if i > 0 {
                        entries[i - 1].1.to_string()
                    } else {
                        "".to_string()
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut obj = TimeMap::new();
        obj.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(obj.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(obj.get("foo".to_string(), 2), "bar".to_string());
        obj.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(obj.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(obj.get("foo".to_string(), 5), "bar2".to_string());
    }
}
