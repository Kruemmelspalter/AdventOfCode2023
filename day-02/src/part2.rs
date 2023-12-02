use color_eyre::eyre::Result;

pub fn process(s: &str) -> Result<i64> {
    let lines = s.lines();

    let mut res: i64 = 0;

    for l in lines {
        if l.is_empty() {
            continue;
        }

        let mut split = l.split(|c| [':', ';'].contains(&c));
        let (num, draws): (&str, Vec<&str>) = (split.next().unwrap(), split.collect());

        let game_num = num.split(' ').last().unwrap().parse::<i64>().unwrap();

        let mut green = 0;
        let mut red = 0;
        let mut blue = 0;

        for d in draws {
            let cols = d
                .split(',')
                .map(|x| x.trim().split(' '))
                .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
                .collect::<Vec<_>>();

            for col in cols {
                let num = col.0.parse::<i64>().unwrap();
                match col.1 {
                    "green" if num > green => green = num,
                    "red" if num > red => red = num,
                    "blue" if num > blue => blue = num,
                    _ => {}
                }
            }
        }

        // println!("{}: {} {} {}", game_num, red, green, blue);

        res += red * green * blue;
    }
    Ok(res)
}
