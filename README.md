# Permutation Generator

Generates the basic permutations of `n` elements and length `n` in a direct fashion. It works index-based, not by iterating over previous permutations.

Currently only available on `nightly`, because it relies on `#![feature(min_type_alias_impl_trait)]`.

Optimized versions:
- `PermutationGenerator8`: for basic permutations upto 8 elements
- `PermutationGenerator16`: for basic permutations upto 16 elements
- `PermutationGenerator32`: for basic permutations upto 32 elements

Permutations of more than 32 elements are not provided, since the index of permutation cannot be represented by a single `u128`.

`PermutationGenerator`s implement `Iterator<Item = impl Iterator<Item = u8>>`.

## Usage

Iterate over the permutations of 4 elements
````rust
let mut pg = PermutationGenerator8::new(4).unwrap();
assert_eq!(&[0, 1, 2, 3], pg.next().unwrap().collect::<Vec<_>>().as_slice());
````

If the specified size of the permutations exceeds the capacity of the used `PermutationGenerator`, and `Err(PermutationError)` is returned.
````rust
let pg = PermutationGenerator8::new(14);
assert_eq!(PermutationGeneratorError::TooManyElements, pg.unwrap_err());
````

If only a single permutation is needed for a known idx
````rust
let mut pg = PermutationGenerator8::new(4).unwrap();
assert_eq!(&[0, 1, 2, 3], pg.next().unwrap().collect::<Vec<_>>().as_slice());

let last_perm_iter = PermutationGenerator8::nth_absolute(4, 4*3*2*1 - 1).unwrap();
assert_eq!(&[3, 2, 1, 0], last_perm_iter.collect::<Vec<_>>().as_slice()));
````

## Panics
The number of permutations of size larger than 20, cannot be represented by `u128`. Collecting all, or querying the `count`, will panic.
````rust
let pg = PermutationGenerator32::new(30);
pg.count() // -> panics!
````