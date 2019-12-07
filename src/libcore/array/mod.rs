//! 定长数组的 `Eq` 之类的实现. 限制到一定长度。 最终我们应该能够扩展到所有长度。
//!
//! *[另请参见数组原生类型](../../std/primitive.array.html).*

#![stable(feature = "core_array", since = "1.36.0")]

use crate::borrow::{Borrow, BorrowMut};
use crate::cmp::Ordering;
use crate::convert::{Infallible, TryFrom};
use crate::fmt;
use crate::hash::{Hash, self};
use crate::marker::Unsize;
use crate::slice::{Iter, IterMut};

#[cfg(not(bootstrap))]
mod iter;

#[cfg(not(bootstrap))]
#[unstable(feature = "array_value_iter", issue = "65798")]
pub use iter::IntoIter;

/// 仅在固定大小的数组上实现的实用特质
///
/// 此特质可用于在固定大小的数组上实现其他特质，而不会引起大量的元数据膨胀
///
/// 将该特质标记为不安全，以将实现者限制为固定大小的数组。 使用此特性的用户可以假定实现者在固定大小的数
/// 组的内存中具有准确的布局（例如，用于不安全的初始化）.
///
/// 请注意，特质[`AsRef`] 和 [`AsMut`] 为可能不是固定大小数组的类型提供了类似的方法。 实施者应该更喜欢这些特质
///
/// [`AsRef`]: ../convert/trait.AsRef.html
/// [`AsMut`]: ../convert/trait.AsMut.html
#[unstable(feature = "fixed_size_array", issue = "27778")]
pub unsafe trait FixedSizeArray<T> {
    /// Converts the array to immutable slice
    #[unstable(feature = "fixed_size_array", issue = "27778")]
    fn as_slice(&self) -> &[T];
    /// Converts the array to mutable slice
    #[unstable(feature = "fixed_size_array", issue = "27778")]
    fn as_mut_slice(&mut self) -> &mut [T];
}

#[unstable(feature = "fixed_size_array", issue = "27778")]
unsafe impl<T, A: Unsize<[T]>> FixedSizeArray<T> for A {
    #[inline]
    fn as_slice(&self) -> &[T] {
        self
    }
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

/// 从切片到数组的转换失败时返回的错误类型
#[stable(feature = "try_from", since = "1.34.0")]
#[derive(Debug, Copy, Clone)]
pub struct TryFromSliceError(());

#[stable(feature = "core_array", since = "1.36.0")]
impl fmt::Display for TryFromSliceError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.__description(), f)
    }
}

impl TryFromSliceError {
    #[unstable(feature = "array_error_internals",
           reason = "available through Error trait and this method should not \
                     be exposed publicly",
           issue = "0")]
    #[inline]
    #[doc(hidden)]
    pub fn __description(&self) -> &str {
        "could not convert slice to array"
    }
}

