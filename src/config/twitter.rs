use serde::Deserialize;

#[derive(Clone, Copy)]
pub struct TwitterConfig {
    pub token: &'static str,
    pub key: &'static str,
    pub secret: &'static str,
}

#[derive(Deserialize)]
pub struct OwnedTwitterConfig {
    pub token: String,
    pub key: String,
    pub secret: String,
}

impl OwnedTwitterConfig {
    pub fn into_static(self) -> TwitterConfig {
        TwitterConfig {
            token: Box::leak(Box::new(self.token)),
            key: Box::leak(Box::new(self.key)),
            secret: Box::leak(Box::new(self.secret)),
        }
    }
}
