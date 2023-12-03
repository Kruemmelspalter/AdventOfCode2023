use color_eyre::eyre::Result;
use itertools::Itertools;

pub fn process(s: &str) -> Result<i64> {
    let engine = s
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut nums = vec![];

    let mut num: Option<i64> = None;
    let mut part_location = None;

    for (x, row) in engine.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                num = match num {
                    None => Some(c.to_digit(10).unwrap() as i64),
                    Some(a) => Some(a * 10 + c.to_digit(10).unwrap() as i64),
                };
                if let Some((x, y, true)) = neighbors(&engine, x, y)?
                    .iter()
                    .map(|(x, y, c)| (x, y, is_symbol(**c)))
                    .find(|(_, _, b)| *b)
                {
                    part_location = Some((*x, *y));
                }
            } else if let Some(n) = num {
                if let Some((x, y)) = part_location {
                    // println!("detected part {} at {},{}", n, x, y);
                    nums.push((x, y, n));
                }
                num = None;
                part_location = None;
            }
        }
    }

    let mut res = 0;

    dbg!(nums
        .iter()
        .filter(|(x, y, _)| *x == 1 && *y == 28)
        .collect::<Vec<_>>());

    // dbg!(&nums);

    for (pos, group) in &nums
        .into_iter()
        .sorted_by_key(|(x, y, _)| (*x, *y))
        .group_by(|(x, y, _)| (*x, *y))
    {
        let group = group.collect::<Vec<_>>();
        if group.len() == 2 {
            let ratio: i64 = group.into_iter().map(|(_, _, p)| p).product();
            res += ratio;
        }
    }
    Ok(res)
}

fn is_symbol(c: char) -> bool {
    c == '*'
}

use std::cmp::{max, min};

fn neighbors<T>(
    m: &Vec<Vec<T>>,
    x_center: usize,
    y_center: usize,
) -> Result<Vec<(usize, usize, &T)>>
where
    T: std::fmt::Debug,
{
    let mut res = vec![];

    for x in max(0, x_center as isize - 1)..=min(x_center + 1, m.len() - 1) as isize {
        for y in max(0, y_center as isize - 1)..=min(y_center + 1, m[x as usize].len() - 1) as isize
        {
            // println!("x: {}, y: {}", x, y);
            let x = x as usize;
            let y = y as usize;
            if x != x_center || y != y_center {
                res.push((x, y, &m[x][y]));
            }
        }
    }

    // dbg!(&res);

    Ok(res)
}
