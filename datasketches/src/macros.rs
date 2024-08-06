macro_rules! wrap {
    ($(#[$attr:meta])* pub fn $name:tt() -> $ret:ty) => {
        #[inline]
        $(#[$attr])*
        pub fn $name(&self) -> $ret {
            self.0.$name()
        }
    };
    ($(#[$attr:meta])* pub fn $name:tt($($v:tt: $t:ty),+) -> $ret:ty) => {
        #[inline]
        $(#[$attr])*
        pub fn $name(&self, $($v: $t),+) -> $ret {
            self.0.$name($($v),+)
        }
    };
}

macro_rules! wrap_mut {
    ($(#[$attr:meta])* pub fn $name:tt() -> $ret:ty) => {
        #[inline]
        $(#[$attr])*
        pub fn $name(&mut self) -> $ret {
            self.0.pin_mut().$name()
        }
    };
    ($(#[$attr:meta])* pub fn $name:tt($($v:tt: $t:ty),+) -> $ret:ty) => {
        #[inline]
        $(#[$attr])*
        pub fn $name(&mut self, $($v: $t),+) -> $ret {
            self.0.pin_mut().$name($($v),+)
        }
    };
}

pub(crate) use wrap;
pub(crate) use wrap_mut;
