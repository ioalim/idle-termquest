use std::fmt::Debug;

use super::types::{Info, BasicStat};

pub mod enemy;
pub mod hero;

pub type Id = u32;

pub trait Entity: Debug {
    fn id(&self) -> Id;
    fn info(&self) -> &Info;
    fn stat(&self) -> &BasicStat;
    fn get_type(&self) -> EntityType;
}

pub enum EntityType {
    Hero,
    Enemy,
}
