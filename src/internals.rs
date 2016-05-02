// Copyright (C) 2016 Morgan Kalvin Nrykkxyyyn
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;

use pancurses;
use rand;

const LIVE: char = '#';
const DEAD: char = ' ';
//const XLEN: i32 = 100;
//const YLEN: i32 = 50;

pub fn update_map(screen: &pancurses::Window, map: HashMap<(i32, i32), bool>) -> HashMap<(i32, i32), bool> {
    let mut new_map: HashMap<(i32, i32), bool> = HashMap::new();
    let (ylen, xlen) = screen.get_max_yx();

    // Create the initial implementation, work on error handling later.
    for x in 0..xlen {
        for y in 0..ylen {
            new_map.insert((x, y), is_alive_or_dead(x, y, &map, xlen, ylen));
        }
    }

    new_map
}

pub fn create_map(screen: &pancurses::Window) -> HashMap<(i32, i32), bool> {
    let mut map: HashMap<(i32, i32), bool> = HashMap::new();
    let (ylen, xlen) = screen.get_max_yx();

    for x in 0..xlen {
        for y in 0..ylen {
            map.insert((x, y), init_state());
        }
    }

    map
}

pub fn draw_screen(screen: &pancurses::Window, map: &HashMap<(i32, i32), bool>) {
    let (ylen, xlen) = screen.get_max_yx();
    for y in 0..ylen {
        for x in 0..xlen {
            let cell: char = {
                    let tb: bool = *(map.get(&(x, y)).unwrap());
                    if tb {
                        LIVE
                    } else {
                        DEAD
                    }
            };
            screen.mvaddch(y, x, cell);
        }
    }
    screen.refresh();
}

fn is_alive_or_dead(x: i32, y: i32, map: &HashMap<(i32, i32), bool>, xlen: i32, ylen: i32) -> bool {
    let neighbors: Vec<(i32, i32)> = {
            let compars: Vec<(i32, i32)> = vec![
                (0, 1),
                (1, 0),
                (0, -1),
                (-1, 0),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ];

            let mut vector: Vec<(i32, i32)> = Vec::new();

            for (t_x, t_y) in compars {
                let mut j_x = x + t_x;
                let mut j_y = y + t_y;

                if j_x < 0 {
                    j_x = xlen-1;
                }
                if j_y < 0 {
                    j_y = ylen-1;
                }
                if j_x >= xlen {
                    j_x = 0;
                }
                if j_y >= ylen {
                    j_y = 0;
                }

                vector.push((j_x, j_y));
            }

            vector
    };

    let mut counter: i32 = 0;

    for coordinate in &neighbors {
        match map.get(&coordinate) {
            Some(b) => {
                if *b {
                    counter += 1;
                }
            },
            None => continue,
        }
    }

    let cell_status: bool = *(map.get(&(x, y)).unwrap());

    if cell_status && (counter == 2 || counter == 3) {
        true
    } else if cell_status {
        false
    } else if counter == 3 {
        true
    } else {
        false
    }
}

fn init_state() -> bool {
    rand::random::<i32>() % 7 == 0
}
