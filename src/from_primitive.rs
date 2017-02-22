

pub trait FromPrimitive {
    fn from_bool(t: bool) -> Self;

    fn from_usize(t: usize) -> Self;
    fn from_u8(t: u8) -> Self;
    fn from_u16(t: u16) -> Self;
    fn from_u32(t: u32) -> Self;
    fn from_u64(t: u64) -> Self;

    fn from_isize(t: isize) -> Self;
    fn from_i8(t: i8) -> Self;
    fn from_i16(t: i16) -> Self;
    fn from_i32(t: i32) -> Self;
    fn from_i64(t: i64) -> Self;

    fn from_f32(t: f32) -> Self;
    fn from_f64(t: f64) -> Self;
}


macro_rules! from_primitive {
    ($n:ident, $t:ident, $f:ident) => (
        #[inline(always)]
        fn $n(f: $f) -> Self { f as $t }
    );
    ($n:ident, $t:ident, $f:ident, $a:ident) => (
        #[inline(always)]
        fn $n(f: $f) -> Self { f as $a as $t }
    );
}

macro_rules! trait_from_primitive {
    ($t:ident) => (
        impl FromPrimitive for $t {
            from_primitive!(from_bool, $t, bool);

            from_primitive!(from_usize, $t, usize);
            from_primitive!(from_u8, $t, u8);
            from_primitive!(from_u16, $t, u16);
            from_primitive!(from_u32, $t, u32);
            from_primitive!(from_u64, $t, u64);

            from_primitive!(from_isize, $t, isize);
            from_primitive!(from_i8, $t, i8);
            from_primitive!(from_i16, $t, i16);
            from_primitive!(from_i32, $t, i32);
            from_primitive!(from_i64, $t, i64);

            from_primitive!(from_f32, $t, f32);
            from_primitive!(from_f64, $t, f64);
        }
    );
}

trait_from_primitive!(usize);
trait_from_primitive!(u8);
trait_from_primitive!(u16);
trait_from_primitive!(u32);
trait_from_primitive!(u64);

trait_from_primitive!(isize);
trait_from_primitive!(i8);
trait_from_primitive!(i16);
trait_from_primitive!(i32);
trait_from_primitive!(i64);


macro_rules! from_primitive_bool {
    ($n:ident, $f:ident, $zero:expr) => (
        #[inline(always)]
        fn $n(f: $f) -> Self { if f != $zero {true} else {false} }
    );
}

macro_rules! trait_bool_from_primitive {
    ($t:ident) => (
        impl FromPrimitive for $t {
            from_primitive_bool!(from_bool, bool, false);

            from_primitive_bool!(from_usize, usize, 0);
            from_primitive_bool!(from_u8, u8, 0);
            from_primitive_bool!(from_u16, u16, 0);
            from_primitive_bool!(from_u32, u32, 0);
            from_primitive_bool!(from_u64, u64, 0);

            from_primitive_bool!(from_isize, isize, 0);
            from_primitive_bool!(from_i8, i8, 0);
            from_primitive_bool!(from_i16, i16, 0);
            from_primitive_bool!(from_i32, i32, 0);
            from_primitive_bool!(from_i64, i64, 0);

            from_primitive_bool!(from_f32, f32, 0f32);
            from_primitive_bool!(from_f64, f64, 0f64);
        }
    );
}


trait_bool_from_primitive!(bool);


macro_rules! trait_float_from_primitive {
    ($t:ident, $a:ident) => (
        impl FromPrimitive for $t {
            from_primitive!(from_bool, $t, bool, $a);

            from_primitive!(from_usize, $t, usize);
            from_primitive!(from_u8, $t, u8);
            from_primitive!(from_u16, $t, u16);
            from_primitive!(from_u32, $t, u32);
            from_primitive!(from_u64, $t, u64);

            from_primitive!(from_isize, $t, isize);
            from_primitive!(from_i8, $t, i8);
            from_primitive!(from_i16, $t, i16);
            from_primitive!(from_i32, $t, i32);
            from_primitive!(from_i64, $t, i64);

            from_primitive!(from_f32, $t, f32);
            from_primitive!(from_f64, $t, f64);
        }
    );
}


trait_float_from_primitive!(f32, i32);
trait_float_from_primitive!(f64, i64);


#[cfg(test)]
mod test {
    use super::FromPrimitive;

    fn create_one<T: FromPrimitive>() -> T {
        T::from_f32(1_f32)
    }

    #[test]
    fn test() {
        assert_eq!(create_one::<bool>(), true);
        assert_eq!(create_one::<usize>(), 1_usize);
        assert_eq!(create_one::<f32>(), 1_f32);
    }
}
