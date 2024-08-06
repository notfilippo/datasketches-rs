macro_rules! wrap {
    ($name:tt, $ret:ty) => {
        #[inline]
        pub fn $name(&self) -> $ret {
            self.0.$name()
        }
    };
    ($name:tt, $ret:ty, $($v:tt: $t:ty),+) => {
        #[inline]
        pub fn $name(&self, $($v: $t),+) -> $ret {
            self.0.$name($($v),+)
        }
    };
}

macro_rules! wrap_mut {
    ($name:tt, $ret:ty) => {
        #[inline]
        pub fn $name(&mut self) -> $ret {
            self.0.pin_mut().$name()
        }
    };
    ($name:tt, $ret:ty, $($v:tt: $t:ty),+) => {
        #[inline]
        pub fn $name(&mut self, $($v: $t),+) -> $ret {
            self.0.pin_mut().$name($($v),+)
        }
    };
}

pub(crate) use wrap;
pub(crate) use wrap_mut;
