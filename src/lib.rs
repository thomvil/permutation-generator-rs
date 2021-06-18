use bit_index::*;

mod single_permutation;

pub use single_permutation::*;

#[inline]
fn factorial(nb_elems: u8) -> u128 {
    match nb_elems {
        0 | 1 | 2 => nb_elems as u128,
        _ => (1..=nb_elems).map(|i| i as u128).product(),
    }
}
