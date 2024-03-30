use crate::finite_field::{FieldSize, FiniteField};
use std::ops::{Add, Sub, Mul, Div};

struct Polynomial<'a> {
    /// c1 + c2*x + c3*x^2 ...
    coefficients: Vec<FieldSize>,
    finite_field: &'a FiniteField,
}

impl<'a> Polynomial<'a> {
    pub fn new(coefficients: Vec<FieldSize>, finite_field: &'a FiniteField) -> Self {
        Self {
            coefficients,
            finite_field,
        }
    }

    pub fn degree(&self) -> FieldSize {
        if self.coefficients.is_empty() {
            return 0;
        }
        for (index, s) in self.coefficients.iter().rev().enumerate() {
            if *s != 0 {
                let coeff_len = self.coefficients.len();
                return (coeff_len - index) as u16;
            }
        }
        0
    }
}

impl<'a> Add for Polynomial<'a> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<'a> Mul for Polynomial<'a> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl<'a> Sub for Polynomial<'a> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<'a> Div for Polynomial<'a> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use crate::finite_field::{FiniteField};
    use crate::polynomial::Polynomial;

    #[test]
    fn new_polynomial() {
        let finite_field = FiniteField::new(97u16);
        let polynomial = Polynomial::new([2, 7, 1, 4, 0, 5].to_vec(), &finite_field);
        assert_eq!(polynomial.degree(), 6);

        let polynomial = Polynomial::new([2, 7, 1, 4, 0, 0].to_vec(), &finite_field);
        assert_eq!(polynomial.degree(), 4);
    }
}
