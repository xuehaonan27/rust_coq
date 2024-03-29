use std::{
    ops::{Add, Mul, Sub},
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq)]
enum Peano {
    O,            // Zero is natural number.
    S(Rc<Peano>), // Successor of a natural number is a natural number.
}

impl Peano {
    fn pred(self) -> Self {
        match self {
            Peano::O => Peano::O,
            Peano::S(p) => p.as_ref().clone(),
        }
    }

    fn succ(self) -> Self {
        Peano::S(Rc::new(self))
    }
}

impl From<usize> for Peano {
    fn from(value: usize) -> Self {
        match value {
            0 => Peano::O,
            n => Peano::S(Rc::new((n - 1).into())),
        }
    }
}

impl Into<usize> for Peano {
    fn into(self) -> usize {
        match self {
            Peano::O => 0,
            Peano::S(p) => 1 + Into::<usize>::into(p.as_ref().clone()),
        }
    }
}

impl Add for Peano {
    type Output = Peano;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Peano::O, Peano::O) => Peano::O,
            (Peano::O, rhs) => rhs,
            (lhs, Peano::O) => lhs,
            (lhs, rhs) => lhs.succ() + rhs.pred(),
        }
    }
}

impl Sub for Peano {
    type Output = Peano;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // This is the most tricky situation. Because there's no negative
            // number in Peano numerals (which is a kind of definition of
            // natural numbers) so we just return zero arbitrarily when there's
            // "0 - n"(n != 0).
            (Peano::O, _) => Peano::O,
            (lhs, Peano::O) => lhs,
            (Peano::S(l), Peano::S(r)) => l.as_ref().clone() - r.as_ref().clone(),
        }
    }
}

impl Mul for Peano {
    type Output = Peano;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Peano::O => Peano::O,
            Peano::S(n) => n.as_ref().clone() * rhs.clone() + rhs,
        }
    }
}

#[cfg(test)]
mod test_peano {
    use super::*;

    #[test]
    pub fn check_one_add_one() {
        let peano_1: Peano = 1.into();
        let peano_2: Peano = 2.into();
        assert_eq!(peano_1.clone() + peano_1, peano_2);
    }

    #[test]
    pub fn check_three_minus_one() {
        let peano_1: Peano = 1.into();
        let peano_2: Peano = 2.into();
        let peano_3: Peano = 3.into();
        assert_eq!(peano_3 - peano_1, peano_2);
    }

    #[test]
    pub fn check_three_mult_four() {
        let peano_3: Peano = 3.into();
        let peano_4: Peano = 4.into();
        let peano_12: Peano = 12.into();
        assert_eq!(peano_3 * peano_4, peano_12);
    }
}
