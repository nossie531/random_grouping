//! Provider of [`RandomGrouping`].

use crate::prelude::*;
use crate::sized_iter::SizedIter;
use crate::staff::Staff;
use rand::prelude::*;
use rand::seq::index::sample;
use rand_pcg::Pcg32;
use simple_scan::prelude::*;
use std::collections::BTreeMap;

/// Random grouping executor.
///
/// This struct is useful for grouping multiple items into some groups at random.
///
/// # Examples
///
/// ```
/// # use random_grouping::prelude::*;
/// let mut rg = RandomGrouping::new();
/// let samples = (0..10).collect::<Vec<_>>();
/// let ratios = [0.3, 0.3, 0.2];
///
/// let result = rg.divide_by_ratio(&samples, &ratios);
///
/// assert!(result.len() == ratios.len());
/// for i in 0..result.len() {
///     let group_size = (ratios[i] * samples.len() as f64).floor() as usize;
///     assert!(result[i].len() == group_size);
///     assert!(result[i].iter().all(|x| samples.contains(x)));
/// }
/// ```
pub struct RandomGrouping<'r> {
    /// Flag to adjust the order inside groups.
    stable: bool,
    /// Rounding strategy for group size.
    rounding: SizeRounding,
    /// Random number generator.
    rng: Staff<'r, dyn RngCore>,
}

impl<'r> RandomGrouping<'r> {
    /// Creates an instance with default random number seed.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an instance with volatile random number seed.
    #[must_use]
    pub fn auto_seed() -> Self {
        Self {
            rng: Staff::new_own(Box::<ThreadRng>::default()),
            ..Default::default()
        }
    }

    /// Creates an instance with the specified random number saed.
    #[must_use]
    pub fn from_seed(seed: u64) -> Self {
        Self {
            rng: Staff::new_own(Box::new(Pcg32::seed_from_u64(seed))),
            ..Default::default()
        }
    }

    /// Creates an instance with the specified random number generator.
    #[must_use]
    pub fn from_rng(rng: &'r mut dyn RngCore) -> Self {
        Self {
            rng: Staff::new_borrow(rng),
            ..Default::default()
        }
    }

    /// Returns `true` if original order is keeped at grouping.
    ///
    /// Default value is `true`.
    #[must_use]
    pub fn stable(&self) -> bool {
        self.stable
    }

    /// Returns rounding strategy for group size.
    ///
    /// Default value is [`Floor`](SizeRounding::Floor).
    #[must_use]
    pub fn rounding(&self) -> SizeRounding {
        self.rounding
    }

    /// Set stable flag.
    ///
    /// See also [`stable`](Self::stable).
    pub fn with_stable(mut self, value: bool) -> Self {
        self.stable = value;
        self
    }

    /// Set rounding strategy for group size.
    ///
    /// See also [`rounding`](Self::rounding).
    pub fn with_rounding(mut self, value: SizeRounding) -> Self {
        self.rounding = value;
        self
    }

    /// Group a collection of samples, with specifying the sizes of each group.
    ///
    /// Behavior of this method is affected by following values.
    ///
    /// * Random number generator and its seed (at construction).
    /// * Orders inside each groups (See [`stable`](Self::stable)).
    ///
    /// # Panics
    ///
    /// Panics if the samples length is less than group size total.
    pub fn divide_by_size<I>(&mut self, samples: I, sizes: &[usize]) -> Vec<Vec<I::Item>>
    where
        I: IntoIterator,
    {
        let mut samples_iter = samples.into_iter();
        let mut samples_iter = SizedIter::new(&mut samples_iter);
        let samples_len = samples_iter.size_hint().1.unwrap();
        let select_len = sizes.iter().sum::<usize>();

        if samples_len < sizes.iter().sum() {
            panic!("Samples length is greater than sizes total.");
        }

        let mut table = BTreeMap::new();
        let idxs = sample(&mut *self.rng, samples_len, select_len).into_vec();
        let group_areas = sizes.iter().cloned().trace2(0, |total, size| total + size);
        let group_ranges = group_areas.map(|(lower, upper)| lower..upper);

        for (group_idx, group_range) in group_ranges.enumerate() {
            for &group_item_idx in &idxs[group_range] {
                table.insert(group_item_idx, group_idx);
            }
        }

        let mut results = Vec::with_capacity(sizes.len());
        let mut prev_idx = -1;

        for &size in sizes {
            results.push(Vec::with_capacity(size));
        }

        for (idx, group_idx) in table {
            let idx_progress = (idx as isize - prev_idx) as usize;
            let sample = samples_iter.nth(idx_progress - 1).unwrap();
            results[group_idx].push(sample);
            prev_idx = idx as isize;
        }

        if !self.stable {
            for group in results.iter_mut() {
                group.shuffle(&mut *self.rng);
            }
        }

        results
    }

