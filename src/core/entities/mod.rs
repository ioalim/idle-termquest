use self::types::{BasicStat, Info};

pub mod enemy;
pub mod hero;
pub mod types;

pub trait Entity {
    fn info(&self) -> &Info;
    fn stat(&self) -> &BasicStat;
    fn get_type(&self) -> EntityType;
}

pub enum EntityType {
    Hero,
    Enemy,
}
