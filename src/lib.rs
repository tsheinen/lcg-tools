use itertools::izip;
use num::Integer;
use num_bigint::{BigInt, ToBigInt};

/// Rust's modulo operator is really remainder and not modular arithmetic so i have this
fn modulo(a: &BigInt, m: &BigInt) -> BigInt {
    ((a % m) + m) % m
}

fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let egcd = std::cmp::max(a, m).extended_gcd(&std::cmp::min(a.clone(), m.clone()));
    if egcd.gcd != num::one() {
        None
    } else {
        Some(modulo(&egcd.y, m))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct LCG {
    pub state: BigInt,
    // Seed
    pub a: BigInt,
    // Multiplier
    pub c: BigInt,
    // Increment
    pub m: BigInt, // Modulus
}

/// Tries to derive LCG parameters based on known values
/// This is probabilistic and may be wrong, especially for low number of values
/// https://tailcall.net/blog/cracking-randomness-lcgs/
pub fn crack_lcg(values: Vec<isize>) -> Option<LCG> {
    if values.len() < 3 {
        return None;
    }
    let diffs = izip!(values.clone(), values.clone().into_iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();
    let zeroes = izip!(
        diffs.clone(),
        diffs.clone().into_iter().skip(1),
        diffs.clone().into_iter().skip(2)
    )
    .map(|(a, b, c)| c * a - b * b)
    .collect::<Vec<_>>();
    let modulus = zeroes
        .iter()
        .fold(0isize, |sum, val| sum.gcd(val))
        .to_bigint()?;

    let multiplier = modulo(
        &((values[2] - values[1]).to_bigint()?
            * modinv(
                &(&values[1].to_bigint()? - &values[0].to_bigint()?),
                &modulus,
            )?),
        &modulus,
    );

    let increment = modulo(&(values[1] - values[0] * &multiplier), &modulus);
    Some(LCG {
        state: values.last()?.to_bigint()?,
        m: modulus,
        a: multiplier,
        c: increment,
    })
}

impl LCG {
    pub fn next(&mut self) -> BigInt {
        self.state =  modulo(&(&self.state * (&self.a) + (&self.c)), &self.m);
        self.state.clone()
    }

    pub fn prev(&mut self) -> Option<BigInt> {
        self.state = modulo(&(modinv(&self.a, &self.m)? * (&self.state - (&self.c))), &self.m);
        Some(self.state.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{crack_lcg, LCG};
    use num::ToPrimitive;
    use num_bigint::ToBigInt;

    #[test]
    fn it_generates_numbers_correctly_forward_and_backwards() {
        let mut rand = LCG {
            state: 32760.to_bigint().unwrap(),
            a: 5039.to_bigint().unwrap(),
            c: 76581.to_bigint().unwrap(),
            m: 479001599.to_bigint().unwrap(),
        };

        let mut forward = (0..10).map(|_| rand.next()).collect::<Vec<_>>();

        assert_eq!(
            forward,
            vec![
                165154221.to_bigint().unwrap(),
                186418737.to_bigint().unwrap(),
                41956685.to_bigint().unwrap(),
                180107137.to_bigint().unwrap(),
                330911418.to_bigint().unwrap(),
                58145764.to_bigint().unwrap(),
                326604388.to_bigint().unwrap(),
                389095148.to_bigint().unwrap(),
                96982646.to_bigint().unwrap(),
                113998795.to_bigint().unwrap()
            ]
        );
        forward.reverse();
        rand.next();
        assert_eq!(
            (0..10).filter_map(|_| rand.prev()).collect::<Vec<_>>(),
            forward
        );
    }

    #[test]
    fn it_cracks_lcg_correctly() {
        let mut rand = LCG {
            state: 32760.to_bigint().unwrap(),
            a: 5039.to_bigint().unwrap(),
            c: 0.to_bigint().unwrap(),
            m: 479001599.to_bigint().unwrap(),
        };

        let cracked_lcg = crack_lcg(
            (0..10)
                .map(|_| rand.next().to_isize().unwrap())
                .collect::<Vec<_>>(),
        )
        .unwrap();
        assert_eq!(rand, cracked_lcg);

    }
}