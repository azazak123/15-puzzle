use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::enable_raw_mode;
use std::io::stdin;

use crate::logic::{check, move_zero, random_generate};
use crate::visualisation::print;

mod logic;
mod visualisation;

fn main() {
    let mut square_size_str = String::new();
    println!("Square size:");
    stdin().read_line(&mut square_size_str).unwrap();

    let square_size = square_size_str.trim().parse::<usize>();
    let square_size: usize = if let Ok(square_size) = square_size {
        square_size
    } else {
        println!("You should input integer");
        stdin().read_line(&mut String::new()).unwrap();
        return;
    };

    let mut field = vec![vec![0; square_size]; square_size];
    field = random_generate(field, square_size);

    enable_raw_mode().unwrap();

    loop {
        print(&field);
        if check(&field) {
            println!("Victory!");
            stdin().read_line(&mut String::new()).unwrap();
            return;
        }
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => field = move_zero(field, KeyCode::Right),
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => field = move_zero(field, KeyCode::Left),
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => field = move_zero(field, KeyCode::Up),
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => field = move_zero(field, KeyCode::Down),
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => return,
            _ => (),
        }
    }
}
