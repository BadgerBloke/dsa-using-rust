use std::i32;

pub fn bruteforce(mut arr: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    println!("Input Array: {:?}", arr);

    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if arr[i][j] == 0 {
                for val in arr[i].iter_mut() {
                    if *val != 0 {
                        *val = -1;
                    }
                }
                for row in arr.iter_mut() {
                    if row[j] != 0 {
                        row[j] = -1;
                    }
                }
            }
        }
    }

    for row in arr.iter_mut() {
        for val in row.iter_mut() {
            if *val == -1 {
                *val = 0
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
    fn test_bruteforce_case_1() {
        let input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let expected_output = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        let actual_output = bruteforce(input.clone());
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_bruteforce_case_2() {
        let input = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let expected_output = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        let actual_output = bruteforce(input.clone());
        assert_eq!(actual_output, expected_output);
    }
}

