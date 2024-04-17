use rand::random;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

pub type FieldSize = i128;

#[derive(Debug, Copy, Clone)]
pub struct FieldElement<'a> {
    pub(crate) element: FieldSize,
    finite_field: &'a FiniteField,
}

impl<'a> PartialEq for FieldElement<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.finite_field.prime != other.finite_field.prime {
            false
        } else {
            self.element % self.finite_field.prime == other.element % self.finite_field.prime
        }
    }
}

impl<'a> Display for FieldElement<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

impl<'a> Add for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field, rhs.finite_field);
        Self {
            element: self.element + rhs.element,
            finite_field: self.finite_field,
        }
        .abs()
    }
}

impl<'a> Add for &FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field, rhs.finite_field);
        FieldElement {
            element: self.element + rhs.element,
            finite_field: self.finite_field,
        }
        .abs()
    }
}

impl<'a> AddAssign for FieldElement<'a> {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.finite_field, rhs.finite_field);
        *self = Self {
            element: self.element + rhs.element,
            finite_field: self.finite_field,
        }
        .abs();
    }
}

impl<'a> Sub for FieldElement<'a> {
    type Output = FieldElement<'a>;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field, rhs.finite_field);
        let res = self.abs() - rhs.abs();
        res.abs()
    }
}

impl<'a> SubAssign for FieldElement<'a> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            element: (self.element - rhs.element) % self.finite_field.prime,
            finite_field: self.finite_field,
        }
        .abs()
    }
}

impl<'a> Mul for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field, rhs.finite_field);
        Self {
            element: self.abs().element * rhs.abs().element,
            finite_field: self.finite_field,
        }
        .abs()
    }
}

impl<'a> Mul for &FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field, rhs.finite_field);
        FieldElement {
            element: self.abs().element * rhs.abs().element,
            finite_field: self.finite_field,
        }
        .abs()
    }
}

impl<'a> Div for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field, rhs.finite_field);
        assert_ne!(
            rhs,
            self.finite_field.zero(),
            "Division by zero is not allowed"
        );
        self * rhs.inverse()
    }
}

impl<'a> Neg for FieldElement<'a> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            element: self.finite_field.prime - self.element,
            finite_field: self.finite_field,
        }
    }
}

impl<'a> FieldElement<'a> {
    pub fn inverse(&self) -> Self {
        let xgcd = FiniteField::extended_euclidean(self.element, self.finite_field.prime);
        let inv = if xgcd.1.is_negative() {
            self.finite_field.prime + xgcd.1
        } else {
            xgcd.1.abs()
        };
        Self {
            element: inv % self.finite_field.prime,
            finite_field: self.finite_field,
        }
        .abs()
    }

    pub fn value(&self) -> FieldSize {
        self.abs().element
    }

    pub fn pow(&self, y: &FieldElement) -> FieldElement {
        let mut result = *self;
        for _i in 0..y.element {
            result = result * result;
        }
        result
    }

    pub fn abs(&self) -> FieldElement<'a> {
        let value = self.element.rem_euclid(self.finite_field.prime);
        if self.element.is_negative() {
            return FieldElement {
                element: value + self.finite_field.prime,
                finite_field: self.finite_field,
            };
        }

        FieldElement {
            element: value,
            finite_field: self.finite_field,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct FiniteField {
    pub prime: FieldSize,
    pub generator: FieldSize,
}

impl FiniteField {
    pub fn new(prime: FieldSize, G: FieldSize) -> Self {
        assert_ne!(G, 0, "Invalid generator");
        Self {
            prime,
            generator: G,
        }
    }

    pub fn element(&self, value: FieldSize) -> FieldElement {
        FieldElement {
            element: value,
            finite_field: self,
        }
    }

    pub fn zero(&self) -> FieldElement {
        self.element(0)
    }
    pub fn one(&self) -> FieldElement {
        self.element(1)
    }

    pub fn extended_euclidean(a: FieldSize, b: FieldSize) -> (FieldSize, FieldSize, FieldSize) {
        if a == 0 {
            return (b, 0, 1);
        }

        let (gcd, x1, y1) = Self::extended_euclidean(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;
        (gcd, x, y) // ax + by = gcd(a, b)
    }

    pub fn random_element(&self) -> FieldElement {
        let random = random();
        self.element(random)
    }
}

#[cfg(test)]
mod tests {
    use super::FiniteField;

    #[test]
    fn test_finite_field() {
        let finite_field = FiniteField::new(97, 1);
        let field_element1 = finite_field.element(6);
        let field_element2 = finite_field.element(3);

        assert_eq!(field_element1 + field_element2, finite_field.element(9));
        assert_eq!(field_element1 - field_element2, finite_field.element(3));
        assert_eq!(field_element1 * field_element2, finite_field.element(18));
    }

    #[test]
    fn test_xeuclidean() {
        let prime = 97;
        let finite_field = FiniteField::new(prime, 1);

        for i in 1..prime {
            let result = FiniteField::extended_euclidean(i, prime);
            assert_eq!(result.0, 1); // no gcd

            let field_element = finite_field.element(1);
            let field_element_inv = field_element.inverse();
            assert_eq!(field_element * field_element_inv, finite_field.one());
        }
    }
}
