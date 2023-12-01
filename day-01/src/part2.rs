use color_eyre::eyre::Result;

pub fn process(s: &str) -> Result<i64> {
    let lines = s.lines();
    let mut sum = 0;
    for l in lines {
        let mut digits = vec![];
        for i in 0..l.len() {
            for j in i..l.len() {
                if [
                    "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three",
                    "four", "five", "six", "seven", "eight", "nine", "zero",
                ]
                .contains(&&l[i..=j])
                {
                    digits.push(match l[i..=j].parse() {
                        Ok(n) => n,
                        Err(_) => match &l[i..=j] {
                            "one" => 1,
                            "two" => 2,
                            "three" => 3,
                            "four" => 4,
                            "five" => 5,
                            "six" => 6,
                            "seven" => 7,
                            "eight" => 8,
                            "nine" => 9,
                            "zero" => 0,
                            _ => unreachable!(),
                        },
                    });
                }
            }
        }
        sum += digits.first().unwrap() * 10 + digits.iter().last().unwrap();
    }

    Ok(sum)
}
