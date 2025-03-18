//! This module provides a whole bunch of implementations for RelationElement<T>, where T is a tuple of a large length.
//! In particular, the tuple may be longer that stdlib's limit of 12 elements for default trait implementations.

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use serde_derive::{Deserialize, Serialize};

// This is unfortunately needed because we need trait impls for tuples of length > 12.
#[derive(Copy, Clone, Deserialize, Serialize)]
#[repr(transparent)]
pub struct RelationElement<T>(pub T);

impl<T> RelationElement<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for RelationElement<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RelationElement<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for RelationElement<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

// Stolen from redb/src/tuple_types.rs: https://github.com/cberner/redb/blob/0358e491c007429d4c6bab227d03d8b0398bf5dd/src/tuple_types.rs
mod redb_key_value_impls {
    use super::RelationElement;
    use redb::{Key, TypeName, Value};
    use std::borrow::Borrow;
    use std::cmp::Ordering;
    use std::mem::size_of;

    fn serialize_tuple_elements_variable(slices: &[&[u8]]) -> Vec<u8> {
        let total_len: usize = slices.iter().map(|x| x.len()).sum();
        let mut output = Vec::with_capacity((slices.len() - 1) * size_of::<u32>() + total_len);
        for len in slices.iter().map(|x| x.len()).take(slices.len() - 1) {
            output.extend_from_slice(&(u32::try_from(len).unwrap()).to_le_bytes());
        }

        for slice in slices {
            output.extend_from_slice(slice);
        }

        output
    }

    fn serialize_tuple_elements_fixed(slices: &[&[u8]]) -> Vec<u8> {
        let total_len: usize = slices.iter().map(|x| x.len()).sum();
        let mut output = Vec::with_capacity(total_len);
        for slice in slices {
            output.extend_from_slice(slice);
        }
        output
    }

    fn parse_lens<const N: usize>(data: &[u8]) -> [usize; N] {
        let mut result = [0; N];
        for i in 0..N {
            result[i] = u32::from_le_bytes(data[4 * i..4 * (i + 1)].try_into().unwrap()) as usize;
        }
        result
    }

