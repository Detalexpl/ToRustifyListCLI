use std::collections::HashMap;
use trl_json::JsonValue::{self, Object};

use crate::tdv::TdvErr::BadJson;
trait Id {
    fn get_id(&self) -> usize;
    fn next_id<G: Id>(list: &[G]) -> usize {
        let used: std::collections::HashSet<usize> = list.iter().map(Id::get_id).collect();
        (0..).find(|id| !used.contains(id)).unwrap()
    }
}
struct ToDoValue {
    name: String,
    id: usize,
    sub: Option<Vec<Sub>>,
    done: bool,
}
impl ToDoValue {
    fn from_json(json: JsonValue) -> Result<Vec<ToDoValue>, TdvErr> {
        let Object(hash) = json else {
            return Err(TdvErr::BadJson);
        };

        let mut used_id = std::collections::HashSet::new();
        for val in hash.values() {
            if let Object(v) = val {
                if let Some(&JsonValue::Number(idf)) = v.get("id") {
                    if !used_id.insert(idf as usize) {
                        return Err(TdvErr::CorruptedIds); // to ids are the same
                    }
                }
            }
        }

        let mut next_free = 0usize;
        let mut vec = Vec::new();
        for (name, val) in hash {
            let Object(mut val) = val else {
                return Err(TdvErr::BadJson);
            };

            let id = match val.get("id") {
                Some(&JsonValue::Number(idf)) => idf as usize,
                _ => {
                    while used_id.contains(&next_free) {
                        next_free += 1
                    }
                    next_free
                }
            };

            let sub = match val.remove("sub") {
                None | Some(JsonValue::Null) => None,
                Some(Object(subr)) => Some(Sub::from_hash_map(subr)?),
                _ => return Err(TdvErr::BadJson),
            };

            let done = match val.get("done") {
                Some(&JsonValue::Boolean(b)) => b,
                _ => return Err(TdvErr::BadJson),
            };

            vec.push(ToDoValue {
                name,
                done,
                id,
                sub,
            });
        }
        return Ok(vec);
    }

    //function to cheange TODoValue to JsonValue
    fn to_json(vec: Vec<ToDoValue>) -> JsonValue {
        let mut mmap = HashMap::new();
        for val in vec {
            let jsub = Sub::to_json(val.sub);
            let jid = JsonValue::Number(val.id as f64);
            let jdone = JsonValue::Boolean(val.done);
            let map = HashMap::from([
                ("id".to_owned(), jid),
                ("sub".to_owned(), jsub),
                ("done".to_owned(), jdone),
            ]);
            mmap.insert(val.name, JsonValue::Object(map));
        }
        JsonValue::Object(mmap)
    }
    fn add(vec: &Vec<ToDoValue>, name: String) -> ToDoValue {
        let id = Id::next_id(vec);
    }
}

impl Id for ToDoValue {
    fn get_id(&self) -> usize {
        self.id
    }
}
struct Sub {
    name: String,
    done: bool,
    id: usize,
}
impl Id for Sub {
    fn get_id(&self) -> usize {
        self.id
    }
}
impl Sub {
    fn from_hash_map(map: HashMap<String, JsonValue>) -> Result<Vec<Sub>, TdvErr> {
        let mut vec = Vec::new();
        let mut used_ids = std::collections::HashSet::new();
        for val in map.values() {
            if let JsonValue::Object(o) = val {
                if let Some(&JsonValue::Number(f)) = o.get("id") {
                    if !used_ids.insert(f as usize) {
                        return Err(TdvErr::CorruptedIds);
                    }
                }
            }
        }

        for (name, val) in map {
            let Object(o) = val else { return Err(BadJson) };

            let done = match o.get("done") {
                Some(&JsonValue::Boolean(b)) => b,
                _ => return Err(BadJson),
            };

            let mut next_free = 0usize;
            let id = match o.get("id") {
                Some(&JsonValue::Number(idf)) => idf as usize,
                _ => {
                    while used_ids.contains(&next_free) {
                        next_free += 1;
                    }
                    next_free
                }
            };
            vec.push(Sub { name, done, id });
        }
        Ok(vec)
    }
    fn to_json(val: Option<Vec<Sub>>) -> JsonValue {
        let Some(s) = val else { return JsonValue::Null };
        let mut map = HashMap::new();
        for sub in s {
            let jid = JsonValue::Number(sub.id as f64);
            let jname = JsonValue::String(sub.name);
            let jdone = JsonValue::Boolean(sub.done);
            map.insert("name".to_owned(), jname);
            map.insert("id".to_owned(), jid);
            map.insert("jdone".to_owned(), jdone);
        }
        JsonValue::Object(map)
    }
}
#[derive(Debug)]
pub enum TdvErr {
    BadJson,
    CorruptedIds,
}
