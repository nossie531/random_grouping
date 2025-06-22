//! Provider of [`SizeRounding`].

/// Rounding strategy for group size.
///
/// This enum is used for group size calculation in
/// [`divide_by_ratio`](crate::RandomGrouping::divide_by_ratio) method. Group
/// size is almost the result of multiplying samples length and group ratio.
/// But it is real number therefore rounding to `usize` is required.
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
