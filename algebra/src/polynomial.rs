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


impl<'a> Add for Polynomial<'a> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field.prime, rhs.finite_field.prime, "Elements of different finite field");

        let shortest_length = self.coefficients.len().min(rhs.coefficients.len());

        // Using iterators and zip
        let mut result: Vec<_> = self.coefficients.iter().zip(rhs.coefficients.iter()).take(shortest_length)
            .map(|(&a, &b)| a + b)
            .chain(self.coefficients.iter().skip(shortest_length).copied())
            .chain(rhs.coefficients.iter().skip(shortest_length).copied())
            .collect();
        let zero = &self.finite_field.zero();
        while let Some(element) = result.last() {
            if element == zero {
                result.pop();
            } else {
                break;
            }
        }
        Self {
            coefficients: result,
            finite_field: self.finite_field,
        }
    }
}

impl<'a> Add for &Polynomial<'a> {
    type Output = Polynomial<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field.prime, rhs.finite_field.prime, "Elements of different finite field");

        let shortest_length = self.coefficients.len().min(rhs.coefficients.len());

        // Using iterators and zip
        let mut result: Vec<_> = self.coefficients.iter().zip(rhs.coefficients.iter()).take(shortest_length)
            .map(|(&a, &b)| a + b)
            .chain(self.coefficients.iter().skip(shortest_length).copied())
            .chain(rhs.coefficients.iter().skip(shortest_length).copied())
            .collect();
        let zero = &self.finite_field.zero();
        while let Some(element) = result.last() {
            if element == zero {
                result.pop();
            } else {
                break;
            }
        }
        Polynomial::new(result, self.finite_field)
    }
}

impl<'a> Mul for Polynomial<'a> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result_coefficients = vec![self.finite_field.zero(); self.coefficients.len() + rhs.coefficients.len() - 1];

        for (i, &coef1) in self.coefficients.iter().enumerate() {
            for (j, &coef2) in rhs.coefficients.iter().enumerate() {
                result_coefficients[i + j] += coef1 * coef2;
            }
        }

        Self {
            coefficients: result_coefficients,
            finite_field: self.finite_field,
        }
    }
}

impl<'a> Mul for &Polynomial<'a> {
    type Output = Polynomial<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result_coefficients = vec![self.finite_field.zero(); self.coefficients.len() + rhs.coefficients.len() - 1];

        for (i, &coef1) in self.coefficients.iter().enumerate() {
            for (j, &coef2) in rhs.coefficients.iter().enumerate() {
                result_coefficients[i + j] += coef1 * coef2;
            }
        }

        Polynomial {
            coefficients: result_coefficients,
            finite_field: self.finite_field,
        }
    }
}

impl<'a> Sub for Polynomial<'a> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field.prime, rhs.finite_field.prime, "Elements of different finite field");
        self + rhs.neg()
    }
}

impl<'a> Sub for &Polynomial<'a> {
    type Output = Polynomial<'a>;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.finite_field.prime, rhs.finite_field.prime, "Elements of different finite field");
        let res = self + &rhs.neg();
        res
    }
}

