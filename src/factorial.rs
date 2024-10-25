macro_rules! impl_factorial {
    ($fact:ident, $fact_type:ty) => {
        #[inline]
        pub(crate) fn $fact(nb_elems: u8) -> $fact_type {
            match nb_elems {
                0..=2 => <$fact_type>::from(nb_elems),
                _ => (1..=nb_elems).map(|i| <$fact_type>::from(i)).product(),
            }
        }
    };
}

impl_factorial!(factorial16, u16);
impl_factorial!(factorial64, u64);
impl_factorial!(factorial128, u128);
