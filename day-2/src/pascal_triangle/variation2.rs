use super::variation1::ncr;

pub fn nth_row(n: u32) -> Vec<u32> {
    let mut row = Vec::new();

    for i in 1..=n {
        row.push(ncr(n, i));
    }
    row
}

#[cfg(test)]
mod tests {
    use super::nth_row;

    #[test]
    fn first_test() {
        let n = 5;
        let expected_row = vec![1, 4, 6, 4, 1];
        let actual_row = nth_row(n);

        assert_eq!(expected_row, actual_row);
    }

    #[test]
    fn second_test() {
        let n = 1;
        let expected_row = vec![1];
        let actual_row = nth_row(n);

        assert_eq!(expected_row, actual_row);
    }
}

