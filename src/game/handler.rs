use rand::seq::SliceRandom;

use crate::data::*;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileColor {
    Empty,
    Red,
    Green,
    Yellow,
    Cyan,
}

#[derive(Resource)]
pub struct TileStatus {
    pub character: Option<char>,
    pub color: TileColor,
}

impl Default for TileStatus {
    fn default() -> Self {
        Self {
            character: None,
            color: TileColor::Empty,
        }
    }
}

#[derive(Resource)]
pub struct GameHandler {
    pub min: usize,
    pub max: usize,
    pub y: usize,
    pub x: usize,
    pub flip: bool,
    pub line: usize,
    pub offset: usize,
    pub size: usize,
    pub tiles: Vec<Vec<TileStatus>>,
    possible: Vec<String>,
    pub ans: String,
}

impl GameHandler {
    fn calc_y(min: usize, max: usize) -> usize {
        max * 3 / 2 - min / 2 + 1
    }

    fn calc_size((width, height): (usize, usize), yl: usize, xl: usize) -> usize {
        std::cmp::min(width / xl, height / yl)
    }

    fn width_height(y: usize, _x: usize, window: &Window) -> (usize, usize, bool) {
        let flip = window.height() / y as f32 <= 60.0;
        let height;
        let width;
        if flip {
            height = window.width() as usize;
            width = window.height() as usize;
        } else {
            height = window.height() as usize;
            width = window.width() as usize;
        }
        (width, height, flip)
    }

    pub fn renew(&mut self) {
        self.tiles = Vec::new();
        for _ in 0..self.y {
            let mut row = Vec::new();
            for _ in 0..self.x {
                row.push(TileStatus::default());
            }
            self.tiles.push(row);
        }
        self.ans = (*self.possible.choose(&mut rand::thread_rng()).unwrap()).clone();
        debug_println!("For debuggers, answer is \"{}\"!", self.ans);
    }

    pub fn new(min: usize, max: usize, window: &Window, words: &Res<CommonWords>) -> Self {
        let y = Self::calc_y(min, max);
        let x = max;
        let (width, height, flip) = Self::width_height(y, x, window);
        let size = Self::calc_size((width, height), y, x);
        let mut tiles = Vec::new();
        for _ in 0..y {
            let mut row = Vec::new();
            for _ in 0..x {
                row.push(TileStatus::default());
            }
            tiles.push(row);
        }
        let possible = words.0.slice(min, max);
        let ans = (*possible.choose(&mut rand::thread_rng()).unwrap()).clone();
        debug_println!("Y length: {}, X length: {}", y, x);
        debug_println!("For debuggers, answer is \"{}\"!", ans);
        Self {
            min,
            max,
            y,
            x,
            flip,
            line: 0,
            offset: 0,
            size,
            tiles,
            possible,
            ans,
        }
    }

    pub fn colorize(&mut self) {
        let mut index: usize = 0;
        let mut list = self
            .ans
            .chars()
            .map(move |c| {
                let result = (c, index);
                index += 1;
                result
            })
            .collect::<Vec<_>>();
        let line = self.line;
        for x in 0..self.offset {
            let tile = &mut self.tiles[line][x];
            if tile.color == TileColor::Empty {
                let mut iter = list.iter();
                let mut remove = usize::MAX;
                let mut index = 0;
                while let Some((c, i)) = iter.next() {
                    if *i == x && *c == tile.character.unwrap() {
                        tile.color = TileColor::Green;
                        remove = index;
                        break;
                    }
                    index += 1;
                }
                if remove != usize::MAX {
                    list.remove(remove);
                }
            }
        }
        for x in 0..self.offset {
            let tile = &mut self.tiles[line][x];
            if tile.color == TileColor::Empty {
                let mut iter = list.iter();
                let mut remove = usize::MAX;
                let mut index = 0;
                while let Some((c, _)) = iter.next() {
                    if *c == tile.character.unwrap() {
                        tile.color = TileColor::Yellow;
                        remove = index;
                        break;
                    }
                    index += 1;
                }
                if remove != usize::MAX {
                    list.remove(remove);
                }
            }
        }
        for x in 0..self.offset {
            let tile = &mut self.tiles[line][x];
            if tile.color == TileColor::Empty {
                tile.color = TileColor::Red;
            }
        }
        for x in self.offset..self.ans.len() {
            let tile = &mut self.tiles[line][x];
            if tile.color == TileColor::Empty {
                tile.color = TileColor::Red;
            }
        }
    }

    pub fn collect_word(&self) -> String {
        let mut word = String::new();
        for x in 0..self.offset {
            if let Some(c) = self.tiles[self.line][x].character {
                word.push(c);
            } else {
                break;
            }
        }
        word
    }

    pub fn test_exists(&self, all_words: &Res<AllWords>) -> bool {
        let word = self.collect_word();
        all_words.0.exists(&word)
    }
}