    /// Group a collection of samples, with specifying the ratios of each group.
    ///
    /// Behavior of this method is affected by following values.
    ///
    /// * Random number generator and its seed (at construction).
    /// * Orders inside each groups (See [`stable`](Self::stable)).
    /// * Rounding strategy for group size (See [`rounding`](Self::rounding)).
    ///
    /// # Panics
    ///
    /// Panics in the following cases.
    ///
    /// * Ratios contains NaN.
    /// * Ratios contains infinite value.
    /// * Ratios contains negative value.
    /// * Ratios summary is greater than 1.
    pub fn divide_by_ratio<I>(&mut self, samples: I, ratios: &[f64]) -> Vec<Vec<I::Item>>
    where
        I: IntoIterator,
    {
        if !ratios.iter().all(Self::check_ratio) {
            panic!("Ratios contains illegal value.");
        }

        if ratios.iter().sum::<f64>() > 1.0 {
            panic!("Ratios total is greater than 1.");
        }

        let mut samples_iter = samples.into_iter();
        let samples_iter = SizedIter::new(&mut samples_iter);
        let samples_len = samples_iter.size_hint().1.unwrap();
        let sizes = self.ratios_to_sizes(ratios, samples_len);
        self.divide_by_size(samples_iter, &sizes)
    }

    /// Group a slice of samples, with specifying the sizes of each group.
    ///
    /// Compared to [`divide_by_size`](Self::divide_by_size), this method
    /// can only be used for slices. However, it generally runs faster when
    /// [`stable`](Self::stable) is `false`.
    ///
    /// Behavior of this method is affected by following values.
    ///
    /// * Random number generator and its seed (at construction).
    /// * Orders inside each groups (See [`stable`](Self::stable)).
    ///
    /// # Panics
    ///
    /// Panics if the samples length is less than group size total.
    pub fn divide_slice_by_size<'t, T>(
        &mut self,
        samples: &'t [T],
        sizes: &[usize],
    ) -> Vec<Vec<&'t T>> {
        if samples.len() < sizes.iter().sum() {
            panic!("Samples length is greater than sizes total.");
        }

        let (len, amount) = (samples.len(), sizes.iter().sum::<usize>());
        let mut idxs = sample(&mut *self.rng, len, amount).into_vec();
        let mut results = Vec::with_capacity(sizes.len());

        for (lower, upper) in sizes.iter().cloned().trace2(0, |total, size| total + size) {
            let group_range = lower..upper;
            let group_item_idxs = sort_if(self.stable, &mut idxs[group_range]);
            let group_items = from_idxs(samples, group_item_idxs);
            results.push(group_items);
        }

        return results;

        fn sort_if(flag: bool, slice: &mut [usize]) -> &[usize] {
            if flag {
                slice.sort();
            }

            slice
        }

        fn from_idxs<'t, T>(slice: &'t [T], idxs: &[usize]) -> Vec<&'t T> {
            let mut result = Vec::with_capacity(idxs.len());

            for &idx in idxs {
                result.push(&slice[idx]);
            }

            result
        }
    }

    /// Group a slice of samples, with specifying the ratios of each group.
    ///
    /// Compared to [`divide_by_ratio`](Self::divide_by_ratio), this method
    /// can only be used for slices. However, it generally runs faster when
    /// [`stable`](Self::stable) is `false`.
    ///
    /// Behavior of this method is affected by following values.
    ///
    /// * Random number generator and its seed (at construction).
    /// * Orders inside each groups (See [`stable`](Self::stable)).
    /// * Rounding strategy for group size (See [`rounding`](Self::rounding)).
    ///
    /// # Panics
    ///
    /// Panics in the following cases.
    ///
    /// * Ratios contains NaN.
    /// * Ratios contains infinite value.
    /// * Ratios contains negative value.
    /// * Ratios summary is greater than 1.
    pub fn divide_slice_by_ratio<'t, T>(
        &mut self,
        samples: &'t [T],
        ratios: &[f64],
    ) -> Vec<Vec<&'t T>> {
        if !ratios.iter().all(Self::check_ratio) {
            panic!("Ratios contains illegal value.");
        }

        if ratios.iter().sum::<f64>() > 1.0 {
            panic!("Ratios total is greater than 1.");
        }

        let sizes = self.ratios_to_sizes(ratios, samples.len());
        self.divide_by_size(samples, &sizes)
    }

    /// Returns `true` if given value is valid as ratio.
    fn check_ratio(x: &f64) -> bool {
        !x.is_nan() && *x >= 0.0 && x.is_finite()
    }

    /// Convert group ratios to group sizes with total length and rounding strategy.
    fn ratios_to_sizes(&self, ratios: &[f64], len: usize) -> Vec<usize> {
        return match self.rounding() {
            SizeRounding::Floor => floor(ratios, len),
            SizeRounding::Tail => tail(ratios, len),
            SizeRounding::Each => each(ratios, len),
        };

        fn floor(ratios: &[f64], len: usize) -> Vec<usize> {
            let results = ratios.iter().map(|x| (x * len as f64).floor() as usize);
            results.collect()
        }

        fn tail(ratios: &[f64], len: usize) -> Vec<usize> {
            let sizes = ratios.iter().map(|x| (x * len as f64).round() as usize);
            let points = sizes.trace(0, |&s, x| (s + x).min(len));
            let results = points.diff(0, |c, p| c - p);
            results.collect()
        }

        fn each(ratios: &[f64], len: usize) -> Vec<usize> {
            let points = ratios.iter().trace(0.0, |&s, x| s + x);
            let points = points.map(move |x| (x * len as f64).round() as usize);
            let results = points.diff(0, |c, p| c - p);
            results.collect()
        }
    }
}

impl Default for RandomGrouping<'_> {
    fn default() -> Self {
        Self {
            stable: true,
            rounding: SizeRounding::Floor,
            rng: Staff::new_own(Box::new(Pcg32::seed_from_u64(0))),
        }
    }
}
