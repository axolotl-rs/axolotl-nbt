macro_rules! gen_method_body {
    ($retur:expr, $func:ident($($arg:ty),*)) => {
        gen_method_body!($retur, $func($($arg),*), result: Self::Ok);
    };
    ($retur:expr, $func:ident($($arg:ty),*), result: $result_ty:path) => {
        fn $func(self, $(_: $arg),*) -> Result<$result_ty, Self::Error> {
            $retur
        }
    };
    ($retur:expr, $func:ident($($arg:ty),*), result: $result_ty:path, where: $where_v:path) => {
        #[inline]
        fn $func<__T: ?Sized>(self, $(_: $arg,)*)
                              -> ::std::result::Result<$result_ty, Self::Error>
            where __T: $where_v
        {
            $retur
        }
    };


}
macro_rules! method_body {
    ($retur:expr, bool) => {
        gen_method_body!($retur, serialize_bool(bool));
    };
    ($retur:expr, i8) => {
        gen_method_body!($retur, serialize_i8(i8));
    };
    ($retur:expr, i16) => {
        gen_method_body!($retur, serialize_i16(i16));
    };
    ($retur:expr, i32) => {
        gen_method_body!($retur, serialize_i32(i32));
    };
    ($retur:expr, i64) => {
        gen_method_body!($retur, serialize_i64(i64));
    };
    ($retur:expr, u8) => {
        gen_method_body!($retur, serialize_u8(u8));
    };
    ($retur:expr, u16) => {
        gen_method_body!($retur, serialize_u16(u16));
    };
    ($retur:expr, u32) => {
        gen_method_body!($retur, serialize_u32(u32));
    };
    ($retur:expr, u64) => {
        gen_method_body!($retur, serialize_u64(u64));
    };
    ($retur:expr, f32) => {
        gen_method_body!($retur, serialize_f32(f32));
    };
    ($retur:expr, f64) => {
        gen_method_body!($retur, serialize_f64(f64));
    };
    ($retur:expr, char) => {
        gen_method_body!($retur, serialize_char(char));
    };
    ($retur:expr, str) => {
        gen_method_body!($retur, serialize_str(&str));
    };
    ($retur:expr, bytes) => {
        gen_method_body! {$retur, serialize_bytes(&[u8])}
    };
    ($retur:expr, none) => {
        gen_method_body! {$retur, serialize_none()}
    };
    ($retur:expr, unit) => {
        gen_method_body! {$retur, serialize_unit()}
    };
    ($retur:expr, unit_struct) => {
        gen_method_body! {
            $retur, serialize_unit_struct(&'static str)
        }
    };
    ($retur:expr, unit_variant) => {
        gen_method_body! {
            $retur, serialize_unit_variant(&'static str, u32, &'static str)
        }
    };
    ($retur:expr, some) => {
        gen_method_body! {
            $retur, serialize_some(&__T),result: Self::Ok,
            where: ::serde::ser::Serialize
        }
    };
    ($retur:expr, newtype_struct) => {
        gen_method_body! {
            $retur, serialize_newtype_struct(&'static str, &__T),
            result: Self::Ok,
            where: ::serde::ser::Serialize
        }
    };
    ($retur:expr, newtype_variant) => {
        gen_method_body! {
            $retur, serialize_newtype_variant(&'static str, u32,
                                              &'static str, &__T),
            result: Self::Ok,
            where: ::serde::ser::Serialize
        }
    };
    ($retur:expr, seq) => {
        gen_method_body! {
            $retur, serialize_seq(Option<usize>),
            result: Self::SerializeSeq
        }
    };
    ($retur:expr, tuple) => {
        gen_method_body! {
            $retur, serialize_tuple(usize),
            result: Self::SerializeTuple
        }
    };
    ($retur:expr, tuple_struct) => {
        gen_method_body! {
            $retur, serialize_tuple_struct(&'static str, usize),
            result: Self::SerializeTupleStruct
        }
    };
    ($retur:expr, tuple_variant) => {
        gen_method_body! {
            $retur, serialize_tuple_variant(&'static str, u32, &'static str,
                                            usize),
            result: Self::SerializeTupleVariant
        }
    };
    ($retur:expr, struct_variant) => {
        gen_method_body! {
            $retur, serialize_struct_variant(&'static str, u32, &'static str,
                                             usize),
            result: Self::SerializeStructVariant
        }
    };
    ($retur:expr, map) => {
        gen_method_body! {
            $retur, serialize_map(Option<usize>),
            result: Self::SerializeMap
        }
    };
    ($retur:expr, struct) => {
        gen_method_body! {
            $retur, serialize_struct(&'static str, usize),
            result: Self::SerializeStruct
        }
    };
}

macro_rules! impossible {
    ($($t:tt),*) => {
        $(
            method_body!{Err(Error::UnrepresentableValueError(stringify!($t))), $t}
        )*
    };
}

macro_rules! name_impossible {
    ($($t:tt),*) => {
        $(
            method_body!{Err(Error::KeyMustBeString), $t}
        )*
    };
}
pub(crate) use gen_method_body;
pub(crate) use impossible;
pub(crate) use method_body;
