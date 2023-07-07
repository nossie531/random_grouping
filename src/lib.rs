/*!
 * Utility for random grouping.
 */

pub mod random_grouping;
pub mod size_rounding;

pub use self::random_grouping::RandomGrouping;
pub use self::size_rounding::SizeRounding;

mod sized_iter;
mod staff;
