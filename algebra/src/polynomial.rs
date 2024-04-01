use crate::finite_field::{FieldElement, FieldSize, FiniteField};
use std::ops::{Add, Sub, Mul, Div};

struct Polynomial<'a, 'b> {
    /// c0 + c1*x^1 + c2*x^2 ...
    coefficients: Vec<FieldElement<'b>>,
    finite_field: &'a FiniteField,
}

impl<'a, 'b> Polynomial<'a, 'b> {
    pub fn new(coefficients: Vec<FieldElement<'b>>, finite_field: &'a FiniteField) -> Self {
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
            if *s != self.finite_field.zero() {
                let coeff_len = self.coefficients.len();
                return (coeff_len - index) as FieldSize;
            }
        }
        0
    }

    pub fn evaluate(&self, x: &'a FieldElement) -> FieldElement {
        if self.coefficients.is_empty() {
            self.finite_field.zero()
        } else {
            let mut result = self.finite_field.zero();
            let mut pow = self.finite_field.one();
            for element in self.coefficients.iter() {
                let term = *element * pow;
                result = result + term;
                pow = pow * *x;
            }
            result
        }
    }
}

impl<'a, 'b> Add for Polynomial<'a, 'b> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<'a, 'b> Mul for Polynomial<'a, 'b> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<'a, 'b> Sub for Polynomial<'a, 'b> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<'a, 'b> Div for Polynomial<'a, 'b> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::finite_field::{FieldSize, FiniteField};
    use crate::polynomial::Polynomial;

    #[test]
    fn new_polynomial() {
        let finite_field = FiniteField::new(97 as FieldSize, 1);
        let polynomial = Polynomial::new([2, 7, 1, 4, 0, 5].to_vec(), &finite_field);
        assert_eq!(polynomial.degree(), 6);

        let polynomial = Polynomial::new([2, 7, 1, 4, 0, 0].to_vec(), &finite_field);
        assert_eq!(polynomial.degree(), 4);
    }
}
