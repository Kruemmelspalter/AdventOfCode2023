use day_11::part2;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("../../input.txt");
    match part2(input) {
        Ok(result) => println!("=> Result: {}", result),
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}
