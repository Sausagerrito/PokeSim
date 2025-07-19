use crate::types::Category::*;
use crate::types::Color::*;
use crate::types::Move;

pub const GROWL: Move = Move {
    name: "Growl",
    color: Normal,
    stats: [0, 100],
    effects: [1, -1],
    category: Status,
    self_target: false,
};
pub const SCRATCH: Move = Move {
    name: "Scratch",
    color: Normal,
    stats: [40, 100],
    effects: [0, 0],
    category: Physical,
    self_target: false,
};
pub const EMBER: Move = Move {
    name: "Ember",
    color: Fire,
    stats: [40, 100],
    effects: [0, 0],
    category: Special,
    self_target: false,
};
pub const SMOKESCREEN: Move = Move {
    name: "Smokescreen",
    color: Normal,
    stats: [0, 100],
    effects: [0, 0],
    category: Status,
    self_target: false,
};

pub const TAIL_WHIP: Move = Move {
    name: "Tail Whip",
    color: Normal,
    stats: [0, 100],
    effects: [2, -1],
    category: Status,
    self_target: false,
};
pub const TACKLE: Move = Move {
    name: "Tackle",
    color: Normal,
    stats: [40, 100],
    effects: [0, 0],
    category: Physical,
    self_target: false,
};
pub const WATER_GUN: Move = Move {
    name: "Water Gun",
    color: Water,
    stats: [40, 100],
    effects: [0, 0],
    category: Special,
    self_target: false,
};
pub const WITHDRAW: Move = Move {
    name: "Withdraw",
    color: Normal,
    stats: [0, 100],
    effects: [2, 1],
    category: Status,
    self_target: true,
};
