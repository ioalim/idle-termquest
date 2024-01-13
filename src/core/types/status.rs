use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Status {
    alive: bool,
    efects: Vec<Box<dyn Effect>>,
    //stunned: bool,
    //poisoned: bool,
    //burning: bool,
    //frozen: bool,
    //bleeding: bool,
    //cursed: bool,
    //blessed: bool,
    //confused: bool,
    //blinded: bool,
    //silenced: bool,
    //enraged: bool,
    //charmed: bool,
    //petrified: bool,
    //invulnerable: bool,
    //invisible: bool,
    //flying: bool,
    //floating: bool,
    //levitating: bool,
    //ethereal: bool,
    //incorporeal: bool,
    //intangible: bool,
    //intangible_to_physical: bool,
    //intangible_to_magical: bool,
    //intangible_to_elemental: bool,
    //intangible_to_status: bool,
    //intangible_to_physical_and_magical: bool,
    //intangible_to_physical_and_elemental: bool,
    //intangible_to_physical_and_status: bool,
    //intangible_to_magical_and_elemental: bool,
    //intangible_to_magical_and_status: bool,
    //intangible_to_elemental_and_status: bool,
    //intangible_to_physical_magical_and_elemental: bool,
    //intangible_to_physical_magical_and_status: bool,
    //intangible_to_physical_elemental_and_status: bool,
    //intangible_to_magical_elemental_and_status: bool,
    //intangible_to_physical_magical_elemental_and_status: bool,
    //intangible_to_physical_and_magical_and_elemental: bool,
    //intangible_to_physical_and_magical_and_status: bool,
    //intangible_to_physical_and_elemental_and_status: bool,
    //intangible_to_magical_and_elemental_and_status: bool,
    //intangible_to_physical_magical_and_elemental_and_status: bool,
    //intangible_to_all: bool,
    //intangible_to_all_except_physical: bool,
    //intangible_to_all_except_magical: bool,
    //intangible_to_all_except_elemental: bool,
    //intangible_to_all_except_status: bool,
    //intangible_to_all_except_physical_and_magical: bool,
    //intangible_to_all_except_physical_and_elemental: bool,
    //intangible_to_all_except_physical_and_status: bool,
    //intangible_to_all_except_magical_and_elemental: bool,
    //intangible_to_all_except_magical_and_status: bool,
    //intangible_to_all_except_elemental_and_status: bool,
    //intangible_to_all_except_physical_magical_and_elemental: bool,
    //intangible_to_all_except_physical_magical_and_status: bool,
    //intangible_to_all_except_physical_elemental_and_status: bool,
    //intangible_to_all_except_magical_elemental_and_status: bool,
    //intangible_to_all_except_physical_magical_elemental_and_status: bool,
    //intangible_to_all_except_physical_and_magical_and_elemental: bool,
    //intangible_to_all_except_physical_and_magical_and_status: bool,
    //intangible_to_all_except_physical_and_elemental_and_status: bool,
    //intangible_to_all_except_magical_and_elemental_and_status: bool,
    //intangible_to_all_except_physical_magical_and_elemental_and_status: bool,
}

// it's just a concept for now, because i currently have no idea how to implement this to affect
// the battle

#[typetag::serde(tag = "type", content = "value")]
pub trait Effect: Debug {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct DragonPoison {
    pub damage: i32,
    pub duration: i32,
}

#[typetag::serde]
impl Effect for DragonPoison {
}


