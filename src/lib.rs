use pyo3::prelude::*;

use std::collections::HashMap;
use std::time::Instant;

#[pyfunction]
pub fn two_sum(nums: Vec<i32>, target: i32) -> PyResult<Vec<i32>> {
    let mut index_hashmap = HashMap::new();

    for (idx, &n) in nums.iter().enumerate() {
        let y = target - n;
        if let Some(&i) = index_hashmap.get(&y) {
            return Ok(vec![i as i32, idx as i32]);
        } else {
            index_hashmap.insert(n, idx);
        }
    }

    Ok(vec![])
}
#[pyfunction]
pub fn sort(nums: Vec<i32>) -> PyResult<Vec<i32>> {
    let mut nums = nums;
    let time_tracker: Instant = Instant::now();
    nums.sort();
    println!("{:?}", time_tracker.elapsed());
    Ok(nums)
}

#[pymodule]
fn rustify(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(two_sum, m)?)?;
    m.add_function(wrap_pyfunction!(sort, m)?)?;
    Ok(())
}
