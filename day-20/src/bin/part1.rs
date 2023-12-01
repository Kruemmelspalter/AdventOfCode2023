use day_20::part1;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("../../input.txt");
    match part1(input) {
        Ok(result) => println!("=> Result: {}", result),
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}
