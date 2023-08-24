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
