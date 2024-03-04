#![cfg_attr(feature = "const-trait", feature(const_trait_impl))]

use core::ops::{Bound, RangeBounds};



/// Similar to [core::ops::RangeBounds], but for returning owned bounds instead for references. 
#[cfg_attr(feature = "const-trait", const_trait)]
pub trait OwnedRangeBounds<R>: RangeBounds<R> + Sized {
    fn into_bounds(self) -> (Bound<R>, Bound<R>);
}


macro_rules! impl_for_range_types {
    ($($c:tt)?) => {
        impl<R> $($c)? OwnedRangeBounds<R> for core::ops::Range<R> {
            fn into_bounds(self) -> (Bound<R>, Bound<R>) {
                (Bound::Included(self.start), Bound::Excluded(self.end))
            }
        }

        impl<R: Copy> $($c)? OwnedRangeBounds<R> for core::ops::Range<&R> {
            fn into_bounds(self) -> (Bound<R>, Bound<R>) {
                (Bound::Included(*self.start), Bound::Excluded(*self.end))
            }
        }

        impl<R> $($c)? OwnedRangeBounds<R> for core::ops::RangeInclusive<R> {
            fn into_bounds(self) -> (Bound<R>, Bound<R>) {
                let (start, end) = self.into_inner();
                (Bound::Included(start), Bound::Included(end))
            }
        }

        impl<R: Copy> $($c)? OwnedRangeBounds<R> for core::ops::RangeInclusive<&R> {
            fn into_bounds(self) -> (Bound<R>, Bound<R>) {
                (self.start_bound().map(|a| *a), self.end_bound().map(|a| *a))
            }
        }


        impl<R> $($c)? OwnedRangeBounds<R> for core::ops::RangeTo<R> {
            fn into_bounds(self) -> (Bound<R>, Bound<R>) {
                (Bound::Unbounded, Bound::Excluded(self.end))
            }
        }

        impl<R: Copy> $($c)? OwnedRangeBounds<R> for core::ops::RangeTo<&R> {
            fn into_bounds(self) -> (Bound<R>, Bound<R>) {
                (Bound::Unbounded, Bound::Excluded(*self.end))
            }
        }

        impl<R> $($c)? OwnedRangeBounds<R> for core::ops::RangeToInclusive<R> {
            fn into_bounds(self) -> (Bound<R>, Bound<R>) {
                (Bound::Unbounded, Bound::Included(self.end))
            }
        }

        impl<R: Copy> $($c)? OwnedRangeBounds<R> for core::ops::RangeToInclusive<&R> {
            fn into_bounds(self) -> (Bound<R>, Bound<R>) {
                (Bound::Unbounded, Bound::Included(*self.end))
            }
        }

        impl<R> $($c)? OwnedRangeBounds<R> for core::ops::RangeFrom<R> {
            fn into_bounds(self) -> (Bound<R>, Bound<R>) {
                (Bound::Included(self.start), Bound::Unbounded)
            }
        }

        impl<R: Copy> $($c)? OwnedRangeBounds<R> for core::ops::RangeFrom<&R> {
            fn into_bounds(self) -> (Bound<R>, Bound<R>) {
                (Bound::Included(*self.start), Bound::Unbounded)
            }
        }

        impl<R> $($c)? OwnedRangeBounds<R> for std::ops::RangeFull {
            fn into_bounds(self) -> (Bound<R>, Bound<R>) {
                (Bound::Unbounded, Bound::Unbounded)
            }
        }

    };
}

#[cfg(feature = "const-trait")]
impl_for_range_types!(const);
#[cfg(not(feature = "const-trait"))]
impl_for_range_types!();