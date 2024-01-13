use rand::random;
use serde::{Deserialize, Serialize};

use crate::core::types::{Info, BasicStat};

use super::{
    Entity, EntityType,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Enemy {
    info: Info,
    stat: BasicStat,
}

impl Enemy {
    pub fn new() -> Self {
        let spd = random::<u8>();
        Self {
            info: Info {
                name: format!("{}. Enemy", spd).into(),
                ..Default::default()
            },
            stat: BasicStat {
                spd: spd as i32,
                ..Default::default()
            },
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
