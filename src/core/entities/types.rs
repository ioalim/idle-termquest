use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicStat {
    pub p_att: i32,
    pub m_att: i32,
    pub p_def: i32,
    pub m_def: i32,
    pub curr_hp: u32,
    pub max_hp: u32,
    pub spd: i32,
}

impl Default for BasicStat {
    fn default() -> Self {
        Self {
            p_att: 15,
            m_att: 15,
            p_def: 15,
            m_def: 15,
            curr_hp: 15,
            max_hp: 15,
            spd: 15,
        }
    }
}

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
