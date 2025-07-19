#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Normal = 0,
    Fire,
    Water,
    Grass,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Category {
    Physical = 0,
    Special,
    Status,
}

#[derive(Debug, Copy, Clone)]
pub struct Bug {
    pub name: &'static str,
    pub color: Color,
    pub stats: [u16; 6],
    pub stages: [i8; 5],
    pub moves: [Move; 4],
    //not included in JSON
    pub hp: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub name: &'static str,
    pub color: Color,
    pub stats: [u16; 2],
    pub effects: [i8; 2],
    pub category: Category,
    pub self_target: bool,
}
