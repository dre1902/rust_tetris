use std::convert::TryInto;
use std::io::{Write, stdout};
use std::{thread, time};
use std::fmt;
use std::process;

use rand::Rng;

use termion::event::Key;
use termion::input::TermRead;
use termion::async_stdin;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor};

const LEFT: i32 = -1;
const RIGHT: i32 = 1;

const ROWS: usize = 20 + 1;
const COLS: usize = 10 + 1;

const TICK_MOVE: i32 = 3;

#[derive(Copy, Clone, PartialEq)]
enum Blocks {
    I, O, J, L, S, T, Z, NONE
}

impl Blocks {
    fn from_u32(n: u32) -> Blocks {
        match n {
            1 => Blocks::I,
            2 => Blocks::O,
            3 => Blocks::J,
            4 => Blocks::L,
            5 => Blocks::S,
            6 => Blocks::T,
            7 => Blocks::Z,
            _ => Blocks::NONE,
        }
    }
}

// Fg2 acts as a wrapper around termion::color::Fg
struct Fg2 {
    color_id: u8,
}

impl fmt::Display for Fg2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.color_id {
            1 => write!(f, "{}", color::Fg(color::Cyan)),
            2 => write!(f, "{}", color::Fg(color::Yellow)),
            3 => write!(f, "{}", color::Fg(color::Blue)),
            4 => write!(f, "{}", color::Fg(color::LightRed)),
            5 => write!(f, "{}", color::Fg(color::Green)),
            6 => write!(f, "{}", color::Fg(color::Magenta)),
            7 => write!(f, "{}", color::Fg(color::Red)),
            _ => write!(f, "{}", color::Fg(color::Black)),
        }
    }
}

enum Rotation {
    UP, RIGHT, DOWN, LEFT
}

// Each individual block
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// Tetromino
struct Block {
    pts: [Point; 4],
    rot: Rotation,
    bl: Blocks,
}

// Board and current falling tetromino
struct Game {
    board: [[Blocks; COLS]; ROWS],
    curr: Block,
    tick: i32,
}

impl Game {
    // Moves the tetromino left or right
    fn translate(&mut self, dir: i32) {
        for pt in self.curr.pts.iter_mut() {
            pt.x += dir;
        }
        if !can_fit(&self.curr.pts, &self.board) {
            for pt in self.curr.pts.iter_mut() {
                pt.x -= dir;
            }
        }
    }

    // Rotates the tetromino left or right
    fn rotate(&mut self, dir: i32) {
        if self.curr.bl == Blocks::I {

        } else if self.curr.bl != Blocks::O {   
            match self.curr.rot {
                Rotation::UP    => (),
                Rotation::RIGHT => (),
                Rotation::DOWN  => (),
                Rotation::LEFT  => (),
            }
        }
    }

    // Causes the tetromino to fall one space
    fn fall(&mut self) -> bool {
        for pt in self.curr.pts.iter_mut() {
            pt.y += 1;
        }
        if !can_fit(&self.curr.pts, &self.board) {
            for pt in self.curr.pts.iter_mut() {
                pt.y -= 1;
            }
            self.place();
            self.new_curr();
            return true;
        }
        return false;
    }

    fn fall_fast(&mut self) {
        for i in 1..ROWS {
            if self.fall() {
                return;
            }
        }
    }

    // Add the tetromino to the board
    fn place(&mut self) {
        for pt in self.curr.pts.iter_mut() {
            self.board[pt.y as usize][pt.x as usize] = self.curr.bl;
        }
        self.clear_row();
    }

    // Clear all rows that are filled
    fn clear_row(&mut self) {
        let mut tracker = 0;
        let mut rows_cleared = 0;
        for i in 1..ROWS {
            for j in 1..COLS {
                if self.board[i][j] != Blocks::NONE {
                    tracker += 1;
                }
            }
            if tracker == COLS - 1 {
                for j in 1..COLS {
                    self.board[i][j] = Blocks::NONE;
                }
                rows_cleared += 1;
            }
            tracker = 0;
        }
        if rows_cleared > 0 {
            for i in (1..ROWS - rows_cleared).rev() {
                for j in 1..COLS {
                    if self.board[i][j] != Blocks::NONE {
                        self.board[i + rows_cleared][j] = self.board[i][j];
                        self.board[i][j] = Blocks::NONE;
                    }
                }
            }
        }
    }

