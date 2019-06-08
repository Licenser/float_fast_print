#[inline]
pub fn print_64(v: &mut u64, dst: &mut [u8]) -> u32 {

    // This is slightly faster than a loop.
    // The average output length is 16.38 digits, so we check high-to-low.
    // Function precondition: v is not an 18, 19, or 20-digit number.
    // (17 digits are sufficient for round-tripping.)
    debug_assert!(*v < 100000000000000000u64);
    unsafe{
        if *v >= 10000000000000000u64 {
            *dst.get_unchecked_mut(16) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(15) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(14) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(13) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(12) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(11) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(10) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(9) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(8) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(7) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(6) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            17
        }
        else if *v >= 1000000000000000u64 {
            *dst.get_unchecked_mut(15) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(14) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(13) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(12) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(11) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(10) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(9) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(8) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(7) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(6) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            16
        }
        else if *v >= 100000000000000u64 {
            *dst.get_unchecked_mut(14) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(13) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(12) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(11) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(10) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(9) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(8) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(7) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(6) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            15
        } else if *v >= 10000000000000u64 {
            *dst.get_unchecked_mut(13) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(12) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(11) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(10) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(9) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(8) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(7) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(6) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            14
        } else if *v >= 1000000000000u64 {
            *dst.get_unchecked_mut(12) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(11) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(10) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(9) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(8) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(7) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(6) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            13
        } else if *v >= 100000000000u64 {
            *dst.get_unchecked_mut(11) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(10) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(9) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(8) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(7) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(6) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            12
        } else if *v >= 10000000000u64 {
            *dst.get_unchecked_mut(10) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(9) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(8) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(7) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(6) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            11
        }
        else if *v >= 1000000000u64 {
            *dst.get_unchecked_mut(9) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(8) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(7) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(6) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            10
        } else if *v >= 100000000u64 {
            *dst.get_unchecked_mut(8) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(7) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(6) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            9
        } else if *v >= 10000000u64 {
            *dst.get_unchecked_mut(7) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(6) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            8
        } else if *v >= 1000000u64 {
            *dst.get_unchecked_mut(6) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            7
        } else if *v >= 100000u64 {
            *dst.get_unchecked_mut(5) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            6
        } else if *v >= 10000u64 {
            *dst.get_unchecked_mut(4) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            5
        } else if *v >= 1000u64 {
            *dst.get_unchecked_mut(3) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            4
        } else if *v >= 100u64 {
            *dst.get_unchecked_mut(2) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            3
        } else if *v >= 10u64 {
            *dst.get_unchecked_mut(1) = b'0' + ((*v % 10) as u8);
            *v /= 10;
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            2
        } else {
            *dst.get_unchecked_mut(0) = b'0' + (*v as u8);
            1
        }
    }
}
