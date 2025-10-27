/// macro for compact operator implementations
#[macro_export]
macro_rules! opion {
    ($op:tt($l:ty, $r:ty) |$li:ident,move $ri:ident| $f:block) => {
        $crate::_opion_grid!($op,$l,$r,$li,$ri,$f);
    };
    ($op:tt($l:ty,$r:ty) |$li:ident,&mut $ri:ident| $f:block) => {
        $crate::_opion_grid!($op,$l,&mut $r,$li,$ri,$f);
    };
    ($op:tt($l:ty,$r:ty) |$li:ident,&$ri:ident| $f:block) => {
        $crate::_opion_grid!($op,$l,&$r,$li,$ri,$f);
    };
    ($op:tt($l:ty,move $r:ty) |$li:ident,$ri:ident| $f:block) => {
        $crate::_opion_grid!($op,$l,$r,$li,$ri,$f);
    };
    ($op:tt($l:ty,&mut $r:ty) |$li:ident,$ri:ident| $f:block) => {
        $crate::_opion_grid!($op,$l,&mut $r,$li,$ri,$f);
    };
    ($op:tt($l:ty,&$r:ty) |$li:ident,$ri:ident| $f:block) => {
        $crate::_opion_grid!($op,$l,&$r,$li,$ri,$f);
    };
    ($op:tt($l:ty,$r:ty) |$li:ident,$ri:ident| $f:block) => {
        $crate::_opion_grid!($op,$l,$r,$li,$ri,$f);
        $crate::_opion_grid!($op,$l,&$r,$li,$ri,$f);
        $crate::_opion_grid!($op,$l,&mut $r,$li,$ri,$f);
    };
}

#[macro_export]
macro_rules! _opion_grid {
    (+,$($t:tt)+) => { $crate::_opion_grid!(add,$($t)+); };
    (-,$($t:tt)+) => { $crate::_opion_grid!(sub,$($t)+); };
    (*,$($t:tt)+) => { $crate::_opion_grid!(mul,$($t)+); };
    (/,$($t:tt)+) => { $crate::_opion_grid!(div,$($t)+); };
    (&,$($t:tt)+) => { $crate::_opion_grid!(bitand,$($t)+); };
    (|,$($t:tt)+) => { $crate::_opion_grid!(bitor,$($t)+); };
    (^,$($t:tt)+) => { $crate::_opion_grid!(bitxor,$($t)+); };
    (%,$($t:tt)+) => { $crate::_opion_grid!(rem,$($t)+); };
    (<<,$($t:tt)+) => { $crate::_opion_grid!(shl,$($t)+); };
    (>>,$($t:tt)+) => { $crate::_opion_grid!(shr,$($t)+); };
    (add,$($t:tt)+) => { $crate::_opion_inner!(Add,add,AddAssign,add_assign,$($t)+); };
    (sub,$($t:tt)+) => { $crate::_opion_inner!(Sub,sub,SubAssign,sub_assign,$($t)+); };
    (mul,$($t:tt)+) => { $crate::_opion_inner!(Mul,mul,MulAssign,mul_assign,$($t)+); };
    (div,$($t:tt)+) => { $crate::_opion_inner!(Div,div,DivAssign,div_assign,$($t)+); };
    (bitand,$($t:tt)+) => { $crate::_opion_inner!(BitAnd,bitand,BitAndAssign,bitand_assign,$($t)+); };
    (bitor,$($t:tt)+) => { $crate::_opion_inner!(BitOr,bitor,BitOrAssign,bitor_assign,$($t)+); };
    (bitxor,$($t:tt)+) => { $crate::_opion_inner!(BitXor,bitxor,BitXorAssign,bitxor_assign,$($t)+); };
    (rem,$($t:tt)+) => { $crate::_opion_inner!(Rem,rem,RemAssign,rem_assign,$($t)+); };
    (shl,$($t:tt)+) => { $crate::_opion_inner!(Shl,shl,ShlAssign,shl_assign,$($t)+); };
    (shr,$($t:tt)+) => { $crate::_opion_inner!(Shr,shr,ShrAssign,shr_assign,$($t)+); };
}