    fn not_equal<T: Key>(data1: &[u8], data2: &[u8]) -> Option<Ordering> {
        match T::compare(data1, data2) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => None,
            Ordering::Greater => Some(Ordering::Greater),
        }
    }

    macro_rules! fixed_width_impl {
        ( $( $t:ty ),+ ) => {
            {
                let mut sum = 0;
                $(
                    sum += <$t>::fixed_width()?;
                )+
                Some(sum)
            }
        };
    }

    macro_rules! as_bytes_impl {
        ( $value:expr, $( $t:ty, $i:tt ),+ ) => {{
            if Self::fixed_width().is_some() {
                serialize_tuple_elements_fixed(&[
                    $(
                        <$t>::as_bytes($value.0.$i.borrow()).as_ref(),
                    )+
                ])
            } else {
                serialize_tuple_elements_variable(&[
                    $(
                        <$t>::as_bytes($value.0.$i.borrow()).as_ref(),
                    )+
                ])
            }
        }};
    }

    // Using TypeName's Debug impl because .name() is private.
    macro_rules! type_name_impl {
        ( $head:ty $(,$tail:ty)* ) => {
            {
                let mut result = String::new();
                result.push_str("qrates_RelationElement<(");
                result.push_str(&format!("{:?}", <$head>::type_name()));
                $(
                    result.push(',');
                    result.push_str(&format!("{:?}", <$tail>::type_name()));
                )*
                result.push_str(")>");

                TypeName::new(&result)
            }
        };
    }

    macro_rules! from_bytes_variable_impl {
        ( $data:expr $(,$t:ty, $v:ident, $i:literal )+ | $t_last:ty, $v_last:ident, $i_last:literal ) => {
            #[allow(clippy::manual_bits)]
            {
                let lens: [usize; $i_last] = parse_lens($data);
                let mut offset = $i_last * size_of::<u32>();
                $(
                    let len = lens[$i];
                    let $v = <$t>::from_bytes(&$data[offset..(offset + len)]);
                    offset += len;
                )+
                let $v_last = <$t_last>::from_bytes(&$data[offset..]);
                ($(
                    $v,
                )+
                    $v_last
                ).into()
            }
        };
    }

    macro_rules! from_bytes_fixed_impl {
        ( $data:expr $(,$t:ty, $v:ident )+ ) => {
            {
                let mut offset = 0;
                $(
                    let len = <$t>::fixed_width().unwrap();
                    let $v = <$t>::from_bytes(&$data[offset..(offset + len)]);
                    #[allow(unused_assignments)]
                    {
                        offset += len;
                    }
                )+

                ($(
                    $v,
                )+).into()
            }
        };
    }

    macro_rules! compare_variable_impl {
        ( $data0:expr, $data1:expr $(,$t:ty, $i:literal )+ | $t_last:ty, $i_last:literal ) => {
            #[allow(clippy::manual_bits)]
            {
                let lens0: [usize; $i_last] = parse_lens($data0);
                let lens1: [usize; $i_last] = parse_lens($data1);
                let mut offset0 = $i_last * size_of::<u32>();
                let mut offset1 = $i_last * size_of::<u32>();
                $(
                    let index = $i;
                    let len0 = lens0[index];
                    let len1 = lens1[index];
                    if let Some(order) = not_equal::<$t>(
                        &$data0[offset0..(offset0 + len0)],
                        &$data1[offset1..(offset1 + len1)],
                    ) {
                        return order;
                    }
                    offset0 += len0;
                    offset1 += len1;
                )+

                <$t_last>::compare(&$data0[offset0..], &$data1[offset1..])
            }
        };
    }

    macro_rules! compare_fixed_impl {
        ( $data0:expr, $data1:expr, $($t:ty),+ ) => {
            {
                let mut offset0 = 0;
                let mut offset1 = 0;
                $(
                    let len = <$t>::fixed_width().unwrap();
                    if let Some(order) = not_equal::<$t>(
                        &$data0[offset0..(offset0 + len)],
                        &$data1[offset1..(offset1 + len)],
                    ) {
                        return order;
                    }
                    #[allow(unused_assignments)]
                    {
                        offset0 += len;
                        offset1 += len;
                    }
                )+

                Ordering::Equal
            }
        };
    }

    macro_rules! tuple_impl {
        ( $($t:ident, $v:ident, $i:tt ),+ | $t_last:ident, $v_last:ident, $i_last:tt ) => {
            impl<$($t: Value,)+ $t_last: Value> Value for RelationElement<($($t,)+ $t_last)> {
                type SelfType<'a> = RelationElement<(
                    $(<$t>::SelfType<'a>,)+
                    <$t_last>::SelfType<'a>,
                )>
                where
                    Self: 'a;
                type AsBytes<'a> = Vec<u8>
                where
                    Self: 'a;

                fn fixed_width() -> Option<usize> {
                    fixed_width_impl!($($t,)+ $t_last)
                }

                fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
                where
                    Self: 'a,
                {
                    if Self::fixed_width().is_some() {
                        from_bytes_fixed_impl!(data $(,$t,$v)+, $t_last, $v_last)
                    } else {
                        from_bytes_variable_impl!(data $(,$t,$v,$i)+ | $t_last, $v_last, $i_last)
                    }
                }

                fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Vec<u8>
                where
                    Self: 'a,
                    Self: 'b,
                {
                    as_bytes_impl!(value, $($t,$i,)+ $t_last, $i_last)
                }

                fn type_name() -> TypeName {
                    type_name_impl!($($t,)+ $t_last)
                }
            }

            impl<$($t: Key,)+ $t_last: Key> Key for RelationElement<($($t,)+ $t_last)> {
                fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
                    if Self::fixed_width().is_some() {
                        compare_fixed_impl!(data1, data2, $($t,)+ $t_last)
                    } else {
                        compare_variable_impl!(data1, data2 $(,$t,$i)+ | $t_last, $i_last)
                    }
                }
            }
        };
    }

    tuple_impl! {
        T0, t0, 0
        | T1, t1, 1
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1
        | T2, t2, 2
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2
        | T3, t3, 3
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3
        | T4, t4, 4
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4
        | T5, t5, 5
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5
        | T6, t6, 6
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6
        | T7, t7, 7
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7
        | T8, t8, 8
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8
        | T9, t9, 9
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9
        | T10, t10, 10
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9,
        T10, t10, 10
        | T11, t11, 11
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9,
        T10, t10, 10,
        T11, t11, 11
        | T12, t12, 12
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9,
        T10, t10, 10,
        T11, t11, 11,
        T12, t12, 12
        | T13, t13, 13
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9,
        T10, t10, 10,
        T11, t11, 11,
        T12, t12, 12,
        T13, t13, 13
        | T14, t14, 14
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9,
        T10, t10, 10,
        T11, t11, 11,
        T12, t12, 12,
        T13, t13, 13,
        T14, t14, 14
        | T15, t15, 15
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9,
        T10, t10, 10,
        T11, t11, 11,
        T12, t12, 12,
        T13, t13, 13,
        T14, t14, 14,
        T15, t15, 15
        | T16, t16, 16
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9,
        T10, t10, 10,
        T11, t11, 11,
        T12, t12, 12,
        T13, t13, 13,
        T14, t14, 14,
        T15, t15, 15,
        T16, t16, 16
        | T17, t17, 17
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9,
        T10, t10, 10,
        T11, t11, 11,
        T12, t12, 12,
        T13, t13, 13,
        T14, t14, 14,
        T15, t15, 15,
        T16, t16, 16,
        T17, t17, 17
        | T18, t18, 18
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9,
        T10, t10, 10,
        T11, t11, 11,
        T12, t12, 12,
        T13, t13, 13,
        T14, t14, 14,
        T15, t15, 15,
        T16, t16, 16,
        T17, t17, 17,
        T18, t18, 18
        | T19, t19, 19
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9,
        T10, t10, 10,
        T11, t11, 11,
        T12, t12, 12,
        T13, t13, 13,
        T14, t14, 14,
        T15, t15, 15,
        T16, t16, 16,
        T17, t17, 17,
        T18, t18, 18,
        T19, t19, 19
        | T20, t20, 20
    }

    tuple_impl! {
        T0, t0, 0,
        T1, t1, 1,
        T2, t2, 2,
        T3, t3, 3,
        T4, t4, 4,
        T5, t5, 5,
        T6, t6, 6,
        T7, t7, 7,
        T8, t8, 8,
        T9, t9, 9,
        T10, t10, 10,
        T11, t11, 11,
        T12, t12, 12,
        T13, t13, 13,
        T14, t14, 14,
        T15, t15, 15,
        T16, t16, 16,
        T17, t17, 17,
        T18, t18, 18,
        T19, t19, 19,
        T20, t20, 20
        | T21, t21, 21
    }

    impl<T0: Value> Value for RelationElement<(T0,)> {
        type SelfType<'a>
            = RelationElement<(T0::SelfType<'a>,)>
        where
            Self: 'a;
        type AsBytes<'a>
            = Vec<u8>
        where
            Self: 'a;

        fn fixed_width() -> Option<usize> {
            T0::fixed_width()
        }

        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
        where
            Self: 'a,
        {
            (T0::from_bytes(data),).into()
        }

        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Vec<u8>
        where
            Self: 'a,
            Self: 'b,
        {
            as_bytes_impl!(value, T0, 0)
        }

        fn type_name() -> TypeName {
            type_name_impl!(T0)
        }
    }

    impl<T0: Key> Key for RelationElement<(T0,)> {
        fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
            not_equal::<T0>(data1, data2).unwrap_or(Ordering::Equal)
        }
    }
}

