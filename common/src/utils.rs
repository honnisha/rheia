/// https://github.com/feather-rs/feather
/// feather/utils/src/lib.rs
pub fn vec_remove_item<T: PartialEq>(vec: &mut Vec<T>, item: &T) {
    let index = vec.iter().position(|x| x == item);
    if let Some(index) = index {
        vec.swap_remove(index);
    }
}
