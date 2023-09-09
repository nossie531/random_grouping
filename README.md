random_grouping
===

Utility for random grouping.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This is useful for grouping multiple items into some groups at random.

## Examples

```rust
let mut rg = RandomGrouping::new();
let samples = (0..10).collect::<Vec<_>>();
let ratios = [0.3, 0.3, 0.2];

let result = rg.divide_by_ratio(&samples, &ratios);

assert!(result.len() == ratios.len());
for i in 0..result.len() {
    let group_size = (ratios[i] * samples.len() as f64).floor() as usize;
    assert!(result[i].len() == group_size);
    assert!(result[i].iter().all(|x| samples.contains(x)));
}
```

## What's New

At Version 0.2.3

* Bug fix: Remove unwanted Debug output.

At Vrrsion 0.2.0

* The default value of `stable` has been changed to `true` from `false`.
* The first argument of `divide_by_size` and `divide_by_ratio` is changed to
  `IntoIterator` from slice.
* Instead, `divide_slice_by_size` and `divide_slice_by_ratio` are introduced
  (which are faster for slices).