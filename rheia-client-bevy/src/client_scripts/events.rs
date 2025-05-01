trait CancellableEvent {
    fn get_cancel(&self) -> bool;
}

#[derive(Debug, serde::Serialize)]
pub struct EmptyEvent {
}
