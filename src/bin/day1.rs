use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day1.txt");

    /* part 1 */
    let count: usize = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .tuple_windows()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum();
    println!("part 1: {}", count);

    /* part 2 */
    let count: usize = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a+b+c)
        .tuple_windows()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum();
    println!("part 2: {}", count);
}
