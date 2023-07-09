#[allow(dead_code)]
fn rucksack_organization() -> usize {
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rucksack_organization_test() {
        assert_eq!(rucksack_organization(), 157);
    }
}
