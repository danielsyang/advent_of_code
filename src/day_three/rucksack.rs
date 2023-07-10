use std::collections::HashSet;

use crate::utils::read_file;

// I want to transform char into usize
#[derive(Debug, Clone, Copy)]
struct Letter(u8);

impl TryFrom<u8> for Letter {
    type Error = color_eyre::Report;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Letter(value)),
            _ => Err(color_eyre::eyre::eyre!(
                "{} is not a valid letter",
                value as char
            )),
        }
    }
}

impl Letter {
    pub(crate) fn letter_to_usize(self) -> usize {
        match self {
            Letter(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Letter(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Item {
    value: usize,
}

#[derive(Debug, Clone)]
struct Rucksack {
    left_compartment: String,
    right_comparment: String,
}

impl Rucksack {
    fn from_str_custom(s: &str) -> Self {
        let (left, right) = s.split_at(s.len() / 2);

        Rucksack {
            left_compartment: left.to_string(),
            right_comparment: right.to_string(),
        }
    }
}

impl Item {
    fn from_rucksack(rucksack: Rucksack) -> Self {
        let left = rucksack.left_compartment;
        let right = rucksack.right_comparment;

        let set = left.chars().collect::<HashSet<char>>();

        let found_char = right
            .chars()
            .find(|c| set.contains(&c))
            .expect("Invalid rucksack, no common character");

        // found_char.

        let value = Letter::try_from(found_char as u8)
            .unwrap()
            .letter_to_usize();

        Item { value }
    }
}

#[allow(dead_code)]
fn rucksack_organization() -> usize {
    let input = read_file("src/day_three/input.txt");

    return input
        .lines()
        .map(Rucksack::from_str_custom)
        .map(Item::from_rucksack)
        .map(|x| x.value)
        .sum::<usize>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rucksack_organization_test() {
        assert_eq!(rucksack_organization(), 157);
    }
}
