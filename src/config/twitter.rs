use serde::Deserialize;

#[derive(Clone, Copy)]
pub struct TwitterConfig {
    pub key: &'static str,
    pub secret: &'static str,
}

#[derive(Deserialize)]
pub struct OwnedTwitterConfig {
    pub key: String,
    pub secret: String,
}

impl OwnedTwitterConfig {
    pub fn into_static(self) -> TwitterConfig {
        TwitterConfig {
            key: Box::leak(Box::new(self.key)),
            secret: Box::leak(Box::new(self.secret)),
        }
    }
}
