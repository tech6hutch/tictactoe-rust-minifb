extern crate minifb;

use std::fmt;

use minifb::{Key, Window, WindowOptions};

type Px = u32;

/// Square size, in pixels, before scaling
const BASE_SQUARE_SIZE: usize = 8;
/// Board size, in pixels, before scaling
const BASE_BOARD_SIZE: usize = BASE_SQUARE_SIZE * 3;

const WINDOW_SCALE: minifb::Scale = minifb::Scale::X1;
const SCALE: usize = 2;
const SCALED_SQUARE_SIZE: usize = SCALE * BASE_SQUARE_SIZE;
const WINDOW_WIDTH: usize = SCALE * BASE_BOARD_SIZE;
const WINDOW_HEIGHT: usize = WINDOW_WIDTH;

fn main() {
    // let mut buffer: Vec<Px> = vec![PIXEL_BLACK; WINDOW_WIDTH * WINDOW_HEIGHT];

    // println!("Starting window {}x{}", WINDOW_WIDTH, WINDOW_HEIGHT);
    // let mut window = Window::new(
    //     "Test - Esc to exit",
    //     WINDOW_WIDTH,
    //     WINDOW_HEIGHT,
    //     WindowOptions::default(),
    // ).unwrap();

    // // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // let mut state = GameState::Start;
    // let mut board = [[None::<Piece>; 3]; 3];

    let mut game = Game::new();

    assert_eq!(SCALED_SQUARE_SIZE, WINDOW_WIDTH / 3);

    while game.window.is_open() && !game.window.is_key_down(Key::Escape) {
        match game.state {
            GameState::PlayerTurn(player_num) => {
                game.do_player_turn(player_num);
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

struct Game {
    window: minifb::Window,
    buffer: Vec<Px>,
    state: GameState,
    board: [[Option<Piece>; 3]; 3],
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
        }
    }

    fn do_player_turn(&mut self, player_num: u8) {
        assert!(player_num > 0);
        assert!(player_num <= 2);

        let (x, y) = match get_player_input(&self.window) {
            Some((x, y)) => (x, y),
            _ => return
        };
        assert!(x < 3 && y < 3, "Unknown coordinates {},{}", x, y);

        let square = &mut self.board[x][y];

        if let Some(piece) = square {
            println!("Square at {},{} already occupied by an {}", x, y, piece);
            return;
        }

        let player_piece = match player_num {
            1 => Piece::X,
            2 => Piece::O,
            _ => unreachable!()
        };
        *square = Some(player_piece);
        draw_shape(&mut self.buffer, player_piece.get_shape(), x, y);

        self.state = GameState::PlayerTurn(match player_num {
            1 => 2,
            2 => 1,
            _ => unreachable!()
        });
    }
}

fn get_player_input(window: &minifb::Window) -> Option<(usize, usize)> {
    if !window.get_mouse_down(minifb::MouseButton::Left) {
        return None;
    }

    let (x, y) = if let Some((screen_x, screen_y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
        let x = (screen_x.round() as usize) / SCALED_SQUARE_SIZE;
        let y = (screen_y.round() as usize) / SCALED_SQUARE_SIZE;
        if x >= 3 || y >= 3 {
            eprintln!("Mouse was in the window, but not in the board");
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

#[derive(Clone, Copy, Debug)]
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

type SHAPE = [[Px; BASE_SQUARE_SIZE]; BASE_SQUARE_SIZE];

const SHAPE_X: SHAPE = {
    const B: Px = PIXEL_BLACK;
    const W: Px = PIXEL_WHITE;
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
    let square_top_left = x * SCALED_SQUARE_SIZE + y * WINDOW_WIDTH;

    for (shape_row_num, shape_row) in shape.iter().enumerate() {
        let scaled_row_num = shape_row_num * SCALE;
        let (scaled_row_top_left, next_scaled_row_top_left) = {
            let n = scaled_row_num * WINDOW_WIDTH;
            (n + square_top_left, n * SCALE + square_top_left)
        };

        for (shape_col_num, &px) in shape_row.iter().enumerate() {
            let scaled_col_num = shape_col_num * SCALE;

            for row_top_left in (scaled_row_top_left..next_scaled_row_top_left).step_by(WINDOW_WIDTH) {
                let (scaled_top_left, next_scaled_top_left) = {
                    let n = row_top_left + scaled_col_num;
                    (n, n + SCALE)
                };

                for i in scaled_top_left..next_scaled_top_left {
                    buffer[i] = px;
                }
            }
        }
    }
}
