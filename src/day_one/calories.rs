use crate::utils::read_file;

// My solution
#[allow(dead_code)]
fn calories() -> u64 {
    let input = read_file("src/day_one/input.txt");
    let mut accumulator = 0;
    let mut result = 0;

    input.lines().into_iter().for_each(|line| {
        if line.len() == 0 {
            if accumulator > result {
                result = accumulator;
            }
            accumulator = 0;
            return;
        }

        let number = line.parse::<u64>().unwrap();
        accumulator += number;
    });

    // checking last accumulator
    if accumulator > result {
        result = accumulator
    }

    return result;
}

#[allow(dead_code)]
fn calories_second_try() -> u64 {
    let input = read_file("src/day_one/input.txt");
    let lines = input.lines().collect::<Vec<_>>();
    let result = lines
        .split(|x| x.is_empty())
        .map(|group| group.iter().map(|x| x.parse::<u64>().unwrap()).sum::<u64>())
        .max()
        .unwrap_or_default();

    return result;
}

#[allow(dead_code)]
fn calories_advent_of_code() -> u64 {
    let mut result = 0;
    for group in include_str!("./input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
    {
        let mut sum = 0;
        for line in group.lines() {
            sum += line.parse::<u64>().unwrap();
        }

        if sum > result {
            result = sum;
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calories() {
        assert_eq!(24000, calories());
    }

    #[test]
    fn test_calories_second_try() {
        assert_eq!(24000, calories_second_try());
    }

    #[test]
    fn test_calories_advent_of_code() {
        assert_eq!(24000, calories_advent_of_code());
    }
}
