mod imp;

/**
 * Clone function but only does cheap ref-cloning like rc
 */
pub trait RefClonable {
    fn refc(&self) -> Self;
}
