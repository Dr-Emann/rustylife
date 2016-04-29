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

use std::time::Duration;
use std::thread::sleep;

use pancurses;

use mapops;

// Character representing a cell that is "on" or "alive".
const LIVE: char = '#';
// Character representing a cell that is "off" or "dead".
const DEAD: char = '.';

fn main() {
    let screen = pancurses::initscr();
    let mut hash_map = mapops::create_map();

    pancurses::noecho();
    pancurses::startcolor();
    pancurses::init_pair(1, pancurses::COLOR_RED, pancurses::COLOR_BLACK);

    loop {
    
    }
}
