pub trait ResultNonDebugUnwrap<T,E> {
    fn expect_nodebug(self, msg: &str) -> T;
    fn expect_err_nodebug(self, msg: &str) -> E;
    fn unwrap_nodebug(self) -> T;
    fn unwrap_err_nodebug(self) -> E;
}

impl<T,E> ResultNonDebugUnwrap<T,E> for Result<T,E> {
    #[inline]
    fn unwrap_nodebug(self) -> T {
        match self {
            Ok(t) => t,
            Err(_) => unwrap_failed::<E>("called `Result::unwrap()` on an `Err` value"),
        }
    }

    #[inline]
    fn expect_nodebug(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_) => unwrap_failed::<E>(msg),
        }
    }

    #[inline]
    fn unwrap_err_nodebug(self) -> E {
        match self {
            Ok(_) => unwrap_failed::<T>("called `Result::unwrap_err()` on an `Ok` value"),
            Err(e) => e,
        }
    }

    #[inline]
    fn expect_err_nodebug(self, msg: &str) -> E {
        match self {
            Ok(_) => unwrap_failed::<T>(msg),
            Err(e) => e,
        }
    }
}

#[inline(never)]
#[cold]
fn unwrap_failed<E>(msg: &str) -> ! {
    panic!("{}: {}", msg, std::any::type_name::<E>())
}