#[macro_export]
macro_rules! _opion_inner {
    ($op:ident,$fn:ident,$op_assign:ident,$fn_assign:ident,$l:ty,&mut $r:ty,$li:ident,$ri:ident,$f:block) => (        
        impl std::ops::$op_assign<&mut $r> for $l {
            #[inline]
            fn $fn_assign(&mut self, $ri: &mut $r) {
                let $li = self;
                $f
            }
        }
        impl<'a> std::ops::$op_assign<&mut $r> for &'a mut $l {
            #[inline]
            fn $fn_assign(&mut self, $ri: &mut $r) {
                let $li: &mut $l = *self;
                $f
            }
        }
        impl std::ops::$op<&mut $r> for $l {
            type Output=$l;
            #[inline]
            fn $fn(mut self, r: &mut $r) -> Self::Output {
                std::ops::$op_assign::$fn_assign(&mut self,r);
                self
            }
        }
        impl<'a> std::ops::$op<&mut $r> for &'a $l {
            type Output=$l;
            #[inline]
            fn $fn(self, r: &mut $r) -> Self::Output {
                std::ops::$op::$fn(Clone::clone(self),r)
            }
        }
        impl<'a> std::ops::$op<&mut $r> for &'a mut $l {
            type Output=$l;
            #[inline]
            fn $fn(self, r: &mut $r) -> Self::Output {
                std::ops::$op::$fn(Clone::clone(self),r)
            }
        }
    );
    ($op:ident,$fn:ident,$op_assign:ident,$fn_assign:ident,$l:ty,&$r:ty,$li:ident,$ri:ident,$f:block) => (        
        impl std::ops::$op_assign<&$r> for $l {
            #[inline]
            fn $fn_assign(&mut self, $ri: &$r) {
                let $li = self;
                $f
            }
        }
        impl<'a> std::ops::$op_assign<&$r> for &'a mut $l {
            #[inline]
            fn $fn_assign(&mut self, $ri: &$r) {
                let $li: &mut $l = *self;
                $f
            }
        }
        impl std::ops::$op<&$r> for $l {
            type Output=$l;
            #[inline]
            fn $fn(mut self, r: &$r) -> Self::Output {
                std::ops::$op_assign::$fn_assign(&mut self,r);
                self
            }
        }
        impl<'a> std::ops::$op<&$r> for &'a $l {
            type Output=$l;
            #[inline]
            fn $fn(self, r: &$r) -> Self::Output {
                std::ops::$op::$fn(Clone::clone(self),r)
            }
        }
        impl<'a> std::ops::$op<&$r> for &'a mut $l {
            type Output=$l;
            #[inline]
            fn $fn(self, r: &$r) -> Self::Output {
                std::ops::$op::$fn(Clone::clone(self),r)
            }
        }
    );
    ($op:ident,$fn:ident,$op_assign:ident,$fn_assign:ident,$l:ty,$r:ty,$li:ident,$ri:ident,$f:block) => (        
        impl std::ops::$op_assign<$r> for $l {
            #[inline]
            fn $fn_assign(&mut self, $ri: $r) {
                let $li = self;
                let $ri = &$ri;
                $f
            }
        }
        impl<'a> std::ops::$op_assign<$r> for &'a mut $l {
            #[inline]
            fn $fn_assign(&mut self, $ri: $r) {
                let $li: &mut $l = *self;
                $f
            }
        }
        impl std::ops::$op<$r> for $l {
            type Output=$l;
            #[inline]
            fn $fn(mut self, r: $r) -> Self::Output {
                std::ops::$op_assign::$fn_assign(&mut self,r);
                self
            }
        }
        impl<'a> std::ops::$op<$r> for &'a $l {
            type Output=$l;
            #[inline]
            fn $fn(self, r: $r) -> Self::Output {
                std::ops::$op::$fn(Clone::clone(self),r)
            }
        }
        impl<'a> std::ops::$op<$r> for &'a mut $l {
            type Output=$l;
            #[inline]
            fn $fn(self, r: $r) -> Self::Output {
                std::ops::$op::$fn(Clone::clone(self),r)
            }
        }
    );
}

#[derive(Clone)]
struct Test {}

opion!(add(Test,Test) |s,r| { *s -= r });
opion!(sub(Test,Test) |s,r| { *s *= r });
opion!(mul(Test,Test) |s,r| { *s /= r });
opion!(div(Test,Test) |s,r| { *s &= r });
opion!(bitand(Test,Test) |s,r| { *s |= r });
opion!(bitor(Test,Test) |s,r| { *s ^= r });
opion!(bitxor(Test,Test) |s,r| { *s %= r });
opion!(rem(Test,Test) |s,r| { *s <<= r });
opion!(shl(Test,Test) |s,r| { *s >>= r });
opion!(shr(Test,Test) |s,r| { *s += r });

#[derive(Clone)]
struct Test2 {}

opion!(+(Test2,Test2) |s,r| { *s -= r });
opion!(-(Test2,Test2) |s,r| { *s *= r });
opion!(*(Test2,Test2) |s,r| { *s /= r });
opion!(/(Test2,Test2) |s,r| { *s &= r });
opion!(&(Test2,Test2) |s,r| { *s |= r });
opion!(|(Test2,Test2) |s,r| { *s ^= r });
opion!(^(Test2,Test2) |s,r| { *s %= r });
opion!(%(Test2,Test2) |s,r| { *s <<= r });
opion!(<<(Test2,Test2) |s,r| { *s >>= r });
opion!(>>(Test2,Test2) |s,r| { *s += r });