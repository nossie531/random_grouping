use crate::for_test::*;
use crate::samples::*;
use random_grouping::RandomGrouping;
use random_grouping::SizeRounding;
use std::iter;
use test_panic::*;

#[test]
fn new() {
    let mut result = RandomGrouping::new();

    assert_eq!(result.stable(), false);
    assert_eq!(result.rounding(), SizeRounding::Floor);
    assert!(check_target(&mut result));
}

#[test]
fn auto_seed() {
    let mut result = RandomGrouping::auto_seed();

    assert_eq!(result.stable(), false);
    assert_eq!(result.rounding(), SizeRounding::Floor);
    assert!(check_target(&mut result));
}

#[test]
fn from_seed() {
    let mut result = RandomGrouping::from_seed(42);

    assert_eq!(result.stable(), false);
    assert_eq!(result.rounding(), SizeRounding::Floor);
    assert!(check_target(&mut result));
}

#[test]
fn from_rng() {
    let mut rng = create_rng();

    let mut result = RandomGrouping::from_rng(&mut rng);

    assert_eq!(result.stable(), false);
    assert_eq!(result.rounding(), SizeRounding::Floor);
    assert!(check_target(&mut result));
}

#[test]
fn divide_by_size() {
    with_zero_groups();
    with_samples_gt_group_totals();
    with_samples_eq_group_totals();
    with_samples_lt_group_totals();
    with_stable();
    with_dup();
    with_empty_group();

    fn with_zero_groups() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = Vec::<usize>::new();

        let results = target.divide_by_size(&samples, &sizes);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_gt_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_small_group_sizes();

        let results = target.divide_by_size(&samples, &sizes);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_eq_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_just_group_sizes();

        let results = target.divide_by_size(&samples, &sizes);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_lt_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_large_group_sizes();

        let result = test_panic(|| {
            target.divide_by_size(&samples, &sizes);
        });

        assert!(result.is_panic());
    }

    fn with_stable() {
        let mut target = create_target().with_stable(true);
        let samples = create_samples();
        let sizes = create_just_group_sizes();

        let results = target.divide_by_size(&samples, &sizes);

        assert!(results.iter().all(|x| is_group_stable(x, &samples)));
    }

    fn with_dup() {
        let mut target_x = create_target();
        let mut target_y = create_target();
        let samples = create_samples();
        let sizes = create_just_group_sizes();

        let results_x = target_x.divide_by_size(&samples, &sizes);
        let results_y = target_y.divide_by_size(&samples, &sizes);

        assert_eq!(results_x, results_y);
    }

    fn with_empty_group() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_group_sizes_with_some_empty();

        let results = target.divide_by_size(&samples, &sizes);

        assert!(check_groups(&results, &sizes, &samples));
    }
}

