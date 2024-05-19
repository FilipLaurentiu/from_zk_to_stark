use rand::random;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use std::rc::Rc;

pub type FieldSize = i128;

#[derive(Debug, Clone)]
pub struct FieldElement {
    pub(crate) element: FieldSize,
    finite_field: Rc<FiniteField>,
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        if self.finite_field.prime != other.finite_field.prime {
            false
        } else {
            self.element % self.finite_field.prime == other.element % self.finite_field.prime
        }
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(
            Rc::ptr_eq(&self.finite_field, &rhs.finite_field),
            "Cannot add elements from different finite fields"
        );
        Self {
            element: self.element + rhs.element,
            finite_field: Rc::clone(&self.finite_field),
        }
        .abs()
    }
}

impl Add for &FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(
            Rc::ptr_eq(&self.finite_field, &rhs.finite_field),
            "Cannot add elements from different finite fields"
        );
        FieldElement {
            element: &self.element + &rhs.element,
            finite_field: self.finite_field.clone(),
        }
        .abs()
    }
}

impl AddAssign for FieldElement {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.finite_field, rhs.finite_field);
        *self = Self {
            element: &self.element + &rhs.element,
            finite_field: self.finite_field.clone(),
        }
        .abs();
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(
            Rc::ptr_eq(&self.finite_field, &rhs.finite_field),
            "Cannot sub elements from different finite fields"
        );
        Self {
            element: self.element - rhs.element,
            finite_field: Rc::clone(&self.finite_field),
        }
        .abs()
    }
}

impl Sub for &FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(
            Rc::ptr_eq(&self.finite_field, &rhs.finite_field),
            "Cannot sub elements from different finite fields"
        );
        FieldElement {
            element: &self.element - &rhs.element,
            finite_field: Rc::clone(&self.finite_field),
        }
        .abs()
    }
}

impl SubAssign for FieldElement {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            element: (self.element - rhs.element) % self.finite_field.prime,
            finite_field: self.finite_field.clone(),
        }
        .abs()
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field, rhs.finite_field);
        Self {
            element: self.abs().element * rhs.abs().element,
            finite_field: self.finite_field.clone(),
        }
        .abs()
    }
}

impl Mul for &FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field, rhs.finite_field);
        FieldElement {
            element: self.abs().element * rhs.abs().element,
            finite_field: self.finite_field.clone(),
        }
        .abs()
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

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

impl Div for &FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field, rhs.finite_field);
        assert_ne!(
            rhs,
            &self.finite_field.zero(),
            "Division by zero is not allowed"
        );

        self * &rhs.inverse()
    }
}

impl Neg for FieldElement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            element: self.finite_field.prime - self.element,
            finite_field: self.finite_field.clone(),
        }
    }
}

impl FieldElement {
    pub fn inverse(&self) -> Self {
        let xgcd = FiniteField::extended_euclidean(self.element, self.finite_field.prime);
        let inv = if xgcd.1.is_negative() {
            self.finite_field.prime + xgcd.1
        } else {
            xgcd.1.abs()
        };
        Self {
            element: inv % self.finite_field.prime,
            finite_field: self.finite_field.clone(),
        }
        .abs()
    }

    pub fn value(&self) -> FieldSize {
        self.abs().element
    }

    pub fn pow(&self, y: &FieldElement) -> FieldElement {
        let mut result = self.clone();
        for _i in 0..y.element {
            result = &result * &result;
        }
        result
    }

    pub fn abs(&self) -> FieldElement {
        let value = self.element.rem_euclid(self.finite_field.prime);
        if self.element.is_negative() {
            return FieldElement {
                element: value + &self.finite_field.prime,
                finite_field: self.finite_field.clone(),
            };
        }

        FieldElement {
            element: value,
            finite_field: self.finite_field.clone(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FiniteField {
    pub prime: FieldSize,
    pub generator: FieldSize,
}

impl FiniteField {
    pub fn new(prime: FieldSize, g: FieldSize) -> Self {
        assert_ne!(g, 0, "Invalid generator");
        Self {
            prime,
            generator: g,
        }
    }

    pub fn element(self: &Rc<Self>, value: FieldSize) -> FieldElement {
        FieldElement {
            element: value,
            finite_field: Rc::clone(self),
        }
    }

    pub fn zero(self: &Rc<Self>) -> FieldElement {
        self.element(0)
    }
    pub fn one(self: &Rc<Self>) -> FieldElement {
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

    pub fn random_element(self: &Rc<Self>) -> FieldElement {
        let random = random();
        self.element(random)
    }
}

#[cfg(test)]
mod tests {
    use super::FiniteField;
    use std::rc::Rc;

    #[test]
    fn test_finite_field() {
        let finite_field = Rc::new(FiniteField::new(97, 1));
        let field_element1 = finite_field.element(6);
        let field_element2 = finite_field.element(3);

        assert_eq!(&field_element1 + &field_element2, finite_field.element(9));
        assert_eq!(&field_element1 - &field_element2, finite_field.element(3));
        assert_eq!(field_element1 * field_element2, finite_field.element(18));
    }

    #[test]
    fn test_xeuclidean() {
        let prime = 97;
        let finite_field = Rc::new(FiniteField::new(prime, 1));

        for i in 1..prime {
            let result = FiniteField::extended_euclidean(i, prime);
            assert_eq!(result.0, 1); // no gcd

            let field_element = finite_field.element(1);
            let field_element_inv = field_element.inverse();
            assert_eq!(field_element * field_element_inv, finite_field.one());
        }
    }
}
