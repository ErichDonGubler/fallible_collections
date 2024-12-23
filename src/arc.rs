//! Implement a Fallible Arc
#[cfg(not(feature = "unstable"))]
use super::FallibleBox;
use super::TryClone;
use crate::TryReserveError;

#[cfg(not(feature = "unstable"))]
use alloc::boxed::Box;
use alloc::sync::Arc;

/// trait to implement Fallible Arc
#[cfg_attr(
    not(feature = "unstable"),
    deprecated(
        since = "0.3.1",
        note = "⚠️️️this function is not completely fallible, it can panic !, see [issue](https://github.com/vcombey/fallible_collections/issues/13). help wanted"
    )
)]
pub trait FallibleArc<T> {
    /// try creating a new Arc, returning a Result<Box<T>,
    /// TryReserveError> if allocation failed
    fn try_new(t: T) -> Result<Self, TryReserveError>
    where
        Self: Sized;
}

#[allow(deprecated)]
impl<T> FallibleArc<T> for Arc<T> {
    fn try_new(t: T) -> Result<Self, TryReserveError> {
        #[cfg(not(feature = "unstable"))]
        {
            // doesn't work as the inner variable of arc are also stocked in the box
            let b = <Box<T> as FallibleBox<T>>::try_new(t)?;
            Ok(Arc::from(b))
        }
        #[cfg(feature = "unstable")]
        {
            use alloc::alloc::Layout;
            use alloc::collections::TryReserveErrorKind;
            Arc::try_new(t).map_err(|_e| {
                TryReserveErrorKind::AllocError {
                    layout: Layout::new::<Arc<T>>(), // This is bullshit
                    non_exhaustive: (),
                }
                .into()
            })
        }
    }
}

/// Just a TryClone boilerplate for Arc
impl<T: ?Sized> TryClone for Arc<T> {
    fn try_clone(&self) -> Result<Self, TryReserveError> {
        Ok(self.clone())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn fallible_rc() {
        use std::sync::Arc;

        let mut x = Arc::new(3);
        *Arc::get_mut(&mut x).unwrap() = 4;
        assert_eq!(*x, 4);

        let _y = Arc::clone(&x);
        assert!(Arc::get_mut(&mut x).is_none());
    }
}
