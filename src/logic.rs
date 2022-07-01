use crossterm::event::KeyCode;
use rand::{thread_rng, Rng};

pub fn check(field: &Vec<Vec<usize>>) -> bool {
    let square_size = field.len();
    let (_num, field): (Vec<usize>, Vec<usize>) = field
        .iter()
        .flatten()
        .enumerate()
        .filter(|(i, x)| *i + 1 == **x)
        .unzip();
    square_size * square_size == field.len() + 1
}

pub fn move_zero(mut field: Vec<Vec<usize>>, key: KeyCode) -> Vec<Vec<usize>> {
    let square_size = field.len();
    let (i, j): (usize, usize) = field
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            let s = x.iter().enumerate().find(|(_j, y)| **y == 0usize);
            s.map(|(j, _y)| (i, j))
        })
        .next()
        .unwrap();
    match key {
        KeyCode::Left => {
            if j > 0 {
                field[i].swap(j, j - 1);
                field
            } else {
                field
            }
        }
        KeyCode::Right => {
            if j < square_size - 1 {
                field[i].swap(j, j + 1);
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

pub fn random_generate(mut field: Vec<Vec<usize>>, square_size: usize) -> Vec<Vec<usize>> {
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
