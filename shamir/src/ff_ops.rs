pub const PRIME: u64 = 997; // 2^64 - 5 // Miller-Rabin primality test

pub fn mod_add(a: u64, b: u64) -> u64 {
    let sum = a as u128 + b as u128;
    (sum % PRIME as u128) as u64
}

pub fn mod_sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { PRIME - (b - a) }
}

pub fn mod_mul(a: u64, b: u64) -> u64 {
    let product = (a as u128) * (b as u128);
    (product % PRIME as u128) as u64
}

// Eg, 2^3
// 1/ result = 1, base = 2, exp = 3
// exp is odd, result = result * base = 1 * 2 = 2
// base = base * base = 2 * 2 = 4
// exp = exp >> 1 = 3 >> 1 = 1
// 2/ exp is odd, result = result * base = 2 * 4 = 8
// base = base * base = 4 * 4 = 16
// exp = exp >> 1 = 1 >> 1 = 0
// 3/ exp is 0, return result = 8

// Formula:
// Given exponent e, e can be written as binary number:
// e = b0 * 2^0 + b1 * 2^1 + b2 * 2^2 + ... + bn * 2^n
// Then, a^e can be written as:
// a^e = a^(b0 * 2^0 + b1 * 2^1 + b2 * 2^2 +... + bn * 2^n)
// = a^(b0 * 2^0) * a^(b1 * 2^1) * a^(b2 * 2^2) *... * a^(bn * 2^n)
// = a^(b0) * a^(b0 * 2) * a^(b1 * 2) * a^(b2 * 2^2) *... * a^(bn * 2^n)
// = a^(b0) * a^(b1) * a^(b2) *... * a^(bn)

// the evaluation looks like:
// only collect when the last bit of exp is 1 (odd number) then accumulate the result
// the base is cummulative multiplication of base
// the exp is right shift by 1 bit (divide by 2)
// 2^3 = 2^(2^1) * 2^(2^1), 3 = b11
pub fn mod_pow(base: u64, exp: u64) -> u64 {
    if exp == 0 {
        return 1;
    }
    let mut result = 1u64;
    let mut base = base;
    let mut exp = exp;
    while exp > 0 {
        // check if exp is odd
        if exp & 1 == 1 {
            result = mod_mul(result, base);
        }
        base = mod_mul(base, base);
        exp >>= 1;
    }
    result
}

pub fn mod_inv(a: u64) -> u64 {
    // Fermat's little theorem
    // a^p ≡ a (mod p).
    // a^(p-1) ≡ 1 mod p
    // If we multiply both sides by a^(-1), we get:
    // a^(p-2) ≡ a^(-1) mod p
    mod_pow(a, PRIME - 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 3), 8);
        assert_eq!(mod_pow(2, 4), 16);
        assert_eq!(mod_pow(2, 5), 32);
        assert_eq!(mod_pow(5, 0), 1);
    }

    #[test]
    fn test_mod_inv() {
        assert_eq!(mod_inv(2), mod_pow(2, PRIME - 2));
    }
}
