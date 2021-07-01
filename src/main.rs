use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::{thread_rng, Rng};
use std::io::{stdin, stdout};
use std::process::Command;

fn main() {
    let n: usize = 4;
    let mut arr = vec![vec![0; n]; n];
    arr = random_generate(arr, n);
    enable_raw_mode().unwrap();
    let no_modifiers = KeyModifiers::empty();
    loop {
        print(&arr);
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => arr = move_zero(arr, KeyCode::Right),
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => arr = move_zero(arr, KeyCode::Left),
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => arr = move_zero(arr, KeyCode::Up),
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => arr = move_zero(arr, KeyCode::Down),
            _ => (),
        }
        //stdin().read_line(&mut String::new());
    }
    disable_raw_mode().unwrap();
    stdin().read_line(&mut String::new());
}

fn random_generate(mut arr: Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    for i in 1..n * n {
        let (v, h): (usize, usize) = loop {
            let v = thread_rng().gen_range(0..n);
            let h = thread_rng().gen_range(0..n);
            if arr[v][h] == 0 {
                break (v, h);
            }
        };
        arr[v][h] = i;
    }
    arr
}

fn print(arr: &Vec<Vec<usize>>) {
    let n = arr.len();
    Command::new("cmd")
        .args(&["/c", "cls"])
        .spawn()
        .expect("cls command failed to start")
        .wait()
        .expect("failed to wait");
    arr.iter().enumerate().for_each(|(i, x)| {
        if i == 0 {
            print!("┏");
        } else {
            print!("┣");
        }

        (0..(n * 6 + n)).for_each(|xr| {
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
        if i == n - 1 {
            print!("┗");
            (0..(n * 6 + n)).for_each(|xr| {
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

fn move_zero(mut arr: Vec<Vec<usize>>, key: KeyCode) -> Vec<Vec<usize>> {
    let n = arr.len();
    let (i, (j, _y)): (usize, (usize, &usize)) = arr
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let s = x.iter().enumerate().find(|(j, y)| **y == 0usize);
            (i, s)
            //.map(|(j, y)| if *y == 0usize { (i, j) } else { (n, n) })
        })
        .filter(|(i, s)| s.is_some())
        .map(|(i, s)| (i, s.unwrap()))
        .next()
        .unwrap();
    match key {
        KeyCode::Left => {
            if j > 0 {
                let temp = arr[i][j];
                arr[i][j] = arr[i][j - 1];
                arr[i][j - 1] = temp;
                arr
            } else {
                arr
            }
        }
        KeyCode::Right => {
            if j < n - 1 {
                let temp = arr[i][j];
                arr[i][j] = arr[i][j + 1];
                arr[i][j + 1] = temp;
                arr
            } else {
                arr
            }
        }
        KeyCode::Up => {
            if i > 0 {
                let temp = arr[i][j];
                arr[i][j] = arr[i - 1][j];
                arr[i - 1][j] = temp;
                arr
            } else {
                arr
            }
        }
        KeyCode::Down => {
            if i < n - 1 {
                let temp = arr[i][j];
                arr[i][j] = arr[i + 1][j];
                arr[i + 1][j] = temp;
                arr
            } else {
                arr
            }
        }
        _ => arr,
    }
}
