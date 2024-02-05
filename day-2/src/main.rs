mod pascal_triangle;

use pascal_triangle::*;

fn main() {
    println!("Program to generate Pascal’s Triangle");

    let r = 7;
    let c = 4;

    let element = ncr(r, c);
    println!("This element at row: {} and col: {} is {}.", 6, 3, element);

    let row = nth_row(r);
    println!("This is nth row: {:?}", row);

    let pt = pascal_triangle(r);
    println!("This is Pascal Triangle having {} rows: {:?}", r, pt);
}