// Stolen from rust stdlib src/fmt/mod.rs
macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        impl<$($name:Debug),+> Debug for RelationElement<($($name,)+)> {
            #[allow(non_snake_case, unused_assignments)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut builder = f.debug_tuple("");
                let ($(ref $name,)+) = self.0;
                $(
                    builder.field(&$name);
                )+

                builder.finish()
            }
        }
        peel! { $($name,)+ }
    )
}

// up to 25
tuple! { T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, }

mod eq_impls {
    use super::RelationElement as RE;
    // stolen from core/tuple.rs

    use std::cmp::Ordering::{self, *};

    use std::ops::Deref;

    // Recursive macro for implementing n-ary tuple functions and operations
    //
    // Also provides implementations for tuples with lesser arity. For example, tuple_impls!(A B C)
    // will implement everything for (A, B, C), (A, B) and (A,).
    macro_rules! tuple_impls {
        // Stopping criteria (1-ary tuple)
        ($T:ident) => {
            tuple_impls!(@impl $T);
        };
        // Running criteria (n-ary tuple, with n >= 2)
        ($T:ident $( $U:ident )+) => {
            tuple_impls!($( $U )+);
            tuple_impls!(@impl $T $( $U )+);
        };
        // "Private" internal implementation
        (@impl $( $T:ident )+) => {
                impl<$($T: PartialEq),+> PartialEq for RE<($($T,)+)>

                {
                    #[inline]
                    fn eq(&self, other: &RE<($($T,)+)>) -> bool {
                        $( ${ignore($T)} self.deref().${index()} == other.deref().${index()} )&&+
                    }
                    #[inline]
                    fn ne(&self, other: &RE<($($T,)+)>) -> bool {
                        $( ${ignore($T)} self.deref().${index()} != other.deref().${index()} )||+
                    }
                }

                impl<$($T: Eq),+> Eq for RE<($($T,)+)>
                {}

                impl<$($T: PartialOrd),+> PartialOrd for RE<($($T,)+)>
                {
                    #[inline]
                    fn partial_cmp(&self, other: &RE<($($T,)+)>) -> Option<Ordering> {
                        lexical_partial_cmp!($( ${ignore($T)} self.deref().${index()}, other.deref().${index()} ),+)
                    }
                    #[inline]
                    fn lt(&self, other: &RE<($($T,)+)>) -> bool {
                        lexical_ord!(lt, Less, $( ${ignore($T)} self.deref().${index()}, other.deref().${index()} ),+)
                    }
                    #[inline]
                    fn le(&self, other: &RE<($($T,)+)>) -> bool {
                        lexical_ord!(le, Less, $( ${ignore($T)} self.deref().${index()}, other.deref().${index()} ),+)
                    }
                    #[inline]
                    fn ge(&self, other:&RE<($($T,)+)>) -> bool {
                        lexical_ord!(ge, Greater, $( ${ignore($T)} self.deref().${index()}, other.deref().${index()} ),+)
                    }
                    #[inline]
                    fn gt(&self, other: &RE<($($T,)+)>) -> bool {
                        lexical_ord!(gt, Greater, $( ${ignore($T)} self.deref().${index()}, other.deref().${index()} ),+)
                    }
                }

                impl<$($T: Ord),+> Ord for RE<($($T,)+)>
                {
                    #[inline]
                    fn cmp(&self, other: &RE<($($T,)+)>) -> Ordering {
                        lexical_cmp!($( ${ignore($T)} self.deref().${index()}, other.deref().${index()} ),+)
                    }
                }

                impl<$($T: Default),+> Default for RE<($($T,)+)> {
                    #[inline]
                    fn default() -> RE<($($T,)+)> {
                        RE(($({ let x: $T = Default::default(); x},)+))
                    }
                }

                impl<T> From<[T; ${count($T)}]> for RE<($(${ignore($T)} T,)+)> {
                    #[inline]
                    #[allow(non_snake_case)]
                    fn from(array: [T; ${count($T)}]) -> Self {
                        let [$($T,)+] = array;
                        RE(($($T,)+))
                    }
                }
        }
    }

