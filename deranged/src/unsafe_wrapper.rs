//! Declaration and implementation of `Unsafe`, which ensures all unsafe operations are correctly
//! placed in unsafe blocks.

/// A value that is safe to use, but is unsafe to construct or mutate.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy_derive::FromBytes,
        zerocopy_derive::IntoBytes,
        zerocopy_derive::Immutable,
        zerocopy_derive::KnownLayout,
    )
)]
#[repr(transparent)]
pub(crate) struct Unsafe<
    #[cfg(not(feature = "zerocopy"))] T,
    #[cfg(feature = "zerocopy")] T: zerocopy::IntoBytes + zerocopy::Immutable,
>(T);

impl<
        #[cfg(not(feature = "zerocopy"))] T: core::fmt::Debug,
        #[cfg(feature = "zerocopy")] T: core::fmt::Debug + zerocopy::IntoBytes + zerocopy::Immutable,
    > core::fmt::Debug for Unsafe<T>
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<
        #[cfg(not(feature = "zerocopy"))] T,
        #[cfg(feature = "zerocopy")] T: zerocopy::IntoBytes + zerocopy::Immutable,
    > Unsafe<T>
{
    /// Create a new `Unsafe`, asserting that all invariants are upheld.
    #[inline(always)]
    pub(crate) const unsafe fn new(value: T) -> Self {
        Self(value)
    }

    /// Get a reference to the inner value.
    #[inline(always)]
    pub(crate) const fn get(&self) -> &T {
        &self.0
    }
}

impl<
        #[cfg(not(feature = "zerocopy"))] T,
        #[cfg(feature = "zerocopy")] T: zerocopy::IntoBytes + zerocopy::Immutable,
    > core::ops::Deref for Unsafe<T>
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
