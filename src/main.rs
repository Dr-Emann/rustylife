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

extern crate pancurses;

use std::collections::HashMap;
use std::time::Duration;
use std::thread::sleep;

use pancurses::*;

// Character representing a cell that is "on" or "alive".
const LIVE: char = '#';
// Character representing a cell that is "off" or "dead".
const DEAD: char = '.';

fn main() {
    let map = HashMap::new();
    let screen = initscr();
    
    noecho();
    curs_set(0);
    start_color();
    init_pair(1, COLOR_GREEN, COLOR_BLACK);

    let (screen_y, screen_x) = screen.get_max_yx();
    screen.attr_on(COLOR_PAIR(1));

    loop {
        for y in 0..screen_y {
            for x in 0..screen_x {
                // ...
            }
        }
        sleep(Duration::new(1, 0));
    }

    endwin();
}
