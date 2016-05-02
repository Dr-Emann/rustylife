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

use pancurses::Input;

mod model;
mod display;

macro_rules! defer {
    ($e:expr) => {
        struct Guard;
        impl Drop for Guard {
            fn drop(&mut self) {
                $e;
            }
        }
        let _guard = Guard;
    }
}

fn main() {
    let screen = pancurses::initscr();
    defer!(pancurses::endwin());

    pancurses::noecho();
    pancurses::cbreak();
    pancurses::start_color();
    pancurses::init_pair(1, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
    pancurses::curs_set(0);
    
    screen.keypad(true);
    screen.nodelay(true);
    screen.attron(pancurses::COLOR_PAIR(1));
    let mut rng = rand::thread_rng();

    let (y, x) = screen.get_max_yx();
    let mut game = model::GameState::new(x as u32, y as u32);
    game.randomize(&mut rng);

    'outer: loop {
        // TODO: Allow user to exit with 'q', and do other keyboard stuff.
        while let Some(ch) = screen.getch() {
            match ch {
                Input::KeyResize => {
                    pancurses::resize_term(0, 0);
                    let (y, x) = screen.get_max_yx();
                    game.resize(x as u32, y as u32);
                }
                Input::Character('q') => {
                    break 'outer;
                }
                _ => {}
            }
        }
        display::show_game(&screen, &game);
        game.update();
        sleep(Duration::new(0, 250_000_000));
    }
}
