use rand::Rng;
use std::io::stdin;

mod types;
use types::{Bug, Category::*, Move};

mod bugs;
mod moves;

const LEVEL: f32 = 50.0;

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

//
//
//
//
//
//
//MAIN FUNCTION
fn main() {
    let bug_vector = bugs::create_bugs();
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
            process_effect(trainer, you, trainer_move);
            process_effect(you, trainer, you_move);
        } else {
            println!("You: {}, use {}!", you.name, you_move.name);
            trainer.hp = trainer
                .hp
                .saturating_sub(calculate_damage(you, trainer, you_move));
            println!("Trainer: {}, use {}!", trainer.name, trainer_move.name);
            you.hp = you
                .hp
                .saturating_sub(calculate_damage(trainer, you, trainer_move));
            process_effect(you, trainer, you_move);
            process_effect(trainer, you, trainer_move);
        }
    }
    println!("Battle over!");
}

//HELPER FUNCTIONS START HERE
//
//
//
//
//
//
//
//
//

fn color_check(selected_move: Move, defender: Bug) -> f32 {
    let result: f32 = COLOR_CHART[selected_move.color as usize][defender.color as usize];
    result
}

fn stab_check(attacker: Bug, selected_move: Move) -> f32 {
    let mut result = 1.0;

    if attacker.color == selected_move.color {
        result = 1.5;
    }
    result
}
//
//
//
//
//
//
fn convert_stage(bug: Bug, stat: i8) -> f32 {
    let stage: usize = bug.stages[stat as usize] as usize;
    let result: f32 = STAGE_VALUES[stage + 6];
    result
}
fn calculate_damage(attacker: Bug, defender: Bug, selected_move: Move) -> u16 {
    let mut damage = 0.0;
    let mut attack = 0.0;
    let mut defense = 0.0;
    if selected_move.stats[0] > 0 {
        let stab = stab_check(attacker, selected_move);
        let color = color_check(selected_move, defender);
        let mut rng = rand::rng();
        let n: f32 = rng.random_range(0.85..=1.0) as f32;

        if selected_move.category == Physical {
            attack = attacker.stats[1] as f32 * convert_stage(attacker, 0);
            defense = defender.stats[2] as f32 * convert_stage(defender, 1);
        } else if selected_move.category == Special {
            attack = attacker.stats[3] as f32 * convert_stage(attacker, 2);
            defense = defender.stats[4] as f32 * convert_stage(defender, 3);
        }

        damage = ((2.0 * LEVEL + 10.0) / 250.0)
            * (attack / defense)
            * selected_move.stats[0] as f32
            * stab
            * color
            * n;
    }

    println!();
    println!(
        "{} dealt {} damage to {}!",
        attacker.name, damage as u16, defender.name
    );

    damage as u16
}
//
//
//
//
//
//
fn process_effect(attacker: Bug, mut defender: Bug, selected_move: Move) {
    let index = selected_move.effects[0] as usize;

    if index > 0 {
        let strength = selected_move.effects[1];
        defender.stages[index + 1] = defender.stages[index + 1] + strength;
        println!(
            "{} changed {}'s {} by {}",
            attacker.name, defender.name, selected_move.effects[0], selected_move.effects[1]
        );
    }
}
//
//
//
//
//
//
fn print_battle(you: Bug, trainer: Bug) {
    println!();
    println!("You: {} HP: ({}/{})", you.name, you.hp, you.stats[0]);
    println!(
        "Trainer: {} HP: ({}/{})",
        trainer.name, trainer.hp, trainer.stats[0]
    );
}
//
//
//
//
//
//
fn print_move_select(you: Bug) {
    println!();
    println!("Select Move:");
    println!("1: {}", you.moves[0].name);
    println!("2: {}", you.moves[1].name);
    println!("3: {}", you.moves[2].name);
    println!("4: {}", you.moves[3].name);
}
//
//
//
//
//
//
fn trainer_move() -> u8 {
    let mut rng = rand::rng();
    let n: u8 = rng.random_range(1..=4);
    n
}
