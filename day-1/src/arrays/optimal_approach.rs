pub fn optimal_approach(mut arr: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    println!("Input Array: {:?}", arr);

    let mut row0 = 1;
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if i == 0 && arr[i][j] == 0 {
                row0 = 0;
            } else if i > 0 && arr[i][j] == 0 {
                arr[i][0] = 0;
                arr[0][j] = 0;
            }
        }
    }

    for i in 1..arr.len() {
        for j in 1..arr[i].len() {
            if arr[0][j] == 0 {
                arr[i][j] = 0;
            }
            if arr[i][0] == 0 {
                arr[i][j] = 0;
            }
        }
    }

    if row0 == 0 {
        for j in 0..arr[0].len() {
            arr[0][j] = 0;
        }
    }

    if arr[0][0] == 0 {
        for r in &mut arr {
            r[0] = 0;
        }
    }

    println!("Output Array: {:?}", arr);
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimal_approach_case_1() {
        let input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let expected_output = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        let actual_output = optimal_approach(input.clone());
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_optimal_approach_case_2() {
        let input = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let expected_output = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        let actual_output = optimal_approach(input.clone());
        assert_eq!(actual_output, expected_output);
    }
}

