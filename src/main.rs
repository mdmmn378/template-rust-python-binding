use crate::lib::{sort, two_sum};

mod lib;
fn main() {
    let nums = vec![4, 3, 2, 5, 6, 8];
    let target = 7;
    let result = two_sum(nums, target);
    println!("{:?}", result);

    let nums = vec![4, 3, 2, 5, 6, 8];
    println!("{:?}", sort(nums));
}
