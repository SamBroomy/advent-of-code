/// Math utilities for Day 14
pub mod crt {
    /// Extended Greatest Common Divisor algorithm
    pub fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
        if a == 0 {
            return (b, 0, 1);
        }
        let (gcd, x1, y1) = extended_gcd(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;
        (gcd, x, y)
    }

    /// Modular inverse using Extended GCD
    pub fn mod_inverse(a: i32, m: i32) -> Option<i32> {
        let (gcd, x, _) = extended_gcd(a, m);
        if gcd != 1 {
            return None;
        }
        Some((x % m + m) % m)
    }

    /// Chinese Remainder Theorem implementation
    pub fn solve(a1: i32, n1: i32, a2: i32, n2: i32) -> Option<i32> {
        let (gcd, _, _) = extended_gcd(n1, n2);
        if gcd != 1 {
            return None;
        }

        let n = n1 * n2;
        let n1_part = n2;
        let n2_part = n1;

        let x1 = mod_inverse(n1_part, n1)?;
        let x2 = mod_inverse(n2_part, n2)?;

        let mut result = (a1 * n1_part * x1 + a2 * n2_part * x2) % n;
        if result < 0 {
            result += n;
        }

        Some(result)
    }
}

/// Statistics utilities for Day 14
pub mod stats {
    /// Calculate variance for a list of positions
    pub fn variance(positions: &[i32]) -> f64 {
        let n = positions.len() as f64;
        let mean = positions.iter().sum::<i32>() as f64 / n;
        positions
            .iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>()
    }
}
