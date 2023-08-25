use core::hash::Hash;


pub trait HashFn<H> {
    const DEFAULT_SEED: H;

    fn hash_with_seed<T: Hash + ?Sized>(seed: H, value: &T) -> H;

    #[inline(always)]
    fn hash<T: Hash + ?Sized>(value: &T) -> H {
        Self::hash_with_seed(Self::DEFAULT_SEED, value)
    }
}


pub mod fxhash;


