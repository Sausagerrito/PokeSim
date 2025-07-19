use rand::Rng;
use std::io::stdin;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    Normal = 0,
    Fire,
    Water,
    Grass,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Category {
    Physical = 0,
    Special,
    Status,
}

const COLOR_CHART: [[f32; 4]; 4] = {
    [
        //Normal
        [1.0, 1.0, 1.0, 1.0],
        //Fire
        [1.0, 0.5, 0.5, 2.0],
        //Water
        [1.0, 2.0, 0.5, 0.5],
        //Grass
        [1.0, 0.5, 2.0, 0.5],
    ]
};

const STAGE_VALUES: [f32; 13] = [
    0.25,
    2.0 / 7.0,
    2.0 / 6.0,
    0.4,
    0.5,
    2.0 / 3.0,
    1.0,
    1.5,
    2.0,
    2.5,
    3.0,
    3.5,
    4.0,
];

#[derive(Debug, Copy, Clone)]
struct Move {
    name: &'static str,
    color: Color,
    stats: [u16; 2],
    effects: [i8; 2],
    category: Category,
    self_target: bool,
}

#[derive(Debug, Copy, Clone)]
struct Bug {
    name: &'static str,
    color: Color,
    stats: [u16; 6],
    stages: [i8; 5],
    moves: [Move; 4],
    //not included in JSON
    hp: u16,
}

fn main() {
    let bug_vector = create_bugs();
    let mut you = bug_vector[0];
    let mut trainer = bug_vector[1];

    println!("Welcome to PokeSim!");
    while you.hp > 0 && trainer.hp > 0 {
        print_battle(you, trainer);
        print_move_select(you);

        let choice = loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<u8>() {
                Ok(num) if num >= 1 && num <= 4 => break num,
                _ => println!("Invalid Input"),
            }
        };

        let you_move = you.moves[choice as usize - 1];

        let trainer_choice = trainer_move();

        let trainer_move = trainer.moves[trainer_choice as usize - 1];

        println!();

        if trainer.stats[5] > you.stats[5] {
            println!("Trainer: {}, use {}!", trainer.name, trainer_move.name);
            you.hp = you
                .hp
                .saturating_sub(calculate_damage(trainer, you, trainer_move));
            println!("You: {}, use {}!", you.name, you_move.name);
            trainer.hp = trainer
                .hp
                .saturating_sub(calculate_damage(you, trainer, you_move));
        } else {
            println!("You: {}, use {}!", you.name, you_move.name);
            trainer.hp = trainer
                .hp
                .saturating_sub(calculate_damage(you, trainer, you_move));
            println!("Trainer: {}, use {}!", trainer.name, trainer_move.name);
            you.hp = you
                .hp
                .saturating_sub(calculate_damage(trainer, you, trainer_move));
        }
    }
    println!("Battle over!");
}

fn color_check(attacker: Bug, defender: Bug) -> f32 {
    let result: f32 = COLOR_CHART[attacker.color as usize][defender.color as usize];
    result
}

fn stab_check(attacker: Bug, selected_move: Move) -> f32 {
    let mut result = 1.0;

    if attacker.color == selected_move.color {
        result = 1.5;
    }
    result
}

fn convert_stage(bug: Bug, stat: i8) -> f32 {
    let stage: usize = bug.stages[stat as usize] as usize;
    let result: f32 = STAGE_VALUES[stage + 6];
    result
}
fn calculate_damage(attacker: Bug, defender: Bug, selected_move: Move) -> u16 {
    use Category::*;
    let mut damage = 0.0;
    let mut attack = 0.0;
    let mut defense = 0.0;
    if selected_move.stats[0] > 0 {
        let stab = stab_check(attacker, selected_move);
        let color = color_check(attacker, defender);
        let mut rng = rand::rng();
        let n: f32 = rng.random_range(85..=100) as f32;

        if selected_move.category == Physical {
            attack = attacker.stats[1] as f32 * convert_stage(attacker, 0);
            defense = defender.stats[2] as f32 * convert_stage(defender, 1);
        } else if selected_move.category == Special {
            attack = attacker.stats[3] as f32 * convert_stage(attacker, 2);
            defense = defender.stats[4] as f32 * convert_stage(defender, 3);
        }

        damage = ((22.0 * selected_move.stats[0] as f32 * attack / defense) / 50.0 + 2.0)
            * stab
            * color
            * n
            / 100.0;
    }

    println!();
    println!(
        "{} dealt {} damage to {}!",
        attacker.name, damage as u16, defender.name
    );

    damage as u16
}

