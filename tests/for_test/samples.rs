use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use random_grouping::RandomGrouping;
use static_assertions::const_assert;

const SAMPLE_SIZE: usize = 30;

pub fn create_target() -> RandomGrouping<'static> {
    RandomGrouping::new().with_stable(true)
}

pub fn create_rng() -> impl Rng {
    Pcg32::seed_from_u64(1)
}

pub fn create_samples() -> Vec<i32> {
    (0..SAMPLE_SIZE as i32).collect()
}

pub fn create_small_group_sizes() -> Vec<usize> {
    const RESULT: [usize; 3] = [8, 9, 10];
    const_assert!(const_sum(&RESULT) < SAMPLE_SIZE);
    RESULT.to_vec()
}

pub fn create_just_group_sizes() -> Vec<usize> {
    const RESULT: [usize; 3] = [9, 10, 11];
    const_assert!(const_sum(&RESULT) == SAMPLE_SIZE);
    RESULT.to_vec()
}

pub fn create_large_group_sizes() -> Vec<usize> {
    const RESULT: [usize; 3] = [10, 11, 12];
    const_assert!(const_sum(&RESULT) > SAMPLE_SIZE);
    RESULT.to_vec()
}

pub fn create_group_sizes_with_some_empty() -> Vec<usize> {
    vec![9, 0, 11]
}

pub fn create_group_ratios_with(ratio: f64) -> Vec<f64> {
    vec![0.3, 0.3, ratio]
}

// TODO: Essentially, `#[allow(dead_code)]` is not necessary.
// https://github.com/rust-lang/rust/issues/101699
#[allow(dead_code)]
const fn const_sum(sizes: &[usize]) -> usize {
    let mut result = 0;
    let mut i = 0;
    while i < sizes.len() {
        result += sizes[i];
        i += 1;
    }

    result
}
