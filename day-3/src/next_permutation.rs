pub fn next_permutation(arr: &mut Vec<i32>) {
    let mut idx: i32 = -1;
    let len = arr.len();

    for i in (0..len).rev() {
        if i > 0 && arr[i] > arr[i - 1] {
            idx = (i as i32) - 1;
            break;
        }
    }

    if idx == -1 {
        arr.sort();
        return;
    }

    for i in (0..len).rev() {
        if arr[i] > arr[idx as usize] {
            arr.swap(i, idx as usize);
            break;
        }
    }

    arr[idx as usize + 1..].sort();
}

#[cfg(test)]
mod tests {
    use super::next_permutation;

    #[test]
    fn first_test() {
        let mut arr = vec![1, 1, 5];
        let expected_output = vec![1, 5, 1];
        next_permutation(&mut arr);

        assert_eq!(expected_output, arr);
    }

    #[test]
    fn second_test() {
        let mut arr = vec![1, 2];
        let expected_output = vec![2, 1];
        next_permutation(&mut arr);

        assert_eq!(expected_output, arr);
    }

    #[test]
    fn third_test() {
        let mut arr = vec![4, 1, 7, 5, 3, 2, 0];
        let expected_output = vec![4, 2, 0, 1, 3, 5, 7];
        next_permutation(&mut arr);

        assert_eq!(expected_output, arr);
    }
}

