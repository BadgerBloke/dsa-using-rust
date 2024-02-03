pub fn better_approach(mut arr: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    println!("Input Array: {:?}", arr);

    let mut rows = vec![0; arr.len()];
    let mut cols = vec![0; arr[0].len()];

    for (i, row) in arr.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            if val == 0 {
                rows[i] = 1;
                cols[j] = 1;
            }
        }
    }

    for (i, &v) in rows.iter().enumerate() {
        if v == 1 {
            for val in arr[i].iter_mut() {
                *val = 0;
            }
        }
    }

    for (i, &v) in cols.iter().enumerate() {
        if v == 1 {
            for r in arr.iter_mut() {
                r[i] = 0;
            }
        }
    }

    println!("Output Array: {:?}", arr);
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_better_approach_case_1() {
        let input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let expected_output = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        let actual_output = better_approach(input.clone());
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_better_approach_case_2() {
        let input = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let expected_output = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        let actual_output = better_approach(input.clone());
        assert_eq!(actual_output, expected_output);
    }
}

