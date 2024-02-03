mod arrays;

use arrays::*;

fn main() {
    let arr = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    bruteforce(arr.clone());
    better_approach(arr.clone());
    optimal_approach(arr.clone());
}

