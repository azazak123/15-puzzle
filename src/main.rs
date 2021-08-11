use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::enable_raw_mode;
use rand::{thread_rng, Rng};
use std::io::stdin;
use std::process::Command;

fn main() {
    let mut square_size_str = String::new();
    println!("Square size:");
    stdin().read_line(&mut square_size_str).unwrap();
    let square_size = square_size_str.trim().parse::<usize>();
    let square_size: usize = if square_size.is_ok() {
        square_size.unwrap()
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
            _ => (),
        }
    }
}

fn random_generate(mut field: Vec<Vec<usize>>, square_size: usize) -> Vec<Vec<usize>> {
    for i in 1..square_size * square_size {
        let (v, h): (usize, usize) = loop {
            let v = thread_rng().gen_range(0..square_size);
            let h = thread_rng().gen_range(0..square_size);
            if field[v][h] == 0 {
                break (v, h);
            }
        };
        field[v][h] = i;
    }
    field
}

fn print(field: &Vec<Vec<usize>>) {
    let square_size = field.len();
    Command::new("cmd")
        .args(&["/c", "cls"])
        .spawn()
        .expect("cls command failed to start")
        .wait()
        .expect("failed to wait");
    field.iter().enumerate().for_each(|(i, x)| {
        if i == 0 {
            print!("┏");
        } else {
            print!("┣");
        }

        (0..(square_size * 6 + square_size)).for_each(|xr| {
            if (xr % 7 == 0) && xr > 2 && i == 0 {
                print!("┳");
            } else if xr % 7 == 0 && xr > 2 {
                print!("╋");
            } else if xr != 0 {
                print!("━");
            }
        });
        if i == 0 {
            print!("┓");
        } else {
            print!("┫");
        }
        println!();
        x.iter().for_each(|y| print!("┃{:^6}", y));
        println!("┃");
        if i == square_size - 1 {
            print!("┗");
            (0..(square_size * 6 + square_size)).for_each(|xr| {
                if (xr % 7 == 0) && xr > 2 {
                    print!("┻");
                } else if xr != 0 {
                    print!("━");
                }
            });
            println!("┛");
        }
    });
}

fn move_zero(mut field: Vec<Vec<usize>>, key: KeyCode) -> Vec<Vec<usize>> {
    let square_size = field.len();
    let (i, j): (usize, usize) = field
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            let s = x.iter().enumerate().find(|(_j, y)| **y == 0usize);
            match s {
                None => None,
                Some((j, _y)) => Some((i, j)),
            }
        })
        .next()
        .unwrap();
    match key {
        KeyCode::Left => {
            if j > 0 {
                let temp = field[i][j];
                field[i][j] = field[i][j - 1];
                field[i][j - 1] = temp;
                field
            } else {
                field
            }
        }
        KeyCode::Right => {
            if j < square_size - 1 {
                let temp = field[i][j];
                field[i][j] = field[i][j + 1];
                field[i][j + 1] = temp;
                field
            } else {
                field
            }
        }
        KeyCode::Up => {
            if i > 0 {
                let temp = field[i][j];
                field[i][j] = field[i - 1][j];
                field[i - 1][j] = temp;
                field
            } else {
                field
            }
        }
        KeyCode::Down => {
            if i < square_size - 1 {
                let temp = field[i][j];
                field[i][j] = field[i + 1][j];
                field[i + 1][j] = temp;
                field
            } else {
                field
            }
        }
        _ => field,
    }
}

fn check(field: &Vec<Vec<usize>>) -> bool {
    let square_size = field.len();
    let (_num, field): (Vec<usize>, Vec<usize>) = field
        .iter()
        .flatten()
        .enumerate()
        .filter(|(i, x)| *i + 1 == **x)
        .unzip();
    square_size * square_size == field.len() + 1
}
