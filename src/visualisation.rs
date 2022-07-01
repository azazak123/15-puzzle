use std::io::stdout;

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

pub fn print(field: &Vec<Vec<usize>>) {
    execute!(stdout(), Clear(ClearType::All)).unwrap();

    let square_size = field.len();

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

        print!("\r\n");
        x.iter().for_each(|y| print!("┃{:^6}", y));
        print!("┃\r\n");
        if i == square_size - 1 {
            print!("┗");
            (0..(square_size * 6 + square_size)).for_each(|xr| {
                if (xr % 7 == 0) && xr > 2 {
                    print!("┻");
                } else if xr != 0 {
                    print!("━");
                }
            });
            print!("┛\r\n");
        }
    });
}