#[stable(feature = "try_from_slice_error", since = "1.36.0")]
impl From<Infallible> for TryFromSliceError {
    fn from(x: Infallible) -> TryFromSliceError {
        match x {}
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T, const N: usize> AsRef<[T]> for [T; N]
where
    [T; N]: LengthAtMost32,
{
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self[..]
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T, const N: usize> AsMut<[T]> for [T; N]
where
    [T; N]: LengthAtMost32,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

#[stable(feature = "array_borrow", since = "1.4.0")]
impl<T, const N: usize> Borrow<[T]> for [T; N]
where
    [T; N]: LengthAtMost32,
{
    fn borrow(&self) -> &[T] {
        self
    }
}

#[stable(feature = "array_borrow", since = "1.4.0")]
impl<T, const N: usize> BorrowMut<[T]> for [T; N]
where
    [T; N]: LengthAtMost32,
{
    fn borrow_mut(&mut self) -> &mut [T] {
        self
    }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl<T, const N: usize> TryFrom<&[T]> for [T; N]
where
    T: Copy,
    [T; N]: LengthAtMost32,
{
    type Error = TryFromSliceError;

    fn try_from(slice: &[T]) -> Result<[T; N], TryFromSliceError> {
        <&Self>::try_from(slice).map(|r| *r)
    }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl<'a, T, const N: usize> TryFrom<&'a [T]> for &'a [T; N]
where
    [T; N]: LengthAtMost32,
{
    type Error = TryFromSliceError;

    fn try_from(slice: &[T]) -> Result<&[T; N], TryFromSliceError> {
        if slice.len() == N {
            let ptr = slice.as_ptr() as *const [T; N];
            unsafe { Ok(&*ptr) }
        } else {
            Err(TryFromSliceError(()))
        }
    }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl<'a, T, const N: usize> TryFrom<&'a mut [T]> for &'a mut [T; N]
where
    [T; N]: LengthAtMost32,
{
    type Error = TryFromSliceError;

    fn try_from(slice: &mut [T]) -> Result<&mut [T; N], TryFromSliceError> {
        if slice.len() == N {
            let ptr = slice.as_mut_ptr() as *mut [T; N];
            unsafe { Ok(&mut *ptr) }
        } else {
            Err(TryFromSliceError(()))
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Hash, const N: usize> Hash for [T; N]
where
    [T; N]: LengthAtMost32,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        Hash::hash(&self[..], state)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: fmt::Debug, const N: usize> fmt::Debug for [T; N]
where
    [T; N]: LengthAtMost32,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&&self[..], f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, const N: usize> IntoIterator for &'a [T; N]
where
    [T; N]: LengthAtMost32,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, const N: usize> IntoIterator for &'a mut [T; N]
where
    [T; N]: LengthAtMost32,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B, const N: usize> PartialEq<[B; N]> for [A; N]
where
    A: PartialEq<B>,
    [A; N]: LengthAtMost32,
    [B; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &[B; N]) -> bool {
        self[..] == other[..]
    }
    #[inline]
    fn ne(&self, other: &[B; N]) -> bool {
        self[..] != other[..]
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B, const N: usize> PartialEq<[B]> for [A; N]
where
    A: PartialEq<B>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &[B]) -> bool {
        self[..] == other[..]
    }
    #[inline]
    fn ne(&self, other: &[B]) -> bool {
        self[..] != other[..]
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B, const N: usize> PartialEq<[A; N]> for [B]
where
    B: PartialEq<A>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &[A; N]) -> bool {
        self[..] == other[..]
    }
    #[inline]
    fn ne(&self, other: &[A; N]) -> bool {
        self[..] != other[..]
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'b, A, B, const N: usize> PartialEq<&'b [B]> for [A; N]
where
    A: PartialEq<B>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &&'b [B]) -> bool {
        self[..] == other[..]
    }
    #[inline]
    fn ne(&self, other: &&'b [B]) -> bool {
        self[..] != other[..]
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'b, A, B, const N: usize> PartialEq<[A; N]> for &'b [B]
where
    B: PartialEq<A>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &[A; N]) -> bool {
        self[..] == other[..]
    }
    #[inline]
    fn ne(&self, other: &[A; N]) -> bool {
        self[..] != other[..]
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'b, A, B, const N: usize> PartialEq<&'b mut [B]> for [A; N]
where
    A: PartialEq<B>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &&'b mut [B]) -> bool {
        self[..] == other[..]
    }
    #[inline]
    fn ne(&self, other: &&'b mut [B]) -> bool {
        self[..] != other[..]
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'b, A, B, const N: usize> PartialEq<[A; N]> for &'b mut [B]
where
    B: PartialEq<A>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &[A; N]) -> bool {
        self[..] == other[..]
    }
    #[inline]
    fn ne(&self, other: &[A; N]) -> bool {
        self[..] != other[..]
    }
}

// NOTE: some less important impls are omitted to reduce code bloat
// __impl_slice_eq2! { [A; $N], &'b [B; $N] }
// __impl_slice_eq2! { [A; $N], &'b mut [B; $N] }

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Eq, const N: usize> Eq for [T; N] where [T; N]: LengthAtMost32 {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: PartialOrd, const N: usize> PartialOrd for [T; N]
where
    [T; N]: LengthAtMost32,
{
    #[inline]
    fn partial_cmp(&self, other: &[T; N]) -> Option<Ordering> {
        PartialOrd::partial_cmp(&&self[..], &&other[..])
    }
    #[inline]
    fn lt(&self, other: &[T; N]) -> bool {
        PartialOrd::lt(&&self[..], &&other[..])
    }
    #[inline]
    fn le(&self, other: &[T; N]) -> bool {
        PartialOrd::le(&&self[..], &&other[..])
    }
    #[inline]
    fn ge(&self, other: &[T; N]) -> bool {
        PartialOrd::ge(&&self[..], &&other[..])
    }
    #[inline]
    fn gt(&self, other: &[T; N]) -> bool {
        PartialOrd::gt(&&self[..], &&other[..])
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord, const N: usize> Ord for [T; N]
where
    [T; N]: LengthAtMost32,
{
    #[inline]
    fn cmp(&self, other: &[T; N]) -> Ordering {
        Ord::cmp(&&self[..], &&other[..])
    }
}

/// Implemented for lengths where trait impls are allowed on arrays in core/std
#[rustc_on_unimplemented(
    message="arrays only have std trait implementations for lengths 0..=32",
)]
#[unstable(feature = "const_generic_impls_guard", issue = "0",
    reason = "will never be stable, just a temporary step until const generics are stable")]
pub trait LengthAtMost32 {}

macro_rules! array_impls {
    ($($N:literal)+) => {
        $(
            #[unstable(feature = "const_generic_impls_guard", issue = "0")]
            impl<T> LengthAtMost32 for [T; $N] {}
        )+
    }
}

array_impls! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

// 默认impls不能使用 array_impls！宏生成，因为它们需要数组字面值

macro_rules! array_impl_default {
    {$n:expr, $t:ident $($ts:ident)*} => {
        #[stable(since = "1.4.0", feature = "array_default")]
        impl<T> Default for [T; $n] where T: Default {
            fn default() -> [T; $n] {
                [$t::default(), $($ts::default()),*]
            }
        }
        array_impl_default!{($n - 1), $($ts)*}
    };
    {$n:expr,} => {
        #[stable(since = "1.4.0", feature = "array_default")]
        impl<T> Default for [T; $n] {
            fn default() -> [T; $n] { [] }
        }
    };
}

array_impl_default!{32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}
