use std::ptr;

pub trait VecExt<T> {
    fn push_option(&mut self, o: Option<T>);

    fn grow_to_with<F: FnMut() -> T>(&mut self, size: usize, f: F);

    fn grow_to(&mut self, size: usize, value: T) where T: Clone;

    fn grow_to_default(&mut self, size: usize) where T: Default;

    fn insert_slice_copy(&mut self, index: usize, slice: &[T]) where T: Copy;

    fn insert_slice_clone(&mut self, index: usize, slice: &[T]) where T: Clone;

    fn extend_from_slice_copy(&mut self, slice: &[T]) where T: Copy;
}

impl<T> VecExt<T> for Vec<T> {
    #[inline]
    fn push_option(&mut self, o: Option<T>) {
        if let Some(o) = o {
            self.push(o);
        }
    }

    #[inline]
    fn grow_to_with<F: FnMut() -> T>(&mut self, size: usize, f: F) {
        if size > self.len() {
            self.resize_with(size, f);
        }
    }

    #[inline]
    fn grow_to(&mut self, size: usize, value: T) where T: Clone {
        if size > self.len() {
            self.resize(size, value);
        }
    }

    #[inline]
    fn grow_to_default(&mut self, size: usize) where T: Default {
        if size > self.len() {
            self.resize_with(size, T::default);
        }
    }

    #[inline]
    fn insert_slice_copy(&mut self, index: usize, slice: &[T]) where T: Copy {
        let vlen = self.len();
        let slen = slice.len();
        assert!(index <= vlen);
        assert!(slice.len() <= isize::MAX as usize); //no UB plz
        let dlen = vlen+slen;

        if dlen > self.capacity() {
            self.reserve(slice.len());
        }

        unsafe {
            {
                let s = slice.as_ptr();
                let p = self.as_mut_ptr().add(index);
                ptr::copy(p, p.add(slen), vlen - index);
                ptr::copy_nonoverlapping(s, p, slen);
            }
            self.set_len(dlen);
        }
    }

    #[inline]
    fn insert_slice_clone(&mut self, index: usize, slice: &[T]) where T: Clone {
        let vlen = self.len();
        let slen = slice.len();
        assert!(index <= vlen);
        assert!(slice.len() <= isize::MAX as usize); //no UB plz
        let dlen = vlen+slen;

        if dlen > self.capacity() {
            self.reserve(slice.len());
        }

        unsafe {
            {
                let mut p = self.as_mut_ptr().add(index);
                ptr::copy(p, p.add(slen), vlen - index);
                for v in slice {
                    ptr::write(p,v.clone());
                    p = p.offset(1);
                }
            }
            self.set_len(dlen);
        }
    }

    fn extend_from_slice_copy(&mut self, slice: &[T]) where T: Copy {
        self.insert_slice_copy(self.len(), slice);
    }
}

#[test]
fn insert_extra() {
    let mut a = vec![1,2,7,8];
    let b = [3,4,5,6];
    a.insert_slice_copy(2,&b);
    a.extend_from_slice_copy(&[9,10,11,12,13,14,15,16]);
    assert_eq!(&a,&[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
    assert_eq!(a.len(),16);
}
#[test]
fn insert_extra_b() {
    let mut a = vec![1,2,7,8];
    let b = [3,4,5,6];
    a.insert_slice_clone(2,&b);
    assert_eq!(&a,&[1,2,3,4,5,6,7,8]);
    assert_eq!(a.len(),8);
}