use super::*;

#[derive(Clone, Copy, Debug)]
pub struct PermutationGenerator8 {
    nb_elems: u8,
    nb_perms: u128,
    next_idx: u128,
}

impl PermutationGenerator8 {
    pub fn new(nb_elems: u8) -> Self {
        if nb_elems > 8 {
            panic!("This PermutationGenerator can only handle 8 elements!");
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
            Some(SinglePermutation8::new_unchecked(
                nb_elems,
                idx.try_into().unwrap(),
            ))
        }
    }
}

impl Iterator for PermutationGenerator8 {
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

    const NB_ELEMS: u8 = 4;

    fn test_slice(ref_slice: &[u8], some_iter: Option<impl Iterator<Item = u8>>) {
        assert_eq!(ref_slice, some_iter.unwrap().collect::<Vec<_>>().as_slice());
    }

    #[test]
    fn zero() {
        let mut pg = PermutationGenerator8::new(0);
        assert!(pg.next_permutation().is_none());
    }

    #[test]
    fn next_permutation() {
        let mut pg = PermutationGenerator8::new(NB_ELEMS);
        test_slice(&[0, 1, 2, 3], pg.next_permutation());
        test_slice(&[0, 1, 3, 2], pg.next_permutation());

        let mut pg = PermutationGenerator8::new(NB_ELEMS + 1);
        test_slice(&[0, 1, 2, 3, 4], pg.next_permutation());
        test_slice(&[0, 1, 2, 4, 3], pg.next_permutation());
    }

    #[test]
    fn nth_absolute() {
        test_slice(
            &[0, 1, 2, 3],
            PermutationGenerator8::nth_absolute(NB_ELEMS, 0),
        );
        test_slice(
            &[3, 2, 1, 0],
            PermutationGenerator8::nth_absolute(NB_ELEMS, factorial(NB_ELEMS) - 1),
        );
        test_slice(
            &[4, 3, 2, 1, 0],
            PermutationGenerator8::nth_absolute(NB_ELEMS + 1, factorial(NB_ELEMS + 1) - 1),
        );
    }

    #[test]
    fn nth() {
        let mut pg = PermutationGenerator8::new(NB_ELEMS);
        test_slice(&[3, 2, 1, 0], pg.nth(factorial(NB_ELEMS) - 1));
        assert!(pg.next_permutation().is_none());

        let mut pg = PermutationGenerator8::new(NB_ELEMS);
        test_slice(&[0, 1, 2, 3], pg.nth(0));
        test_slice(&[3, 2, 1, 0], pg.nth(factorial(NB_ELEMS) - 2));
    }

    #[test]
    fn iter() {
        let list = PermutationGenerator8::new(NB_ELEMS)
            .map(|perm| perm.collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(factorial(NB_ELEMS) as usize, list.len());
        assert_eq!(&[0, 1, 2, 3], list[0].as_slice());
        assert_eq!(
            &[3, 2, 1, 0],
            list[factorial(NB_ELEMS) as usize - 1].as_slice()
        );
    }
}
