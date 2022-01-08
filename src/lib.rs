use conditional::conditional;
use std::fmt::{self, Display, Formatter};

macro_rules! impl_type {
    ($utype:ident) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $utype(pub f32);

        impl From<$utype> for f32 {
            fn from(item: $utype) -> f32 {
                item.0
            }
        }

        impl From<f32> for $utype {
            fn from(item: f32) -> Self {
                Self(item)
            }
        }

        impl Display for $utype {
            // `f` is a buffer, and this method must write the formatted string into it
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "{} {}{}", self.0,
                    std::any::type_name::<Self>().split("::").last().unwrap(),
                    conditional!(self.0 == 1.0 ? "" : "s")
                )
            }
        }

        impl PartialEq for $utype {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

    };
    ($x:ident, $($y:ident),+) => (
        impl_type!($x);
        impl_type!($($y), +);
    )
}

macro_rules! impl_scaled_from {
    ( $from:ident, $to:ident, $scaler:literal ) => {
        impl From<$from> for $to {
            fn from(item: $from) -> Self {
                $to(item.0 * $scaler)
            }
        }

        impl From<$to> for $from {
            fn from(item: $to) -> Self {
                $from(item.0 / $scaler)
            }
        }
    };
}

impl_type!(Grain, Gram, Inch, MilliMeter);
impl_scaled_from!(Gram, Grain, 15.4324);
impl_scaled_from!(Inch, MilliMeter, 25.4);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gram_to_grain() {
        let g: Grain = Gram(1 as f32).into();
        println!("{}", g);
        assert_eq!(g, Grain::from(15.4324));
    }
    #[test]
    fn grain_to_gram() {
        let g: Gram = Grain(15.4324).into();
        println!("{}", g);
        assert_eq!(g, Gram::from(1.0));
    }
}
