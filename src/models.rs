use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    X,
    O,
    N,
}

impl Cell {
   pub fn print(&self) -> &str {
        match self {
            Self::X => return "X",
            Self::O => return "O",
            Self::N => return " ",
        }
    }
}

pub enum Point {
    I,
    Ii,
    Iii,
}

impl Point {
    fn value(&self) -> usize {
        match self {
            Self::I => 0,
            Self::Ii => 1,
            Self::Iii => 2,
        }
    }
}

pub struct Place {
    pub row: Point,
    pub collum: Point,
}

pub struct Board{
    pub board_state: [[Cell; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Self {board_state: [[Cell::N; 3]; 3]}
    }

    pub fn play_move(&mut self, place: Place, player: &Cell) {
        self[&place] = player.clone();
    }

    pub fn print(&self) -> String {
        let mut result = String::new();
        result += "___________\n";
        for i in self.board_state.iter() {
            result += "|";
            for j in i.iter() {
                result += &("_".to_owned() + j.print() + "_");
            }
            result += "|\n";
        }

        result
    }
}

impl Index<&Place> for Board {
    type Output = Cell;

    fn index(&self, index: &Place) -> &Self::Output {
        &self.board_state[index.row.value()][index.collum.value()]
    } 
}

impl IndexMut<&Place> for Board {
    fn index_mut(&mut self, index: &Place) -> &mut Cell {
        &mut self.board_state[index.row.value()][index.collum.value()]
    } 
}