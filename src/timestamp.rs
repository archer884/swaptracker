use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Timestamp<T> {
    pub time: DateTime<Utc>,
    pub item: T,
}

impl<T> Timestamp<T> {
    pub fn new(item: T) -> Self {
        Self {
            time: Utc::now(),
            item,
        }
    }
}

impl<T> AsRef<T> for Timestamp<T> {
    fn as_ref(&self) -> &T {
        &self.item
    }
}
