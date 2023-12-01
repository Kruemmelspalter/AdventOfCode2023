use color_eyre::eyre::Result;

pub fn process(s: &str) -> Result<i64> {
    let lines = s.lines();
    let mut sum = 0;
    for l in lines {
        let digits: Vec<char> = l.chars().filter(|c| c.is_ascii_digit()).collect();
        sum += digits.first().unwrap().to_digit(10).unwrap() * 10
            + digits.iter().last().unwrap().to_digit(10).unwrap();
    }

    Ok(sum as i64)
}
