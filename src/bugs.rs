use crate::moves::*;
use crate::types::Bug;
use crate::types::Color::*;

pub fn create_bugs() -> [Bug; 2] {
    let charmander = Bug {
        name: "Charmander",
        color: Fire,
        stats: [114, 72, 63, 80, 70, 85],
        stages: [0, 0, 0, 0, 0],
        moves: [GROWL, SCRATCH, EMBER, SMOKESCREEN],
        hp: 114,
    };
    let squirtle = Bug {
        name: "Squirtle",
        color: Water,
        stats: [119, 68, 85, 70, 84, 63],
        stages: [0, 0, 0, 0, 0],
        moves: [TAIL_WHIP, TACKLE, WATER_GUN, WITHDRAW],
        hp: 119,
    };

    let bug_vector = [charmander, squirtle];

    bug_vector
}
