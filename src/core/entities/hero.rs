use rand::random;
use serde::{Deserialize, Serialize};

use crate::core::types::{Info, BasicStat, Status};

use super::{
    Entity, EntityType, Id,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Hero {
    pub id: Id,
    pub info: Info,
    pub stat: BasicStat,
    pub status: Status,
}

impl Hero {
    pub fn new() -> Self {
        let id = random::<u8>();
        Self {
            id: id as u32,
            info: Info {
                name: format!("{}. Hero", id).into(),
                ..Default::default()
            },
            stat: BasicStat {
                spd: id as i32,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Entity for Hero {
    fn id(&self) -> Id {
        self.id
    }

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
