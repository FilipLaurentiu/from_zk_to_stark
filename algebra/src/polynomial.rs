use std::fmt::{Display, Formatter};
use crate::finite_field::{FieldElement, FieldSize, FiniteField};
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone)]
struct Polynomial<'a> {
    /// c0 + c1*x^1 + c2*x^2 ...
    pub coefficients: Vec<FieldElement<'a>>,
    finite_field: &'a FiniteField,
}

impl<'a> Display for Polynomial<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            if coeff != self.finite_field.zero() {
                if !output.is_empty() {
                    output.push_str(" + ");
                }
                if i == 0 {
                    output.push_str(&coeff.to_string());
                } else {
                    let mut power = String::from("*x");
                    if i > 1 {
                        power.push_str(&format!("^{}", i));
                    }
                    output.push_str(&format!("{}{}", coeff, power));
                }
            }
        }
        write!(f, "{}", output)
    }
}

impl<'a> PartialEq for Polynomial<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.finite_field != other.finite_field {
            return false;
        }
        if self.coefficients.len() != other.coefficients.len() {
            return false;
        }

        for (index, element) in self.coefficients.iter().enumerate() {
            if element != &other.coefficients[index] {
                return false;
            }
        }
        true
    }
}

impl<'a> Polynomial<'a> {
    pub fn new(coefficients: Vec<FieldElement<'a>>, finite_field: &'a FiniteField) -> Self {
        Self {
            coefficients,
            finite_field,
        }
    }

    pub fn from_slice(coefficients: &'a [FieldSize], finite_field: &'a FiniteField) -> Self {
        let coeff_mod: Vec<FieldElement> = coefficients.iter().map(|x| finite_field.element(*x)).collect();
        Self {
            coefficients: coeff_mod.clone(),
            finite_field,
        }
    }

    pub fn degree(&self) -> FieldSize {
        if self.coefficients.is_empty() {
            return -1;
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

impl<'a> Add for Polynomial<'a> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field.prime, rhs.finite_field.prime, "Elements of different finite field");

        Self {
            coefficients: self.coefficients.iter().zip(rhs.coefficients.iter()).map(|(&a, &b)| a + b).collect(),
            finite_field: self.finite_field,
        }
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
        assert_eq!(self.finite_field.prime, rhs.finite_field.prime, "Elements of different finite field");
        self + rhs.neg()
    }
}

impl<'a> Div for Polynomial<'a> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<'a> Neg for Polynomial<'a> {
    type Output = Polynomial<'a>;

    fn neg(self) -> Self::Output {
        Self {
            coefficients: self.coefficients.iter().map(|x| x.neg()).collect(),
            finite_field: self.finite_field,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::finite_field::{FiniteField};
    use crate::polynomial::Polynomial;

    #[test]
    fn new_polynomial() {
        let finite_field = FiniteField::new(97, 1);
        let polynomial = Polynomial::from_slice(&[2, 7, 1, 4, 0, 5], &finite_field);
        assert_eq!(polynomial.degree(), 6);

        let polynomial = Polynomial::from_slice(&[2, 7, 1, 4, 0, 0], &finite_field);
        assert_eq!(polynomial.degree(), 4);
    }

    #[test]
    fn test_evaluate() {
        let finite_field = FiniteField::new(13, 1);
        let polynomial = Polynomial::from_slice(&[5, 2, 3], &finite_field);
        assert_eq!(polynomial.evaluate(&finite_field.element(3)), finite_field.element(12));
        assert_eq!(polynomial.evaluate(&finite_field.element(2)), finite_field.element(8));
    }

    #[test]
    fn test_add_polynomial() {
        let finite_field = FiniteField::new(97, 1);
        let polynomial1 = Polynomial::from_slice(&[2, 7, 1, 4, 0, 5], &finite_field);
        let polynomial2 = Polynomial::from_slice(&[1, 3, 4, 2, 7, 8], &finite_field);

        let expected = Polynomial::from_slice(&[3, 10, 5, 6, 7, 13], &finite_field);
        assert_eq!(polynomial1 + polynomial2, expected);
    }

    #[test]
    fn test_sub_polynomial() {
        let finite_field = FiniteField::new(97, 1);
        let polynomial1 = Polynomial::from_slice(&[2, 7, 7, 4, 8, 9], &finite_field);
        let polynomial2 = Polynomial::from_slice(&[1, 3, 4, 2, 3, 8], &finite_field);

        let expected = Polynomial::from_slice(&[1, 4, 3, 2, 5, 1], &finite_field);
        assert_eq!(polynomial1 - polynomial2, expected);
    }
}
