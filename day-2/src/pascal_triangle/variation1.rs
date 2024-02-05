pub fn ncr(n: u32, r: u32) -> u32 {
    let mut res = 1;
    for i in 0..(r - 1) {
        res *= n - 1 - i;
        res /= i + 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let r = 5;
        let c = 3;
        let expected_res = 6;
        let actual_res = ncr(r, c);

        assert_eq!(expected_res, actual_res);
    }

    #[test]
    fn second_test() {
        let r = 1;
        let c = 1;
        let expected_res = 1;
        let actual_res = ncr(r, c);

        assert_eq!(expected_res, actual_res);
    }
}