    // Constructs an expression that performs a lexical ordering using method `$rel`.
    // The values are interleaved, so the macro invocation for
    // `(a1, a2, a3) < (b1, b2, b3)` would be `lexical_ord!(lt, opt_is_lt, a1, b1,
    // a2, b2, a3, b3)` (and similarly for `lexical_cmp`)
    //
    // `$ne_rel` is only used to determine the result after checking that they're
    // not equal, so `lt` and `le` can both just use `Less`.
    macro_rules! lexical_ord {
        ($rel: ident, $ne_rel: ident, $a:expr, $b:expr, $($rest_a:expr, $rest_b:expr),+) => {{
            let c = PartialOrd::partial_cmp(&$a, &$b);
            if c != Some(Equal) { c == Some($ne_rel) }
            else { lexical_ord!($rel, $ne_rel, $($rest_a, $rest_b),+) }
        }};
        ($rel: ident, $ne_rel: ident, $a:expr, $b:expr) => {
            // Use the specific method for the last element
            PartialOrd::$rel(&$a, &$b)
        };
    }

    macro_rules! lexical_partial_cmp {
        ($a:expr, $b:expr, $($rest_a:expr, $rest_b:expr),+) => {
            match ($a).partial_cmp(&$b) {
                Some(Equal) => lexical_partial_cmp!($($rest_a, $rest_b),+),
                ordering => ordering
            }
        };
        ($a:expr, $b:expr) => { ($a).partial_cmp(&$b) };
    }

