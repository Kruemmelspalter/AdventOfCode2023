use color_eyre::eyre::Result;

pub fn process(s: &str) -> Result<i64> {
    let engine = s
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut parts = vec![];

    let mut num: Option<i64> = None;
    let mut num_is_part = false;

    for (x, row) in engine.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                num = match num {
                    None => Some(c.to_digit(10).unwrap() as i64),
                    Some(a) => Some(a * 10 + c.to_digit(10).unwrap() as i64),
                };
                if num == Some(617) {
                    println!("617");
                }
                if neighbors(&engine, x, y)?.iter().any(|c| is_symbol(**c)) {
                    num_is_part = true;
                }
            } else if let Some(n) = num {
                if num_is_part {
                    println!("detected part {} at {},{}", n, x, y);
                    parts.push(n);
                }
                num = None;
                num_is_part = false;
            }
        }
    }
    return Ok(parts.iter().sum());
}

fn is_symbol(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
}

use std::cmp::{max, min};

fn neighbors<T>(m: &Vec<Vec<T>>, x_center: usize, y_center: usize) -> Result<Vec<&T>>
where
    T: std::fmt::Debug,
{
    let mut res: Vec<&T> = vec![];

    for x in max(0, x_center as isize - 1)..=min(x_center + 1, m.len() - 1) as isize {
        for y in max(0, y_center as isize - 1)..=min(y_center + 1, m[x as usize].len() - 1) as isize
        {
            println!("x: {}, y: {}", x, y);
            let x = x as usize;
            let y = y as usize;
            if x != x_center || y != y_center {
                res.push(&m[x][y]);
            }
        }
    }

    dbg!(&res);

    Ok(res)
}
