use rand::{thread_rng, Rng};
use std::io::{stdin, stdout};
use std::process::Command;

fn main() {
    let n: usize = 4;
    let mut arr = vec![vec![0; n]; n];
    arr = random_generate(arr, n);
    print(arr, n);

    stdin().read_line(&mut String::new()).unwrap();
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

fn print(arr: Vec<Vec<usize>>, n: usize) {
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
