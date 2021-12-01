use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day1.txt");

    println!(
        "part 1: {}",
        input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .tuple_windows()
            .filter(|(a, b)| a < b)
            .count()
    );

    println!(
        "part 2: {}",
        input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(a, b)| a < b)
            .count()
    );
}
