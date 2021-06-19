use super::*;

#[derive(Clone, Copy, Debug)]
pub struct PermutationGenerator16 {
    nb_elems: u8,
    nb_perms: u128,
    next_idx: u128,
}

impl PermutationGenerator16 {
    pub fn new(nb_elems: u8) -> Self {
        if nb_elems > 16 {
            panic!("This PermutationGenerator can only handle 16 elements!");
        }
        Self {
            next_idx: 0,
            nb_perms: factorial(nb_elems),
            nb_elems,
        }
    }

    pub fn next_permutation(&mut self) -> Option<impl Iterator<Item = u8>> {
        self.nth(0)
    }

    pub fn nth_absolute(nb_elems: u8, idx: u128) -> Option<impl Iterator<Item = u8>> {
        Self::nth_absolute_raw(nb_elems, factorial(nb_elems), idx)
    }

    pub fn nth(&mut self, step: u128) -> Option<impl Iterator<Item = u8>> {
        let res = Self::nth_absolute_raw(self.nb_elems, self.nb_perms, self.next_idx + step);
        self.next_idx += step + 1;
        res
    }

    pub fn nb_remaining(&self) -> usize {
        (self.nb_perms - self.next_idx) as usize
    }

    fn nth_absolute_raw(
        nb_elems: u8,
        nb_perms: u128,
        idx: u128,
    ) -> Option<impl Iterator<Item = u8>> {
        if idx >= nb_perms {
            None
        } else {
            Some(SinglePermutation16::new_unchecked(
                nb_elems,
                idx.try_into().unwrap(),
            ))
        }
    }
}

impl Iterator for PermutationGenerator16 {
    type Item = impl Iterator<Item = u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_permutation()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let nb_remaining = self.nb_remaining();
        (nb_remaining, Some(nb_remaining))
    }

    fn count(self) -> usize {
        self.nb_remaining()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NB_ELEMS: u8 = 9;

    fn test_slice(ref_slice: &[u8], some_iter: Option<impl Iterator<Item = u8>>) {
        assert_eq!(ref_slice, some_iter.unwrap().collect::<Vec<_>>().as_slice());
    }

    #[test]
    fn zero() {
        let mut pg = PermutationGenerator16::new(0);
        assert!(pg.next_permutation().is_none());
    }

    #[test]
    fn next_permutation() {
        let mut pg = PermutationGenerator16::new(NB_ELEMS);
        test_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8], pg.next_permutation());
        test_slice(&[0, 1, 2, 3, 4, 5, 6, 8, 7], pg.next_permutation());
    }

    #[test]
    fn nth_absolute() {
        test_slice(
            &[0, 1, 2, 3, 4, 5, 6, 7, 8],
            PermutationGenerator16::nth_absolute(NB_ELEMS, 0),
        );
        test_slice(
            &[8, 7, 6, 5, 4, 3, 2, 1, 0],
            PermutationGenerator16::nth_absolute(NB_ELEMS, factorial(NB_ELEMS) - 1),
        );
        test_slice(
            &[1, 0, 2, 3, 4, 5, 6, 7, 8],
            PermutationGenerator16::nth_absolute(NB_ELEMS, factorial(NB_ELEMS - 1)),
        );
    }

    #[test]
    fn nth() {
        let mut pg = PermutationGenerator16::new(NB_ELEMS);
        test_slice(
            &[8, 7, 6, 5, 4, 3, 2, 1, 0],
            pg.nth(factorial(NB_ELEMS) - 1),
        );
        assert!(pg.next_permutation().is_none());

        let mut pg = PermutationGenerator16::new(NB_ELEMS);
        test_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8], pg.nth(0));
        test_slice(
            &[1, 0, 2, 3, 4, 5, 6, 7, 8],
            pg.nth(factorial(NB_ELEMS - 1) - 1),
        );
    }

    #[test]
    fn iter() {
        let iter = PermutationGenerator16::new(NB_ELEMS);
        assert_eq!(factorial(NB_ELEMS) as usize, iter.count());
    }
}
