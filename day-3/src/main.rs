mod next_permutation;

use next_permutation::*;

fn main() {
    println!("next_permutation : find next lexicographically greater permutation");

    let mut arr = vec![4, 1, 7, 5, 3, 2, 0];
    next_permutation(&mut arr);
}

