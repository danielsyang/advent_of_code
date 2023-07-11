use itertools::Itertools;

use crate::utils::read_file;

#[allow(dead_code)]
fn cleanup() -> usize {
    let input = read_file("src/day_four/input.txt");
    let a = input
        .lines()
        .map(|line| {
            // line looks like this: 2-4
            line.split("-")
                .map(|digit| {
                    digit
                        .parse::<usize>()
                        .expect("Invalid interval, digit is not parseable")
                })
                .collect_tuple::<(_, _)>()
                .map(|(start, end)| start..=end)
                .expect("Each interval should have a start and end")
        })
        .collect::<Vec<_>>();

    for aa in a {
        println!("{:?}", aa)
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cleanup_test() {
        assert_eq!(cleanup(), 2)
    }
}
