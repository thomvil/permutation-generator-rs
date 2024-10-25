use bit_index::BitIndex32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SinglePermutation32 {
    elems:       BitIndex32,
    next_mod:    u128,
    current_idx: u128,
}

impl SinglePermutation32 {
    #[must_use]
    pub fn new(nb_elems: u8, nb_perms: u128, idx: u128) -> Option<Self> {
        if idx >= nb_perms {
            None
        } else {
            Some(Self {
                elems:       BitIndex32::new(nb_elems).unwrap(),
                next_mod:    nb_perms / u128::from(nb_elems),
                current_idx: idx,
            })
        }
    }

    #[inline]
    #[must_use]
    pub fn nb_remaining(&self) -> usize { self.elems.nb_elements() as usize }
}

impl Iterator for SinglePermutation32 {
    type Item = u8;

    #[allow(clippy::cast_possible_truncation)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.elems.nb_elements() == 0 {
            return None;
        }
        let bit_nb = self.current_idx / self.next_mod;
        self.current_idx -= bit_nb * self.next_mod;
        self.next_mod /= u128::from(self.elems.nb_elements()).saturating_sub(2) + 1;
        self.elems.pop(bit_nb as u8)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let nb_remaining = self.nb_remaining();
        (nb_remaining, Some(nb_remaining))
    }

    fn count(self) -> usize { self.nb_remaining() }
}

#[cfg(test)]
mod tests {
    use crate::{SinglePermutation32, factorial::factorial128};

    fn single_perm(nb_elems: u8, idx: u128) -> Option<SinglePermutation32> {
        SinglePermutation32::new(nb_elems, factorial128(nb_elems), idx)
    }

    #[test]
    fn new() {
        assert_eq!(None, single_perm(30, 265_252_859_812_191_058_636_308_480_000_000));
    }

    #[test]
    fn new_unchecked_iterator() {
        assert_eq!(
            (0..30).collect::<Vec<_>>().as_slice(),
            single_perm(30, 0).unwrap().collect::<Vec<_>>().as_slice()
        );

        assert_eq!(
            (0..30).rev().collect::<Vec<_>>().as_slice(),
            single_perm(30, 265_252_859_812_191_058_636_308_480_000_000 - 1)
                .unwrap()
                .collect::<Vec<_>>()
                .as_slice()
        );
    }
}
