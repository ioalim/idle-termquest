use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub name: Box<str>,
    pub description: Box<str>,
    pub image_path: Box<str>,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            name: "Hero".into(),
            description: "".into(),
            image_path: "".into(),
        }
    }
}
