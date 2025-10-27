pub trait FromInto<T> {
    fn qfrom(t: T) -> Self;
    fn qinto(self) -> T;
}

impl<T,U> FromInto<U> for T where T: From<U> + Into<U> {
    #[inline]
    fn qfrom(t: U) -> Self {
        <T as From<U>>::from(t)
    }
    #[inline]
    fn qinto(self) -> U {
        <T as Into<U>>::into(self)
    }
}