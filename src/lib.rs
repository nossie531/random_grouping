/*!
 * Utility for random grouping.
 */

use helper::*;
use nameof::name_of;
use rand::rngs::ThreadRng;
use rand::seq::index::sample;
use rand::RngCore;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use simple_scan::IteratorSimpleScanExt;

/// Random grouping executor.
///
/// This struct is useful for grouping multiple items into some groups at random.
///
/// # Examples
///
/// ```
/// # use random_grouping::*;
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
    pub fn new() -> Self {
        Self {
            stable: false,
            rounding: SizeRounding::Floor,
            rng: Staff::new_own(Box::new(Pcg32::seed_from_u64(0))),
        }
    }

    /// Creates an instance with volatile random number seed.
    pub fn auto_seed() -> Self {
        Self {
            stable: false,
            rounding: SizeRounding::Floor,
            rng: Staff::new_own(Box::<ThreadRng>::default()),
        }
    }

    /// Creates an instance with the specified random number saed.
    pub fn from_seed(seed: u64) -> Self {
        Self {
            stable: false,
            rounding: SizeRounding::Floor,
            rng: Staff::new_own(Box::new(Pcg32::seed_from_u64(seed))),
        }
    }

    /// Creates an instance with the specified random number generator.
    pub fn from_rng(rng: &'r mut dyn RngCore) -> Self {
        Self {
            stable: false,
            rounding: SizeRounding::Floor,
            rng: Staff::new_borrow(rng),
        }
    }

    /// Returns `true` if original order is keeped at grouping.
    ///
    /// Default value is `false`.
    pub fn stable(&self) -> bool {
        self.stable
    }

    /// Returns rounding strategy for group size.
    ///
    /// Default value is [`Floor`](SizeRounding::Floor).
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
    pub fn divide_by_size<'t, T>(&mut self, samples: &'t [T], sizes: &[usize]) -> Vec<Vec<&'t T>> {
        if samples.len() < sizes.iter().sum() {
            panic!(
                "`{}` length is greater than `{}` total.",
                name_of!(samples),
                name_of!(sizes)
            );
        }

        let (len, amount) = (samples.len(), sizes.iter().sum::<usize>());
        let mut idxs = sample(&mut *self.rng, len, amount).into_vec();
        let mut results = Vec::with_capacity(sizes.len());

        for (total, size) in sizes.iter().cloned().trace2(0, |total, size| total + size) {
            let group_range = (total - size).min(samples.len())..total;
            let group_item_idxs = sort_if(self.stable, &mut idxs[group_range]);
            let group_items = from_idxs(samples, group_item_idxs);
            results.push(group_items);
        }

        return results;
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
    pub fn divide_by_ratio<'t, T>(&mut self, samples: &'t [T], ratios: &[f64]) -> Vec<Vec<&'t T>> {
        if !ratios.iter().all(check_ratio) {
            panic!("`{}` contains illegal value.", name_of!(ratios));
        }

        if ratios.iter().sum::<f64>() > 1.0 {
            panic!("`{}` total is greater than 1.", name_of!(ratios));
        }

        let sizes = ratios_to_sizes(self.rounding, ratios, samples.len());
        return self.divide_by_size(samples, &sizes);

        fn check_ratio(x: &f64) -> bool {
            !x.is_nan() && *x >= 0.0 && x.is_finite()
        }

        fn ratios_to_sizes(round: SizeRounding, ratios: &[f64], len: usize) -> Vec<usize> {
            match round {
                SizeRounding::Floor => rts_floor(ratios, len),
                SizeRounding::Tail => rts_tail(ratios, len),
                SizeRounding::Each => rts_each(ratios, len),
            }
        }

        fn rts_floor(ratios: &[f64], len: usize) -> Vec<usize> {
            let results = ratios.iter().map(|x| (x * len as f64).floor() as usize);
            results.collect()
        }

        fn rts_tail(ratios: &[f64], len: usize) -> Vec<usize> {
            let sizes = ratios.iter().map(|x| (x * len as f64).round() as usize);
            let points = sizes.trace(0, |&s, x| (s + x).min(len));
            let results = points.diff(0, |c, p| c - p);
            results.collect()
        }

        fn rts_each(ratios: &[f64], len: usize) -> Vec<usize> {
            let points = ratios.iter().trace(0.0, |&s, x| s + x);
            let points = points.map(move |x| (x * len as f64).round() as usize);
            let results = points.diff(0, |c, p| c - p);
            results.collect()
        }
    }
}

impl Default for RandomGrouping<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Rounding strategy for group size.
///
/// This enum is used for group size calculation in
/// [`divide_by_ratio`](RandomGrouping::divide_by_ratio) method. Group size is
/// almost the result of multiplying samples length and group ratio. But it
/// is real number therefore rounding to `usize` is required.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SizeRounding {
    /// Size is calculated with floor operation.
    ///
    /// Pros: If ratios are equal, result sizes are equal too.<br/>
    /// Cons: floor operation, not round operation.
    Floor,

    /// Size of tail group is truncated.
    ///
    /// Pros: If ratios are equal, result sizes are equal too.<br/>
    /// Cons: The size of the tail group could be cut down significantly.
    Tail,

    /// Size of each group is adjusted.
    ///
    /// Pros: Group size totals can be controlled.<br/>
    /// Cons: Even If ratios are equal, result sizes could be not equal.
    Each,
}

mod helper {
    use std::ops::{Deref, DerefMut};

    /// Sort given slice if flag is `true`.
    pub fn sort_if<'a>(flag: bool, slice: &'a mut [usize]) -> &'a [usize] {
        if flag {
            slice.sort();
        }

        slice
    }

    /// Collect specified index elements from given slice.
    pub fn from_idxs<'t, T>(slice: &'t [T], idxs: &[usize]) -> Vec<&'t T> {
        let mut result = Vec::with_capacity(idxs.len());

        for &idx in idxs {
            result.push(&slice[idx]);
        }

        result
    }

    /// A pointer type that owns or borrows data.
    pub enum Staff<'a, T: ?Sized + 'a> {
        /// Target data is owned.
        Own(Box<T>),
        /// Target data is borrowed.
        Borrow(&'a mut T),
    }

    impl<'a, T: ?Sized> Staff<'a, T> {
        /// Create owned instance.
        pub fn new_own(x: Box<T>) -> Self {
            Self::Own(x)
        }

        /// Create borrowed instance.
        pub fn new_borrow(x: &'a mut T) -> Self {
            Self::Borrow(x)
        }
    }

    impl<'a, T: ?Sized> Deref for Staff<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            match self {
                Self::Own(x) => x.deref(),
                Self::Borrow(x) => x,
            }
        }
    }

    impl<'a, T: ?Sized> DerefMut for Staff<'a, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            match self {
                Self::Own(x) => x.deref_mut(),
                Self::Borrow(x) => x,
            }
        }
    }
}
