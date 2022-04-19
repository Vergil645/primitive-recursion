use logic::*;

#[allow(dead_code)]
fn add_two(x: [u32; 1]) -> u32 {
    s(n, [n])(x)
}

#[allow(dead_code)]
fn sum(x: [u32; 2]) -> u32 {
    r(u::<1, 1>, s(n, [u::<3, 3>]))(x)
}

#[allow(dead_code)]
fn one(x: [u32; 1]) -> u32 {
    s(n, [z])(x)
}

// 1.a
#[allow(dead_code)]
fn lim_sub(x: [u32; 2]) -> u32 {
    r(u::<1, 1>, s(r(z, u::<3, 2>), [u::<3, 3>, u::<3, 3>]))(x)
}

// 1.b
#[allow(dead_code)]
fn mul(x: [u32; 2]) -> u32 {
    r(z, s(sum, [u::<3, 1>, u::<3, 3>]))(x)
}

// 1.c
#[allow(dead_code)]
fn power(x: [u32; 2]) -> u32 {
    r(one, s(mul, [u::<3, 1>, u::<3, 3>]))(x)
}

// 1.d
// (x, y) -> if x > y { 1 } else { 0 }
#[allow(dead_code)]
fn is_greater(x: [u32; 2]) -> u32 {
    s(r(z, s(one, [u::<3, 1>])), [u::<2, 1>, lim_sub])(x)
}

// (x, y) -> if x <= y { 1 } else { 0 }
#[allow(dead_code)]
fn is_lower_or_eq(x: [u32; 2]) -> u32 {
    // (_, _) -> 1
    fn one2(x: [u32; 2]) -> u32 {
        s(one, [u::<2, 1>])(x)
    }
    s(lim_sub, [one2, is_greater])(x)
}

#[allow(dead_code)]
fn div(x: [u32; 2]) -> u32 {
    fn n2(x: [u32; 3]) -> u32 {
        s(n, [u::<3, 2>])(x)
    }
    fn n3(x: [u32; 3]) -> u32 {
        s(n, [u::<3, 3>])(x)
    }
    fn mul_1_n3(x: [u32; 3]) -> u32 {
        s(mul, [u::<3, 1>, n3])(x)
    }
    fn try_add(x: [u32; 3]) -> u32 {
        s(is_lower_or_eq, [mul_1_n3, n2])(x)
    }
    s(r(z, s(sum, [u::<3, 3>, try_add])), [u::<2, 2>, u::<2, 1>])(x)
}

// 1.e
// (x, y) -> x % y
#[allow(dead_code)]
fn rem(x: [u32; 2]) -> u32 {
    fn mul_div(x: [u32; 2]) -> u32 {
        s(mul, [u::<2, 2>, div])(x)
    }
    s(lim_sub, [u::<2, 1>, mul_div])(x)
}

// 1.f
#[allow(dead_code)]
fn is_prime(x: [u32; 1]) -> u32 {
    // (_, _, _) -> 0
    fn z3(x: [u32; 3]) -> u32 {
        s(z, [u::<3, 3>])(x)
    }
    // (_, y, _) -> y + 1
    fn n2(x: [u32; 3]) -> u32 {
        s(n, [u::<3, 2>])(x)
    }
    // (x, y, _) -> x % (y + 1)
    fn rem_1_n2(x: [u32; 3]) -> u32 {
        s(rem, [u::<3, 1>, n2])(x)
    }
    // (x, y, _) -> if x % (y + 1) == 0 { 1 } else { 0 }
    fn try_add(x: [u32; 3]) -> u32 {
        s(is_lower_or_eq, [rem_1_n2, z3])(x)
    }
    // (x) -> number of divisors x
    fn divisors_count(x: [u32; 1]) -> u32 {
        fn rec(x: [u32; 2]) -> u32 {
            r(z, s(sum, [u::<3, 3>, try_add]))(x)
        }
        s(rec, [u::<1, 1>, u::<1, 1>])(x)
    }
    // (_) -> 2
    fn two(x: [u32; 1]) -> u32 {
        s(n, [one])(x)
    }
    // (x) -> if number of divisors x <= 2 { 1 } else { 0 }
    fn is_divisors_count_lower_or_eq_two(x: [u32; 1]) -> u32 {
        s(is_lower_or_eq, [divisors_count, two])(x)
    }
    // (x) -> if x > 1 { 1 } else { 0 }
    fn is_greater_than_one(x: [u32; 1]) -> u32 {
        s(is_greater, [u::<1, 1>, one])(x)
    }
    s(mul, [is_greater_than_one, is_divisors_count_lower_or_eq_two])(x)
}

// 1.h
#[allow(dead_code)]
fn if_or_else(x: [u32; 3]) -> u32 {
    fn mul_1_2(x: [u32; 3]) -> u32 {
        s(mul, [u::<3, 1>, u::<3, 2>])(x)
    }
    fn mul_not_1_3(x: [u32; 3]) -> u32 {
        fn one3(x: [u32; 3]) -> u32 {
            s(one, [u::<3, 1>])(x)
        }
        fn not_1(x: [u32; 3]) -> u32 {
            s(lim_sub, [one3, u::<3, 1>])(x)
        }
        s(mul, [not_1, u::<3, 3>])(x)
    }
    s(sum, [mul_1_2, mul_not_1_3])(x)
}

