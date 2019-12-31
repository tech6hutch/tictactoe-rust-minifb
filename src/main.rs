extern crate minifb;

use std::fmt;

use minifb::{Key, Window, WindowOptions};

type Px = u32;

const SQUARE_SIZE: usize = 8;
const WINDOW_SCALE: minifb::Scale = minifb::Scale::X16;
const WINDOW_WIDTH: usize = SQUARE_SIZE * 3;
const WINDOW_HEIGHT: usize = WINDOW_WIDTH;

fn main() {
    let mut game = Game::new();

    while game.window.is_open() && !game.window.is_key_down(Key::Escape) {
        if game.mouse_is_held && !game.window.get_mouse_down(minifb::MouseButton::Left) {
            game.mouse_is_held = false;
        }

        match game.state {
            GameState::PlayerTurn(player_num) => {
                let did_turn = game.do_player_turn(player_num);

                if did_turn {
                    game.check_win();
                }
            },

            GameState::PlayerWon(winner) => {
                println!("Player {} won", winner);
                break;
            },
        }

        game.window
            .update_with_buffer(&game.buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}

fn test_window(mut window: minifb::Window) {
    const B: u32 = PIXEL_BLACK;
    const W: u32 = PIXEL_WHITE;
    window.update_with_buffer(&[
        B, B, B, B, B, B, B,
        B, W, B, W, B, W, B,
        B, W, W, W, B, W, B,
        B, W, B, W, B, W, B,
        B, B, B, B, B, B, B,
    ], 7, 5).unwrap();
    println!("HI");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }
    println!("closing");
}

struct Player {
    num: u8,
    piece: Piece,
}

struct Game {
    window: minifb::Window,
    buffer: Vec<Px>,
    state: GameState,
    board: [[Option<Piece>; 3]; 3],
    players: Vec<Player>,
    /// Used to tell when the mouse is pressed vs. held.
    mouse_is_held: bool,
}

impl Game {
    fn new() -> Self {
        println!("Starting window {}x{}", WINDOW_WIDTH, WINDOW_HEIGHT);
        let mut window = Window::new(
            "Test - Esc to exit",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions {
                scale: WINDOW_SCALE,
                ..Default::default()
            },
        ).unwrap();

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Game {
            window,
            buffer: vec![PIXEL_BLACK; WINDOW_WIDTH * WINDOW_HEIGHT],
            state: GameState::PlayerTurn(1),
            board: [[None; 3]; 3],
            players: vec![
                Player { num: 1, piece: Piece::X },
                Player { num: 2, piece: Piece::O },
            ],
            mouse_is_held: false,
        }
    }

    /// (Maybe) handles a player's turn.
    /// Returns whether the turn actually finished.
    fn do_player_turn(&mut self, player_num: u8) -> bool {
        assert!(player_num > 0);
        assert!(player_num <= 2);

        let (x, y) = match get_click_on_square(self) {
            Some((x, y)) => (x, y),
            _ => return false
        };
        assert!(x < 3 && y < 3, "Unknown coordinates {},{}", x, y);

        let square = &mut self.board[x][y];

        if let Some(piece) = square {
            println!("Square at {},{} already occupied by an {}", x, y, piece);
            return false;
        }

        let player_piece = self.players.iter().find(|p| p.num == player_num).unwrap().piece;
        *square = Some(player_piece);
        draw_shape(&mut self.buffer, player_piece.get_shape(), x, y);

        self.state = GameState::PlayerTurn(match player_num {
            1 => 2,
            2 => 1,
            _ => unreachable!()
        });

        return true;
    }

    fn check_win(&mut self) {
        let board = self.board;

        // Check horizontal wins
        for y in 0..3 {
            if let Some(piece) = board[0][y] {
                if Some(piece) == board[1][y] &&
                    board[1][y] == board[2][y] {
                    return self._piece_win(piece);
                }
            }
        }

        // Check vertical wins
        for x in 0..3 {
            if let Some(piece) = board[x][0] {
                if Some(piece) == board[x][1] &&
                    board[x][1] == board[x][2] {
                    return self._piece_win(piece);
                }
            }
        }

        // Check diagonal wins
        if let Some(piece) = board[0][0] {
            if Some(piece) == board[1][1] &&
                board[1][1] == board[2][2] {
                return self._piece_win(piece);
            }
        }
        if let Some(piece) = board[2][0] {
            if Some(piece) == board[1][1] &&
                board[1][1] == board[0][2] {
                return self._piece_win(piece);
            }
        }
    }

    fn _piece_win(&mut self, piece: Piece) {
        self.state = GameState::PlayerWon(
            self
                .players.iter()
                .find(|p| p.piece == piece).unwrap()
                .num
        );
    }
}

fn get_click_on_square(game: &mut Game) -> Option<(usize, usize)> {
    let window = &mut game.window;

    if game.mouse_is_held || !window.get_mouse_down(minifb::MouseButton::Left) {
        return None;
    }

    game.mouse_is_held = true;

    let (x, y) = if let Some((screen_x, screen_y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
        let x = (screen_x.floor() as usize) / SQUARE_SIZE;
        let y = (screen_y.floor() as usize) / SQUARE_SIZE;
        if x >= 3 || y >= 3 {
            eprintln!("Mouse was in the window, but not in the board? Pos {},{}", x, y);
            return None;
        }

        (x, y)
    } else {
        return None;
    };

    Some((x, y))
}

enum GameState {
    PlayerTurn(u8),
    PlayerWon(u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Piece {
    X,
    O,
}

impl Piece {
    fn get_shape(&self) -> SHAPE {
        match self {
            Piece::X => SHAPE_X,
            Piece::O => SHAPE_O,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const PIXEL_BLACK: Px = 0b00000000_00000000_00000000_00000000;
const PIXEL_WHITE: Px = 0b00000000_11111111_11111111_11111111;
const PIXEL_RED: Px = 0b00000000_11111111_00000000_00000000;

type SHAPE = [[Px; SQUARE_SIZE]; SQUARE_SIZE];

const SHAPE_X: SHAPE = {
    const B: Px = PIXEL_BLACK;
    const W: Px = PIXEL_RED;
    [
        [W, W, B, B, B, B, W, W],
        [W, W, W, B, B, W, W, W],
        [B, W, W, B, B, W, W, B],
        [B, B, W, W, W, W, B, B],
        [B, B, W, W, W, W, B, B],
        [B, W, W, B, B, W, W, B],
        [W, W, W, B, B, W, W, W],
        [W, W, B, B, B, B, W, W],
    ]
};

const SHAPE_O: SHAPE = {
    const B: Px = PIXEL_BLACK;
    const W: Px = PIXEL_WHITE;
    [
        [B, B, B, W, W, B, B, B],
        [B, W, W, B, B, W, W, B],
        [B, W, B, B, B, B, W, B],
        [W, B, B, B, B, B, B, W],
        [W, B, B, B, B, B, B, W],
        [B, W, B, B, B, B, W, B],
        [B, W, W, B, B, W, W, B],
        [B, B, B, W, W, B, B, B],
    ]
};

fn draw_shape(buffer: &mut Vec<Px>, shape: SHAPE, x: usize, y: usize) {
    let square_start_idx = x * SQUARE_SIZE +
        y * SQUARE_SIZE * WINDOW_WIDTH;

    for (row_num, row) in shape.iter().enumerate() {
        let row_start_idx = square_start_idx + row_num * WINDOW_WIDTH;

        for (col_num, &px) in row.iter().enumerate() {
            buffer[row_start_idx + col_num] = px;
        }
    }
}

fn all_equal<T>(s: &[T]) -> bool
    where T: PartialEq
{
    s
        .first()
        .map(|first| s.iter().skip(1).all(|item| item == first))
        .unwrap_or(false)
}
