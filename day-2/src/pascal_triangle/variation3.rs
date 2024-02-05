use super::variation2::nth_row;

pub fn pascal_triangle(n: u32) -> Vec<Vec<u32>> {
    let mut pt = Vec::new();

    for i in 1..=n {
        pt.push(nth_row(i));
    }

    pt
}

#[cfg(test)]
mod tests {
    use super::pascal_triangle;

    #[test]
    fn first_test() {
        let n = 5;
        let expected_pt = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        let actual_pt = pascal_triangle(n);

        assert_eq!(expected_pt, actual_pt);
    }

    #[test]
    fn second_test() {
        let n = 1;
        let expected_pt = vec![vec![1]];
        let actual_pt = pascal_triangle(n);

        assert_eq!(expected_pt, actual_pt);
    }
}

