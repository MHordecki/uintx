use crate::{type_conversion, type_impl, u112, u120, u24, u40, u48, u56, u72, u80, u88, u96};


#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u104(pub(crate) [u8; 13]);

impl u104 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u104([a, b, c, d, e, f, g, h, i, j, k, l, m]) = self;
        u128::from_ne_bytes([0,0,0,a, b, c, d, e, f, g, h, i, j, k, l, m])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u104([a, b, c,d, e, f, g, h, i, j, k, l, m]) = self;
        u128::from_ne_bytes([a, b, c, d, e, f, g, h, i, j, k, l, m,0,0,0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, _, _, _] = n.to_ne_bytes();
        u104([a, b, c, d, e, f, g, h, i, j, k, l, m])
    }

    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [_, _, _, d, e, f, g, h, i, j, k, l, m, n, o, p] = n.to_ne_bytes();
        u104([d, e, f, g, h, i, j, k, l, m, n, o, p])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 13]) -> [u8; 13] {
        [data[12], data[11], data[10], data[9], data[8], data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0]]
    }
}


type_impl!(u104, u128, 13);
type_conversion!(u24::u24, u128, u104);
type_conversion!(u40::u40, u128, u104);
type_conversion!(u48::u48, u128, u104);
type_conversion!(u56::u56, u128, u104);
type_conversion!(u72::u72, u128, u104);
type_conversion!(u80::u80, u128, u104);
type_conversion!(u88::u88, u128, u104);
type_conversion!(u96::u96, u128, u104);
type_conversion!(u112::u112, u128, u104);
type_conversion!(u120::u120, u128, u104);
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut my_vec: Vec<u104> = Vec::with_capacity(512);
        my_vec.push(u104::from(u104::MAX_VALUE-10));
        my_vec.push(u104::from(4u32));
        my_vec.push(u104::from(4u32));
        println!("{}", my_vec[0]);
        println!("{}", my_vec[1]);
        let z2 = unsafe{my_vec[0].fetch_unsafe()};

        let n = unsafe {my_vec[0].unsafe_add_with_aligned_into_aligned(6u128)};
        println!("{:?}", n);
        println!("{:?}", u104::MAX_VALUE-n);
    }
}