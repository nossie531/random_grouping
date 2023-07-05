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
