use color_eyre::eyre::Result;

pub fn process(s: &str) -> Result<i64> {
    let cards = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split([':', '|']))
        .map(|s| {
            let mut s = s.skip(1);
            (
                parse_numbers(s.next().unwrap()),
                parse_numbers(s.next().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let mut res = 0i64;

    for card in cards {
        let mut winning = 0;
        for win in card.0.iter() {
            for have in card.1.iter() {
                if *win == *have {
                    winning += 1;
                }
            }
        }

        res += 2i64.pow(winning) / 2;
    }
    Ok(res)
}

fn parse_numbers(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}
