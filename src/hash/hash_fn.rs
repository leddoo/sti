use crate::mem::PhantomData;


pub trait HashFn<T: ?Sized> {
    type Seed;
    type Hash;

    const DEFAULT_SEED: Self::Seed;

    fn hash_with_seed(seed: Self::Seed, value: &T) -> Self::Hash;

    #[inline(always)]
    fn hash(value: &T) -> Self::Hash {
        Self::hash_with_seed(Self::DEFAULT_SEED, value)
    }
}


pub trait HashFnSeed<T: ?Sized> {
    type Seed;
    type Hash;
    type F: HashFn<T, Seed=Self::Seed, Hash=Self::Hash>;

    fn seed(&self) -> Self::Seed;

    #[inline(always)]
    fn hash(&self, value: &T) -> Self::Hash {
        Self::F::hash_with_seed(self.seed(), value)
    }
}


#[derive(Copy)]
pub struct DefaultHashFnSeed<F>(PhantomData<F>);

impl<F> DefaultHashFnSeed<F> {
    #[inline(always)]
    pub const fn new() -> Self { Self(PhantomData) }
}

impl<F> Default for DefaultHashFnSeed<F> {
    #[inline(always)]
    fn default() -> Self { Self::new() }
}

impl<F> Clone for DefaultHashFnSeed<F> {
    #[inline(always)]
    fn clone(&self) -> Self { Self(PhantomData) }
}

impl<T: ?Sized, F: HashFn<T>> HashFnSeed<T> for DefaultHashFnSeed<F> {
    type Seed = F::Seed;
    type Hash = F::Hash;
    type F = F;

    #[inline(always)]
    fn seed(&self) -> Self::Seed { F::DEFAULT_SEED }
}

