use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SinglePermutation16 {
    elems: BitIndex16,
    next_mod: u64,
    current_idx: u64,
}

impl SinglePermutation16 {
    pub fn new_unchecked(nb_elems: u8, idx: u64) -> Self {
        Self {
            elems: BitIndex16::new(nb_elems).unwrap(),
            next_mod: factorial(nb_elems - 1) as u64,
            current_idx: idx,
        }
    }

    pub fn new(nb_elems: u8, idx: u64) -> Option<Self> {
        if (idx as u128) >= factorial(nb_elems) - 1 {
            None
        } else {
            Some(Self::new_unchecked(nb_elems, idx))
        }
    }

    #[inline]
    fn nb_remaining(&self) -> usize {
        self.elems.nb_elements() as usize
    }
}

impl Iterator for SinglePermutation16 {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.elems.nb_elements() == 0 {
            return None;
        }
        let bit_nb = self.current_idx / self.next_mod;
        self.current_idx -= bit_nb * self.next_mod;
        self.next_mod /= (self.elems.nb_elements() as u64).saturating_sub(2) + 1;
        self.elems.pop(bit_nb as u8)
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

    #[test]
    fn new() {
        assert_eq!(None, SinglePermutation16::new(10, 3628800));
    }

    #[test]
    fn new_unchecked_iterator() {
        assert_eq!(
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            SinglePermutation16::new_unchecked(10, 0)
                .collect::<Vec<_>>()
                .as_slice()
        );

        assert_eq!(
            &[1, 0, 2, 3, 4, 5, 6, 7, 8, 9],
            SinglePermutation16::new_unchecked(10, 362880)
                .collect::<Vec<_>>()
                .as_slice()
        );

        assert_eq!(
            &[2, 0, 1, 3, 4, 5, 6, 7, 8, 9],
            SinglePermutation16::new_unchecked(10, 362880 * 2)
                .collect::<Vec<_>>()
                .as_slice()
        );

        assert_eq!(
            &[3, 0, 1, 2, 4, 5, 6, 7, 8, 9],
            SinglePermutation16::new_unchecked(10, 362880 * 3)
                .collect::<Vec<_>>()
                .as_slice()
        );

        assert_eq!(
            &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            SinglePermutation16::new_unchecked(10, 3628800 - 1)
                .collect::<Vec<_>>()
                .as_slice()
        );
    }
}