    macro_rules! lexical_cmp {
        ($a:expr, $b:expr, $($rest_a:expr, $rest_b:expr),+) => {
            match $a.cmp(&$b) {
                Equal => lexical_cmp!($($rest_a, $rest_b),+),
                ordering => ordering
            }
        };
        ($a:expr, $b:expr) => { ($a).cmp(&$b) };
    }

    tuple_impls! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 }
}

mod hash_impls {
    use super::RelationElement as RE;
    // stolen from core/hash/mod.rs
    use std::hash::{Hash, Hasher};

    macro_rules! impl_hash_tuple {
        ( $($name:ident)+) => (
                impl<$($name: Hash),+> Hash for RE<($($name,)+)> {
                    #[allow(non_snake_case)]
                    #[inline]
                    fn hash<S: Hasher>(&self, state: &mut S) {
                        let RE(($(ref $name,)+)) = *self;
                        $($name.hash(state);)+
                    }
                }
        );
    }

    impl_hash_tuple! { T1 }
    impl_hash_tuple! { T1 T2 }
    impl_hash_tuple! { T1 T2 T3 }
    impl_hash_tuple! { T1 T2 T3 T4 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 }
    impl_hash_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, hash::{DefaultHasher, Hash, Hasher}};

    use redb::{Key, Value};

    use super::*;

    type RelationElement13 = super::RelationElement<(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32)>;


    #[test]
    fn test_relation_element() {
        // Test 13 elements, that's more than the standard library permits
        let x = RelationElement((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13));
        let y = RelationElement((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13));
        assert_eq!(x, y);
        
        let mut hasher_x = DefaultHasher::new();
        x.hash(&mut hasher_x);
        let mut hasher_y = DefaultHasher::new();
        y.hash(&mut hasher_y);
        assert_eq!(hasher_x.finish(), hasher_y.finish());

        let z = RelationElement((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14));
        assert_ne!(x, z);

        let mut hasher_z = DefaultHasher::new();
        z.hash(&mut hasher_z);
        assert_ne!(hasher_x.finish(), hasher_z.finish());

        let bytes_x: Vec<u8> = RelationElement13::as_bytes(&x);
        let x_from_bytes = RelationElement13::from_bytes(&bytes_x);
        assert_eq!(x, x_from_bytes);

        let bytes_y: Vec<u8> = RelationElement13::as_bytes(&y);

        let order = RelationElement13::compare(&bytes_x, &bytes_y);
        assert_eq!(order, Ordering::Equal);
    }
}