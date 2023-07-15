use itertools::Itertools;

use crate::utils::read_file;

#[allow(dead_code)]
fn tuning() -> usize {
    read_file("src/day_six/input.txt")
        .as_bytes()
        .windows(4)
        .position(|window| window.iter().unique().count() == 4)
        .map(|pos| pos + 4)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stack_test() {
        assert_eq!(tuning(), 7);
    }
}
