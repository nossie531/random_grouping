use crate::for_test::samples::*;
use random_grouping::prelude::*;
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
