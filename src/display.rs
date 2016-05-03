use pancurses::Window;
use model::GameState;

const LIVE: char = '#';
const DEAD: char = ' ';

pub fn show_game(window: &Window, game: &GameState) {
    for y in 0..game.height() as i32 {
        for x in 0..game.width() as i32 {
            window.mvaddch(y, x, if game.get(x, y) { LIVE } else { DEAD });
        }
    }

    window.refresh();
}
