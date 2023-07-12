use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::utils::read_file;

trait ValidateRange {
    fn contains_range(&self, other: &Self) -> bool;

    fn contained_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }
}

impl<T> ValidateRange for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
}

#[allow(dead_code)]
fn cleanup() -> usize {
    let input = read_file("src/day_four/input.txt");
    input
        .lines()
        .map(|line| {
            // line looks like this: 2-4,6-8
            line.split(",")
                .map(|interval| {
                    // interval looks like this 2-4
                    interval
                        .split("-")
                        .map(|digit| {
                            digit
                                .parse::<usize>()
                                .expect("Invalid interval, digit is not parseable")
                        })
                        .collect_tuple::<(_, _)>()
                        .map(|(start, end)| start..=end)
                        .expect("Each interval should have a start and end")
                })
                .collect_tuple::<(_, _)>()
                .expect("Each line must have a pair of intervals")
        })
        .filter(|(start, end)| start.contained_or_is_contained(&end))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cleanup_test() {
        assert_eq!(cleanup(), 2)
    }
}
