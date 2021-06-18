use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SinglePermutation8 {
    elems: BitIndex8,
    next_mod: u16,
    current_idx: u16,
}

impl SinglePermutation8 {
    pub fn new_unchecked(nb_elems: u8, idx: u16) -> Self {
        Self {
            elems: BitIndex8::new(nb_elems).unwrap(),
            next_mod: factorial(nb_elems - 1) as u16,
            current_idx: idx,
        }
    }

    pub fn new(nb_elems: u8, idx: u16) -> Option<Self> {
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

impl Iterator for SinglePermutation8 {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.elems.nb_elements() == 0 {
            return None;
        }
        let bit_nb = self.current_idx / self.next_mod;
        self.current_idx -= bit_nb * self.next_mod;
        self.next_mod /= (self.elems.nb_elements() as u16).saturating_sub(2) + 1;
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
        assert_eq!(None, SinglePermutation8::new(4, 24));
    }

    #[test]
    fn new_unchecked_iterator() {
        assert_eq!(
            &[0, 1, 2, 3],
            SinglePermutation8::new_unchecked(4, 0)
                .collect::<Vec<_>>()
                .as_slice()
        );

        assert_eq!(
            &[1, 0, 2, 3],
            SinglePermutation8::new_unchecked(4, 6)
                .collect::<Vec<_>>()
                .as_slice()
        );

        assert_eq!(
            &[2, 0, 1, 3],
            SinglePermutation8::new_unchecked(4, 12)
                .collect::<Vec<_>>()
                .as_slice()
        );

        assert_eq!(
            &[3, 0, 1, 2],
            SinglePermutation8::new_unchecked(4, 18)
                .collect::<Vec<_>>()
                .as_slice()
        );

        assert_eq!(
            &[3, 2, 1, 0],
            SinglePermutation8::new_unchecked(4, 23)
                .collect::<Vec<_>>()
                .as_slice()
        );
    }
}