    // Get a random new tetrimono
    fn new_curr(&mut self) {
        //random block
        let mut rng = rand::thread_rng();
        self.curr.bl = Blocks::from_u32(rng.gen_range(1..8));
        //self.curr.bl = Blocks::I;
        let mid: i32 = (COLS / 2).try_into().unwrap();
        match self.curr.bl {
            Blocks::I => {
                self.curr.pts[0].x = mid; self.curr.pts[0].y = 1;
                self.curr.pts[1].x = mid; self.curr.pts[1].y = 2;
                self.curr.pts[2].x = mid; self.curr.pts[2].y = 3;
                self.curr.pts[3].x = mid; self.curr.pts[3].y = 4;
            },
            Blocks::O => {
                self.curr.pts[0].x = mid; self.curr.pts[0].y = 1;
                self.curr.pts[1].x = mid + 1; self.curr.pts[1].y = 1;
                self.curr.pts[2].x = mid; self.curr.pts[2].y = 2;
                self.curr.pts[3].x = mid + 1; self.curr.pts[3].y = 2;
            },
            Blocks::J => {
                self.curr.pts[0].x = mid; self.curr.pts[0].y = 1;
                self.curr.pts[1].x = mid; self.curr.pts[1].y = 2;
                self.curr.pts[2].x = mid; self.curr.pts[2].y = 3;
                self.curr.pts[3].x = mid - 1; self.curr.pts[3].y = 3;
            },
            Blocks::L => {
                self.curr.pts[0].x = mid; self.curr.pts[0].y = 1;
                self.curr.pts[1].x = mid; self.curr.pts[1].y = 2;
                self.curr.pts[2].x = mid; self.curr.pts[2].y = 3;
                self.curr.pts[3].x = mid + 1; self.curr.pts[3].y = 3;
            },
            Blocks::S => {
                self.curr.pts[0].x = mid; self.curr.pts[0].y = 1;
                self.curr.pts[1].x = mid + 1; self.curr.pts[1].y = 1;
                self.curr.pts[2].x = mid - 1; self.curr.pts[2].y = 2;
                self.curr.pts[3].x = mid; self.curr.pts[3].y = 2;
            },
            Blocks::T => {
                self.curr.pts[0].x = mid - 1; self.curr.pts[0].y = 1;
                self.curr.pts[1].x = mid; self.curr.pts[1].y = 1;
                self.curr.pts[2].x = mid + 1; self.curr.pts[2].y = 1;
                self.curr.pts[3].x = mid; self.curr.pts[3].y = 2;
            },
            Blocks::Z => {
                self.curr.pts[0].x = mid - 1; self.curr.pts[0].y = 1;
                self.curr.pts[1].x = mid; self.curr.pts[1].y = 1;
                self.curr.pts[2].x = mid; self.curr.pts[2].y = 2;
                self.curr.pts[3].x = mid + 1; self.curr.pts[3].y = 2;
            },
            _         => {
                self.curr.pts[0].x = 1; self.curr.pts[0].y = 1;
                self.curr.pts[1].x = 1; self.curr.pts[1].y = 1;
                self.curr.pts[2].x = 1; self.curr.pts[2].y = 1;
                self.curr.pts[3].x = 1; self.curr.pts[3].y = 1;
            },
        }
    }
}

fn main() {

    let mut stdin = async_stdin();
    let mut game = Game {
        board: [[Blocks::NONE; COLS]; ROWS],
        curr: {Block {pts: [Point {x: 1, y: 1}; 4], rot: Rotation::UP, bl: Blocks::I}},
        tick: 0,
    };
    game.new_curr();

    let mut stdout = stdout().into_raw_mode().unwrap();
    loop {
        input(&mut game, &mut stdin, &mut stdout);
        update(&mut game);
        draw(&game, &mut stdout);
        thread::sleep(time::Duration::from_millis(100));
    }
}

// Check if the current tetrimono will fit in its spot
fn can_fit(pts: &[Point; 4], board: &[[Blocks; COLS]; ROWS]) -> bool {
    for pt in pts.iter() {
        if pt.y >= ROWS.try_into().unwrap() || pt.x <= 0 || pt.x >= COLS.try_into().unwrap() ||
        board[pt.y as usize][pt.x as usize] != Blocks::NONE {
            return false;
        }
    }
    return true;
}

// Block to color
fn bltoc(bl: Blocks) -> u8 {
    match bl {
        Blocks::I => 1,
        Blocks::O => 2,
        Blocks::J => 3,
        Blocks::L => 4,
        Blocks::S => 5,
        Blocks::T => 6,
        Blocks::Z => 7,
        _         => 8,
    }
}

fn draw_block<W: Write>(x: u16, y: u16, bl: Blocks, stdout: &mut W) {
    write!(stdout, "{}{}\u{2588}{}",
        cursor::Goto(x, y),
        Fg2{color_id: bltoc(bl)},
        color::Fg(color::Reset))
    .unwrap();
}

// input() takes stdout as an argument so it can drop it to exit raw mode on exit
// doesn't seem to work though
fn input<W: Write>(game: &mut Game, stdin: &mut termion::AsyncReader, stdout: &mut W) {
    let key = stdin.keys().next();
    if let Some(k) = key {
        match k.unwrap() {
            Key::Char('q') => {drop(stdout); process::exit(1)},
            Key::Char('z') => game.rotate(LEFT),
            Key::Char('c') => game.rotate(RIGHT),
            Key::Left      => game.translate(LEFT),
            Key::Right     => game.translate(RIGHT),
            Key::Down      => game.fall_fast(),
            _              => ()
        }
    }
}

fn update(game: &mut Game) {
    if game.tick < TICK_MOVE {
        game.tick += 1;
        return
    }
    game.tick = 0;
    game.fall();
}

fn draw<W: Write>(game: &Game, stdout: &mut W) {
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    for i in 1..ROWS {
        for j in 1..COLS {
            if game.board[i][j] != Blocks::NONE {
                draw_block(j.try_into().unwrap(), i.try_into().unwrap(), game.board[i][j], stdout);
            }
        }
    }
    for pt in game.curr.pts.iter() {
        draw_block(pt.x.try_into().unwrap(), pt.y.try_into().unwrap(), game.curr.bl, stdout);
    }
    stdout.flush().unwrap();
}
