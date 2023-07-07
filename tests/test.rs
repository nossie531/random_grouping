mod for_test;

use crate::for_test::helper::*;
use crate::for_test::samples::*;
use random_grouping::RandomGrouping;
use random_grouping::SizeRounding;
use std::iter;
use test_panic::*;

#[test]
fn new() {
    let mut result = RandomGrouping::new();

    assert_eq!(result.stable(), true);
    assert_eq!(result.rounding(), SizeRounding::Floor);
    assert!(check_target(&mut result));
}

#[test]
fn auto_seed() {
    let mut result = RandomGrouping::auto_seed();

    assert_eq!(result.stable(), true);
    assert_eq!(result.rounding(), SizeRounding::Floor);
    assert!(check_target(&mut result));
}

#[test]
fn from_seed() {
    let mut result = RandomGrouping::from_seed(42);

    assert_eq!(result.stable(), true);
    assert_eq!(result.rounding(), SizeRounding::Floor);
    assert!(check_target(&mut result));
}

#[test]
fn from_rng() {
    let mut rng = create_rng();

    let mut result = RandomGrouping::from_rng(&mut rng);

    assert_eq!(result.stable(), true);
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
fn divide_slice_by_size() {
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

        let results = target.divide_slice_by_size(&samples, &sizes);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_gt_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_small_group_sizes();

        let results = target.divide_slice_by_size(&samples, &sizes);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_eq_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_just_group_sizes();

        let results = target.divide_slice_by_size(&samples, &sizes);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_lt_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_large_group_sizes();

        let result = test_panic(|| {
            target.divide_slice_by_size(&samples, &sizes);
        });

        assert!(result.is_panic());
    }

    fn with_stable() {
        let mut target = create_target().with_stable(true);
        let samples = create_samples();
        let sizes = create_just_group_sizes();

        let results = target.divide_slice_by_size(&samples, &sizes);

        assert!(results.iter().all(|x| is_group_stable(x, &samples)));
    }

    fn with_dup() {
        let mut target_x = create_target();
        let mut target_y = create_target();
        let samples = create_samples();
        let sizes = create_just_group_sizes();

        let results_x = target_x.divide_slice_by_size(&samples, &sizes);
        let results_y = target_y.divide_slice_by_size(&samples, &sizes);

        assert_eq!(results_x, results_y);
    }

    fn with_empty_group() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_group_sizes_with_some_empty();

        let results = target.divide_slice_by_size(&samples, &sizes);

        assert!(check_groups(&results, &sizes, &samples));
    }
}

#[test]
fn divide_slice_by_ratio() {
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

        let results = target.divide_slice_by_ratio(&samples, &ratios);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_gt_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_small_group_sizes();
        let ratios = sizes_to_ratios(&sizes, samples.len());

        let results = target.divide_slice_by_ratio(&samples, &ratios);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_eq_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_just_group_sizes();
        let ratios = sizes_to_ratios(&sizes, samples.len());

        let results = target.divide_slice_by_ratio(&samples, &ratios);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_samples_lt_group_totals() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_large_group_sizes();
        let ratios = sizes_to_ratios(&sizes, samples.len());

        let result = test_panic(|| {
            target.divide_slice_by_ratio(&samples, &ratios);
        });

        assert!(result.is_panic());
    }

    fn with_empty_group() {
        let mut target = create_target();
        let samples = create_samples();
        let sizes = create_group_sizes_with_some_empty();
        let ratios = sizes_to_ratios(&sizes, samples.len());

        let results = target.divide_slice_by_ratio(&samples, &ratios);

        assert!(check_groups(&results, &sizes, &samples));
    }

    fn with_nan_ratio_group() {
        let mut target = create_target();
        let samples = create_samples();
        let ratios = create_group_ratios_with(f64::NAN);

        let result = test_panic(|| {
            target.divide_slice_by_ratio(&samples, &ratios);
        });

        assert!(result.is_panic());
    }

    fn with_infinite_ratio_group() {
        let mut target = create_target();
        let samples = create_samples();
        let ratios = create_group_ratios_with(f64::INFINITY);

        let result = test_panic(|| {
            target.divide_slice_by_ratio(&samples, &ratios);
        });

        assert!(result.is_panic());
    }

    fn with_negative_ratio_group() {
        let mut target = create_target();
        let samples = create_samples();
        let ratios = create_group_ratios_with(-0.3);

        let result = test_panic(|| {
            target.divide_slice_by_ratio(&samples, &ratios);
        });

        assert!(result.is_panic());
    }

    fn with_stable() {
        let mut target = create_target().with_stable(true);
        let samples = create_samples();
        let sizes = create_just_group_sizes();
        let ratios = sizes_to_ratios(&sizes, samples.len());

        let results = target.divide_slice_by_ratio(&samples, &ratios);

        assert!(results.iter().all(|x| is_group_stable(x, &samples)));
    }

    fn with_rounding_floor() {
        let mut target = create_target().with_rounding(SizeRounding::Floor);
        let samples = (0..10).collect::<Vec<_>>();
        let ratios = iter::repeat(1.0 / 4.0).take(3).collect::<Vec<_>>();

        let results = target.divide_slice_by_ratio(&samples, &ratios);

        let expected_sizes = vec![2, 2, 2];
        assert!(check_groups(&results, &expected_sizes, &samples));
    }

    fn with_rounding_tail() {
        let mut target = create_target().with_rounding(SizeRounding::Tail);
        let samples = (0..10).collect::<Vec<_>>();
        let ratios = iter::repeat(1.0 / 4.0).take(4).collect::<Vec<_>>();

        let results = target.divide_slice_by_ratio(&samples, &ratios);

        let expected_sizes = vec![3, 3, 3, 1];
        assert!(check_groups(&results, &expected_sizes, &samples));
    }

    fn with_rounding_each() {
        let mut target = create_target().with_rounding(SizeRounding::Each);
        let samples = (0..10).collect::<Vec<_>>();
        let ratios = iter::repeat(1.0 / 3.0).take(3).collect::<Vec<_>>();

        let results = target.divide_slice_by_ratio(&samples, &ratios);

        let expected_sizes = vec![3, 4, 3];
        assert!(check_groups(&results, &expected_sizes, &samples));
    }
}

#[test]
fn default() {
    let mut result = RandomGrouping::default();
    check_target(&mut result);
}