impl<'a> Div for Polynomial<'a> {
    type Output = (Polynomial<'a>, Polynomial<'a>);
    fn div(self, rhs: Polynomial<'a>) -> Self::Output {
        let mut dividend = self.clone();
        let mut result_coefficients = vec![self.finite_field.zero(); dividend.coefficients.len() - rhs.coefficients.len() + 1];

        let leading_coeff_index_rhs = rhs.leading_coefficient_index();
        let leading_coeff_rhs = rhs.coefficients[leading_coeff_index_rhs].element;


        while dividend.coefficients.len() >= rhs.coefficients.len() {
            let leading_coeff_index_dividend = dividend.coefficients.len() - 1;
            let leading_coeff_dividend = dividend.coefficients[leading_coeff_index_dividend].element;

            let leading_quotient = leading_coeff_dividend / leading_coeff_rhs;
            let leading_quotient_index = dividend.coefficients.len() - rhs.coefficients.len();
            result_coefficients[leading_quotient_index].element = leading_quotient;

            let mut temp_quotient = vec![self.finite_field.zero(); leading_quotient_index + 1];
            temp_quotient[leading_quotient_index].element = leading_quotient;

            let temp_quotient_polynomial = Polynomial::new(temp_quotient, self.finite_field);
            dividend = dividend - (&temp_quotient_polynomial * &rhs);
        }

        (Self { // quotient
            coefficients: result_coefficients,
            finite_field: self.finite_field,
        },
         dividend // remainder
        )
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

impl<'a> Neg for &Polynomial<'a> {
    type Output = Polynomial<'a>;

    fn neg(self) -> Self::Output {
        Polynomial {
            coefficients: self.coefficients.iter().map(|x| x.neg()).collect(),
            finite_field: self.finite_field,
        }
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

    pub fn scalar_mul(self, scalar: FieldElement<'a>) -> Self {
        Self {
            coefficients: self.coefficients.iter().map(|&x| x * scalar).collect(),
            finite_field: self.finite_field,
        }
    }

    pub fn scalar_div(self, scalar: FieldElement<'a>) -> Self {
        Self {
            coefficients: self.coefficients.iter().map(|&x| x / scalar).collect(),
            finite_field: self.finite_field,
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

    fn leading_coefficient_index(&self) -> usize {
        for i in (0..self.coefficients.len()).rev() {
            if self.coefficients[i] != self.finite_field.zero() {
                return i;
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
                result += *element * pow;
                pow = pow * *x;
            }
            result
        }
    }

    pub fn lagrange_interpolation(points: &[(FieldElement<'a>, FieldElement<'a>)], finite_field: &'a FiniteField) -> Self {
        let x = &Polynomial::from_slice(&[0, 1], finite_field);
        let mut acc = Polynomial::new(Vec::new(), finite_field);
        for (i, i_element) in points.iter().enumerate() {
            let mut value = Polynomial::new([i_element.1].to_vec(), finite_field);
            for (j, j_element) in points.iter().enumerate() {
                if i == j { continue; }
                let basis = (x - &Polynomial::new([j_element.0].to_vec(), finite_field)) * Polynomial::new([(i_element.0 - j_element.0).inverse()].to_vec(), finite_field);
                value = value * basis;
            }
            acc = acc + value;
        }
        acc
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


        let polynomial1 = Polynomial::from_slice(&[2, 7, 7, 4, 8, 9], &finite_field);
        let polynomial2 = Polynomial::from_slice(&[1, 3, 4, 2], &finite_field);
        let expected = Polynomial::from_slice(&[1, 4, 3, 2, 8, 9], &finite_field);
        assert_eq!(polynomial1 - polynomial2, expected);

        let polynomial1 = Polynomial::from_slice(&[2, 7, 7], &finite_field);
        let polynomial2 = Polynomial::from_slice(&[1, 3, 7], &finite_field);
        let expected = Polynomial::from_slice(&[1, 4], &finite_field);
        assert_eq!(polynomial1 - polynomial2, expected);
    }

    #[test]
    fn test_leading_coefficient_index() {
        let finite_field = FiniteField::new(97, 1);
        let polynomial1 = Polynomial::from_slice(&[2, 7, 7], &finite_field);
        let leading_coeff_index = polynomial1.leading_coefficient_index();
        assert_eq!(leading_coeff_index, 2);
    }

    #[test]
    fn test_mul_polynomial() {
        let finite_field = FiniteField::new(97, 1);
        let polynomial1 = Polynomial::from_slice(&[2, 7, 7], &finite_field);
        let polynomial2 = Polynomial::from_slice(&[3, 5], &finite_field);

        assert_eq!(&polynomial1 * &polynomial2, Polynomial::from_slice(&[6, 31, 56, 35], &finite_field));
    }

    #[test]
    fn test_div_polynomial() {
        let finite_field = FiniteField::new(97, 1);
        let polynomial1 = Polynomial::from_slice(&[74, 79, 81, 1], &finite_field);
        let polynomial2 = Polynomial::from_slice(&[94, 1], &finite_field);
        let division = polynomial1 / polynomial2;
        assert_eq!(division.0, Polynomial::from_slice(&[40, 84, 1], &finite_field));
    }

    #[test]
    fn lagrange_interpolation() {
        let finite_field = FiniteField::new(97, 1);

        let points = [
            (finite_field.element(1), finite_field.element(7)),
            (finite_field.element(2), finite_field.element(6)),
            (finite_field.element(3), finite_field.element(8))
        ];

        let p = Polynomial::lagrange_interpolation(&points, &finite_field);
        let expected = Polynomial::from_slice(&[11, 43, 50], &finite_field);
        assert_eq!(&p, &expected);

        assert_eq!(p.evaluate(&points[0].0), points[0].1);
        assert_eq!(p.evaluate(&points[1].0), points[1].1);
        assert_eq!(p.evaluate(&points[2].0), points[2].1);
    }
}
