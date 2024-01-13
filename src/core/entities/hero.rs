use rand::random;
use serde::{Deserialize, Serialize};

use crate::core::types::{Info, BasicStat, Status};

use super::{
    Entity, EntityType,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Hero {
    pub info: Info,
    pub stat: BasicStat,
    pub status: Status,
}

impl Hero {
    pub fn new() -> Self {
        let spd = random::<u8>();
        Self {
            info: Info {
                name: format!("{}. Hero", spd).into(),
                ..Default::default()
            },
            stat: BasicStat {
                spd: spd as i32,
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
