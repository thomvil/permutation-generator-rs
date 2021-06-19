use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SinglePermutation32 {
    elems: BitIndex32,
    next_mod: u128,
    current_idx: u128,
}

impl SinglePermutation32 {
    pub fn new_unchecked(nb_elems: u8, idx: u128) -> Self {
        Self {
            elems: BitIndex32::new(nb_elems).unwrap(),
            next_mod: factorial(nb_elems - 1),
            current_idx: idx,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new(nb_elems: u8, idx: u128) -> Option<Self> {
        if idx >= factorial(nb_elems) - 1 {
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

impl Iterator for SinglePermutation32 {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.elems.nb_elements() == 0 {
            return None;
        }
        let bit_nb = self.current_idx / self.next_mod;
        self.current_idx -= bit_nb * self.next_mod;
        self.next_mod /= (self.elems.nb_elements() as u128).saturating_sub(2) + 1;
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
        assert_eq!(
            None,
            SinglePermutation32::new(30, 265252859812191058636308480000000)
        );
    }

    #[test]
    fn new_unchecked_iterator() {
        assert_eq!(
            (0..30).collect::<Vec<_>>().as_slice(),
            SinglePermutation32::new_unchecked(30, 0)
                .collect::<Vec<_>>()
                .as_slice()
        );

        assert_eq!(
            (0..30).rev().collect::<Vec<_>>().as_slice(),
            SinglePermutation32::new_unchecked(30, 265252859812191058636308480000000 - 1)
                .collect::<Vec<_>>()
                .as_slice()
        );
    }
}