#[allow(dead_code)]
fn gcd(x: [u32; 2]) -> u32 {
    fn z2(x: [u32; 2]) -> u32 {
        s(z, [u::<2, 2>])(x)
    }
    fn n3(x: [u32; 4]) -> u32 {
        s(n, [u::<4, 3>])(x)
    }
    // (x, y) -> 1 if x % y == 0 else 0
    fn is_divisor(x: [u32; 2]) -> u32 {
        s(is_lower_or_eq, [rem, z2])(x)
    }
    fn is_divisor_1_n3(x: [u32; 4]) -> u32 {
        s(is_divisor, [u::<4, 1>, n3])(x)
    }
    fn is_divisor_2_n3(x: [u32; 4]) -> u32 {
        s(is_divisor, [u::<4, 2>, n3])(x)
    }
    fn n3_can_be_gcd(x: [u32; 4]) -> u32 {
        s(mul, [is_divisor_1_n3, is_divisor_2_n3])(x)
    }
    fn upd_res(x: [u32; 4]) -> u32 {
        s(if_or_else, [n3_can_be_gcd, n3, u::<4, 4>])(x)
    }
    fn rec(x: [u32; 3]) -> u32 {
        r(s(one, [u::<2, 1>]), upd_res)(x)
    }
    s(rec, [u::<2, 1>, u::<2, 2>, u::<2, 2>])(x)
}

// 1.i
//noinspection SpellCheckingInspection
#[allow(dead_code)]
fn plog(x: [u32; 2]) -> u32 {
    // (_, _) -> 0
    fn z2(x: [u32; 2]) -> u32 {
        s(z, [u::<2, 2>])(x)
    }
    // (_, _, _, w) -> w + 1
    fn n4(x: [u32; 4]) -> u32 {
        s(n, [u::<4, 4>])(x)
    }
    // (_, y, _, w) -> y^(w + 1)
    fn power_2_n4(x: [u32; 4]) -> u32 {
        s(power, [u::<4, 2>, n4])(x)
    }
    // (x, y) -> if x % y == 0 { 1 } else { 0 }
    fn is_divisor(x: [u32; 2]) -> u32 {
        s(is_lower_or_eq, [rem, z2])(x)
    }
    // (x, y, _, w) -> if x % y^(w + 1) == 0 { 1 } else { 0 }
    fn try_add(x: [u32; 4]) -> u32 {
        s(is_divisor, [u::<4, 1>, power_2_n4])(x)
    }
    s(r(z2, s(sum, [u::<4, 4>, try_add])), [u::<2, 1>, u::<2, 2>, u::<2, 1>])(x)
}

// 1.j
#[allow(dead_code)]
fn head(x: [u32; 1]) -> u32 {
    fn two(x: [u32; 1]) -> u32 {
        s(n, [one])(x)
    }
    s(plog, [u::<1, 1>, two])(x)
}

fn main() {
    println!("Hello, World!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(0 + 2, add_two([0]));
        assert_eq!(3 + 2, add_two([3]));
        assert_eq!(123121 + 2, add_two([123121]));
    }

    #[test]
    fn test_sum() {
        assert_eq!(1 + 0, sum([1, 0]));
        assert_eq!(9 + 3, sum([9, 3]));
        assert_eq!(2123 + 707, sum([2123, 707]));
    }

    #[test]
    fn test_lim_sub() {
        assert_eq!(4, lim_sub([7, 3]));
        assert_eq!(0, lim_sub([2, 15]));
        assert_eq!(1234 - 123, lim_sub([1234, 123]));
    }

    #[test]
    fn test_mul() {
        assert_eq!(2 * 2, mul([2, 2]));
        assert_eq!(243 * 0, mul([243, 0]));
        assert_eq!(137 * 21, mul([137, 21]));
    }

    #[test]
    fn test_pow() {
        assert_eq!(8, power([2, 3]));
        assert_eq!(1, power([1231244, 0]));
        assert_eq!(243, power([3, 5]));
    }

    #[test]
    fn test_is_greater() {
        assert_eq!(0, is_greater([5, 5]));
        assert_eq!(0, is_greater([0, 9]));
        assert_eq!(1, is_greater([7, 3]));
        assert_eq!(1, is_greater([3, 0]));
    }

    #[test]
    fn test_is_lower_or_eq() {
        assert_eq!(1, is_lower_or_eq([5, 5]));
        assert_eq!(1, is_lower_or_eq([0, 9]));
        assert_eq!(0, is_lower_or_eq([7, 3]));
        assert_eq!(0, is_lower_or_eq([3, 0]));
    }

    #[test]
    fn test_div() {
        assert_eq!(0, div([0, 134]));
        assert_eq!(0, div([5, 7]));
        assert_eq!(157 / 24, div([157, 24]));
        assert_eq!(36 / 9, div([36, 9]));
    }

    #[test]
    fn test_div_mod() {
        assert_eq!(0, rem([0, 134]));
        assert_eq!(5, rem([5, 7]));
        assert_eq!(157 % 24, rem([157, 24]));
        assert_eq!(36 % 9, rem([36, 9]));
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(0, is_prime([0]));
        assert_eq!(0, is_prime([1]));
        assert_eq!(1, is_prime([2]));
        assert_eq!(1, is_prime([3]));
        assert_eq!(0, is_prime([4]));
        assert_eq!(1, is_prime([5]));
        assert_eq!(0, is_prime([6]));
        assert_eq!(1, is_prime([7]));
        assert_eq!(1, is_prime([23]));
        assert_eq!(0, is_prime([35]));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(1, gcd([2, 3]));
        assert_eq!(1, gcd([7, 5]));
        assert_eq!(2, gcd([2, 6]));
        assert_eq!(6, gcd([24, 18]));
        assert_eq!(3, gcd([21, 9]));
        assert_eq!(14, gcd([14, 28]));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_plog() {
        assert_eq!(0, plog([5, 2]));
        assert_eq!(1, plog([6, 2]));
        assert_eq!(2, plog([20, 2]));
        assert_eq!(3, plog([54, 3]));
        assert_eq!(4, plog([16, 2]));
    }
}
