pub trait SetMinMax {
    fn set_min(&mut self, v: Self) -> bool;
    fn set_max(&mut self, v: Self) -> bool;
}
impl<T: PartialOrd> SetMinMax for T {
    fn set_min(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn set_max(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! div_floor {
    ($a:expr, $b:expr) => {
        if $b < 0 {
            if $a <= 0 {
                (-$a) / (-$b)
            } else {
                (-$a + 1) / (-$b) - 1
            }
        } else {
            if $a >= 0 {
                $a / $b
            } else {
                ($a + 1) / $b - 1
            }
        }
    };
}
#[macro_export]
macro_rules! div_ceil {
    ($a:expr, $b:expr) => {
        if $b < 0 {
            if $a < 0 {
                (-$a - 1) / (-$b) + 1
            } else {
                (-$a) / (-$b)
            }
        } else {
            if $a > 0 {
                ($a - 1) / $b + 1
            } else {
                $a / $b
            }
        }
    };
}
