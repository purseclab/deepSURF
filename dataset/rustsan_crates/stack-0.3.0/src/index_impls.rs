use std::ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull};
use array_vec::ArrayVec;
use small_vec::{SmallVec, Spilled};
use array::Array;

macro_rules! index_impl {
    ($t:ty) => {
        impl<T: Array> Index<$t> for ArrayVec<T> where [T::Item]: Index<$t> {
            type Output = <[T::Item] as Index<$t>>::Output;

            fn index(&self, index: $t) -> &Self::Output {
                Index::index(&**self, index)
            }
        }

        impl<T: Array> IndexMut<$t> for ArrayVec<T> {
            fn index_mut(&mut self, index: $t) -> &mut Self::Output {
                IndexMut::index_mut(&mut **self, index)
            }
        }

        impl<T: Array, S: Spilled<ArrayVec<T>> + Index<$t>> Index<$t> for SmallVec<T, S> where ArrayVec<T>: Index<$t, Output=<S as Index<$t>>::Output> {
            type Output = <S as Index<$t>>::Output;

            #[inline]
            fn index(&self, index: $t) -> &Self::Output {
                let v = self.0.as_ref();
                coalesce!(2 => |v| v.index(index))
            }
        }

        impl<T: Array, S: Spilled<ArrayVec<T>> + IndexMut<$t>> IndexMut<$t> for SmallVec<T, S> where ArrayVec<T>: IndexMut<$t, Output=<S as Index<$t>>::Output> {
            #[inline]
            fn index_mut(&mut self, index: $t) -> &mut Self::Output {
                let v = self.0.as_mut();
                coalesce!(2 => |v| v.index_mut(index))
            }
        }
    };
    ($($t:ty),+) => {
        $(
            index_impl!($t);
        )+
    };
}

index_impl!(usize, Range<usize>, RangeFrom<usize>, RangeTo<usize>, RangeFull);