#[test]
fn divide_by_ratio() {
    with_zero_groups();
    with_samples_gt_group_totals();
    with_samples_eq_group_totals();
    with_samples_lt_group_totals();
    with_empty_group();
    with_nan_ratio_group();
    with_infinite_ratio_group();
    with_negative_ratio_group();
    with_stable();
    with_rounding_floor();
    with_rounding_tail();
    with_rounding_each();

    fn with_zero_groups() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = Vec::<usize>::new();
        let ratios = sizes_to_ratios(&sizes, samples.len());

        let results = target.divide_by_ratio(&samples, &ratios);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_gt_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_small_group_sizes();
        let ratios = sizes_to_ratios(&sizes, samples.len());

        let results = target.divide_by_ratio(&samples, &ratios);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_eq_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_just_group_sizes();
        let ratios = sizes_to_ratios(&sizes, samples.len());

        let results = target.divide_by_ratio(&samples, &ratios);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_lt_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_large_group_sizes();
        let ratios = sizes_to_ratios(&sizes, samples.len());

        let result = test_panic(|| {
            target.divide_by_ratio(&samples, &ratios);
        });

        assert!(result.is_panic());
    }

    fn with_empty_group() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_group_sizes_with_some_empty();
        let ratios = sizes_to_ratios(&sizes, samples.len());

        let results = target.divide_by_ratio(&samples, &ratios);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_nan_ratio_group() {
        let mut target = create_target();
        let samples = create_samples();
        let ratios = create_group_ratios_with(f64::NAN);

        let result = test_panic(|| {
            target.divide_by_ratio(&samples, &ratios);
        });

        assert!(result.is_panic());
    }

    fn with_infinite_ratio_group() {
        let mut target = create_target();
        let samples = create_samples();
        let ratios = create_group_ratios_with(f64::INFINITY);

        let result = test_panic(|| {
            target.divide_by_ratio(&samples, &ratios);
        });

        assert!(result.is_panic());
    }

    fn with_negative_ratio_group() {
        let mut target = create_target();
        let samples = create_samples();
        let ratios = create_group_ratios_with(-0.3);

        let result = test_panic(|| {
            target.divide_by_ratio(&samples, &ratios);
        });

        assert!(result.is_panic());
    }

    fn with_stable() {
        let mut target = create_target().with_stable(true);
        let samples = create_samples();
        let sizes = create_just_group_sizes();
        let ratios = sizes_to_ratios(&sizes, samples.len());

        let results = target.divide_by_ratio(&samples, &ratios);

        assert!(results.iter().all(|x| is_group_stable(x, &samples)));
    }

    fn with_rounding_floor() {
        let mut target = create_target().with_rounding(SizeRounding::Floor);
        let samples = (0..10).collect::<Vec<_>>();
        let ratios = iter::repeat(1.0 / 4.0).take(3).collect::<Vec<_>>();

        let results = target.divide_by_ratio(&samples, &ratios);

        let expected_sizes = vec![2, 2, 2];
        assert!(check_groups(&results, &expected_sizes, &samples));
    }

    fn with_rounding_tail() {
        let mut target = create_target().with_rounding(SizeRounding::Tail);
        let samples = (0..10).collect::<Vec<_>>();
        let ratios = iter::repeat(1.0 / 4.0).take(4).collect::<Vec<_>>();

        let results = target.divide_by_ratio(&samples, &ratios);

        let expected_sizes = vec![3, 3, 3, 1];
        assert!(check_groups(&results, &expected_sizes, &samples));
    }

    fn with_rounding_each() {
        let mut target = create_target().with_rounding(SizeRounding::Each);
        let samples = (0..10).collect::<Vec<_>>();
        let ratios = iter::repeat(1.0 / 3.0).take(3).collect::<Vec<_>>();

        let results = target.divide_by_ratio(&samples, &ratios);

        let expected_sizes = vec![3, 4, 3];
        assert!(check_groups(&results, &expected_sizes, &samples));
    }
}

#[test]
fn default() {
    let mut result = RandomGrouping::default();
    check_target(&mut result);
}

mod samples {
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
}

mod for_test {
    use crate::samples::*;
    use random_grouping::RandomGrouping;
    use std::collections::HashSet;
    use std::hash::Hash;

    pub fn is_group_stable(group: &Vec<&i32>, samples: &[i32]) -> bool {
        let find_idx = |s: &i32| samples.iter().position(|x| s == x).unwrap();
        let idxs = group.iter().map(|&x| find_idx(x));
        let idxs = idxs.collect::<Vec<_>>();
        idxs.windows(2).all(|w| w[0] < w[1])
    }

    pub fn check_target(target: &mut RandomGrouping) -> bool {
        let samples = create_samples();
        let sizes = create_just_group_sizes();
        let results = target.divide_by_size(&samples, &sizes);
        check_groups(&results, &sizes, &samples)
    }

    pub fn check_groups(groups: &Vec<Vec<&i32>>, sizes: &[usize], samples: &[i32]) -> bool {
        if groups.len() != sizes.len() {
            return false;
        }

        if !unique_all(groups.iter().flatten()) {
            return false;
        }

        for (idx, group) in groups.iter().enumerate() {
            if !check_group(group, sizes[idx], samples) {
                return false;
            }
        }

        return true;

        fn check_group(group: &Vec<&i32>, size: usize, samples: &[i32]) -> bool {
            let len_ok = group.len() == size;
            let content_ok = group.iter().all(|x| samples.contains(x));
            len_ok && content_ok
        }
    }

    pub fn sizes_to_ratios(sizes: &[usize], len: usize) -> Vec<f64> {
        sizes
            .iter()
            .map(|&x| x as f64 / len as f64)
            .collect::<Vec<_>>()
    }

    fn unique_all<I>(iter: I) -> bool
    where
        I: IntoIterator,
        I::Item: Eq + Hash,
    {
        let mut result = true;
        let mut set = HashSet::new();
        for x in iter {
            result &= set.insert(x);
        }

        result
    }
}
