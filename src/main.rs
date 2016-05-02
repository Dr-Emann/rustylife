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

// TODO: Add argument parsing with argparse.
extern crate pancurses;
extern crate rand;

use std::time::Duration;
use std::thread::sleep;

mod internals;

fn main() {
    let screen = pancurses::initscr();
    let mut hash_map = internals::create_map(&screen);

    pancurses::noecho();
    pancurses::start_color();
    pancurses::init_pair(1, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
    pancurses::curs_set(0);
    screen.attron(pancurses::COLOR_PAIR(1));

    loop {
        // TODO: Allow user to exit with 'q', and do other keyboard stuff.
        internals::draw_screen(&screen, &hash_map);
        hash_map = internals::update_map(&screen, hash_map);
        sleep(Duration::new(0, 250_000_000));
    }

    pancurses::endwin();
}
