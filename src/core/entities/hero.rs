use serde::{Deserialize, Serialize};

use super::{
    types::{BasicStat, Info},
    Entity, EntityType,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Hero {
    pub info: Info,
    pub stat: BasicStat,
}

impl Hero {
    pub fn new() -> Self {
        Self {
            info: Info {
                name: "Hero".into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Entity for Hero {
    fn info(&self) -> &Info {
        &self.info
    }

    fn stat(&self) -> &BasicStat {
        &self.stat
    }

    fn get_type(&self) -> EntityType {
        EntityType::Hero
    }
}
