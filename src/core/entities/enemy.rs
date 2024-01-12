use serde::{Deserialize, Serialize};

use super::{
    types::{BasicStat, Info},
    Entity, EntityType,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Enemy {
    info: Info,
    stat: BasicStat,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            info: Info {
                name: "Enemy".into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Entity for Enemy {
    fn info(&self) -> &Info {
        &self.info
    }

    fn stat(&self) -> &BasicStat {
        &self.stat
    }

    fn get_type(&self) -> EntityType {
        EntityType::Enemy
    }
}
