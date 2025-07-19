pub mod block_mesh;
pub mod colors;
pub mod spiral_iterator;

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::CHUNK_SIZE;

/// https://github.com/feather-rs/feather
/// feather/utils/src/lib.rs
pub fn vec_remove_item<T: PartialEq>(vec: &mut Vec<T>, item: &T) -> bool {
    let index = vec.iter().position(|x| x == item);
    if let Some(index) = index {
        vec.swap_remove(index);
        return true;
    }
    return false;
}

pub fn fix_chunk_loc_pos(p: i64) -> i64 {
    if p < 0 {
        return (p + 1_i64) / CHUNK_SIZE as i64 + -1_i64;
    }
    return p / CHUNK_SIZE as i64;
}

pub fn calculate_hash<T: Hash>(obj: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}


// Split "test://test/file.glb" into ("test", "test/file.glb")
pub fn split_resource_path(path: &String) -> Option<(String, String)> {
    let s: Vec<&str> = path.split("://").collect();
    if s.len() < 2 {
        return None;
    }
    let res_path = s[1..s.len()].join("/");
    return Some((s.get(0).unwrap().to_string(), res_path));
}

pub fn validate_username(username: &String) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9_]{2,16}$").unwrap();
    re.is_match(username)
}

pub fn uppercase_first(s: &String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn string_remove_range(s: &str, start: usize, stop: usize) -> String {
    let mut rslt = "".to_string();
    for (i, c) in s.chars().enumerate() {
        if start > i || stop < i + 1 {
            rslt.push(c);
        }
    }
    rslt
}