fn process_effect(mut defender: Bug, selected_move: Move) {
    let index = selected_move.effects[0] as usize;

    if index > 0 {
        let strength = selected_move.effects[1];
        defender.stages[index] = defender.stages[index] + strength;
    }
}

fn create_bugs() -> [Bug; 2] {
    use Color::*;
    let charmander = Bug {
        name: "Charmander",
        color: Fire,
        stats: [39, 52, 43, 60, 50, 65],
        stages: [0, 0, 0, 0, 0],
        moves: [GROWL, SCRATCH, EMBER, SMOKESCREEN],
        hp: 39,
    };
    let squirtle = Bug {
        name: "Squirtle",
        color: Water,
        stats: [44, 48, 65, 50, 64, 43],
        stages: [0, 0, 0, 0, 0],
        moves: [TAIL_WHIP, TACKLE, WATERGUN, WITHDRAW],
        hp: 44,
    };

    let bug_vector = [charmander, squirtle];

    bug_vector
}

const GROWL: Move = Move {
    name: "Growl",
    color: Color::Normal,
    stats: [0, 100],
    effects: [1, -1],
    category: Category::Status,
    self_target: false,
};
const SCRATCH: Move = Move {
    name: "Scratch",
    color: Color::Normal,
    stats: [40, 100],
    effects: [0, 0],
    category: Category::Physical,
    self_target: false,
};
const EMBER: Move = Move {
    name: "Ember",
    color: Color::Fire,
    stats: [40, 100],
    effects: [0, 0],
    category: Category::Special,
    self_target: false,
};
const SMOKESCREEN: Move = Move {
    name: "Smokescreen",
    color: Color::Normal,
    stats: [0, 100],
    effects: [0, 0],
    category: Category::Status,
    self_target: false,
};

const TAIL_WHIP: Move = Move {
    name: "Tail Whip",
    color: Color::Normal,
    stats: [0, 100],
    effects: [2, -1],
    category: Category::Status,
    self_target: false,
};
const TACKLE: Move = Move {
    name: "Tackle",
    color: Color::Normal,
    stats: [40, 100],
    effects: [0, 0],
    category: Category::Physical,
    self_target: false,
};
const WATERGUN: Move = Move {
    name: "Watergun",
    color: Color::Water,
    stats: [40, 100],
    effects: [0, 0],
    category: Category::Special,
    self_target: false,
};
const WITHDRAW: Move = Move {
    name: "Withdraw",
    color: Color::Normal,
    stats: [0, 100],
    effects: [2, 1],
    category: Category::Status,
    self_target: true,
};

fn print_battle(you: Bug, trainer: Bug) {
    println!();
    println!("You: {} HP: ({}/{})", you.name, you.hp, you.stats[0]);
    println!(
        "Trainer: {} HP: ({}/{})",
        trainer.name, trainer.hp, trainer.stats[0]
    );
}

fn print_move_select(you: Bug) {
    println!();
    println!("Select Move:");
    println!("1: {}", you.moves[0].name);
    println!("2: {}", you.moves[1].name);
    println!("3: {}", you.moves[2].name);
    println!("4: {}", you.moves[3].name);
}

fn trainer_move() -> u8 {
    let mut rng = rand::rng();
    let n: u8 = rng.random_range(1..=4);
    n
}
