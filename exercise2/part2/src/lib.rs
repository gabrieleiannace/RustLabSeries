pub mod solution{
    use std::cmp::Ordering;
    use std::fmt::{Formatter};

    #[derive(Copy, Clone, Debug)]
    pub struct ComplexNumber{
        real: f64,
        imag: f64
    }

    impl ComplexNumber{
        pub fn new(real: f64, imag: f64) -> Self{
            ComplexNumber{real, imag}
        }
        pub fn real(&self) -> f64 {self.real}
        pub fn imag(&self) -> f64 {self.imag}

        pub fn from_real(real: f64) -> Self {
            ComplexNumber::new(real, 0f64)
        }

        pub fn to_tuple(&self) -> (f64, f64) {(self.real, self.imag)}

        pub fn into(self) -> f64 {
            if self.imag != 0.0 {panic!("Panic: Image part not null!")}
            self.real
        }
    }

    impl std::ops::Add<Self> for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self{
                real: self.real + rhs.real(),
                imag: self.imag + rhs.imag(),
            }
        }
    }

    impl std::ops::Add<f64> for ComplexNumber{
        type Output = Self;

        fn add(self, rhs: f64) -> Self::Output {
            Self{
                real: self.real + rhs,
                imag: self.imag
            }
        }
    }

    impl std::ops::AddAssign<Self> for ComplexNumber {
        fn add_assign(&mut self, rhs: Self) {
            self.real += rhs.real();
            self.imag += rhs.imag();
        }
    }

    impl std::ops::Add<&Self> for ComplexNumber{
        type Output = Self;

        fn add(self, rhs: &Self) -> Self::Output {
            Self{
                real: self.real + rhs.real(),
                imag: self.imag + rhs.imag()
            }
        }
    }

    impl std::ops::Add<Self> for &ComplexNumber{
        type Output = ComplexNumber;
        fn add(self, rhs: Self) -> ComplexNumber {
            ComplexNumber::new(self.real+rhs.real(), self.imag+rhs.imag())
        }
    }

    //Potrei anche derivarlo volendo
    impl Default for ComplexNumber{
        fn default() -> Self {
            Self{real: 0.0, imag: 0.0}
        }
    }

    // impl Into<f64> for ComplexNumber{
    //     fn into(self) -> f64 {
    //         if self.imag != 0.0 {panic!("Panic: Image part not null!")}
    //         self.real
    //     }
    // }

    // impl TryInto<f64> for ComplexNumber {
    //     type Error = CustomError;
    //
    //     fn try_into(self) -> Result<f64, CustomError> {
    //         if self.imag != 0.0 {
    //             Err(CustomError{info: "Errore: Parte immaginaria non nulla".to_string() })
    //         } else {
    //             Ok(self.real)
    //         }
    //     }
    // }

    impl TryFrom<ComplexNumber> for f64{
        type Error = CustomError;

        fn try_from(value: ComplexNumber) -> Result<Self, Self::Error> {
            if value.imag != 0.0 {
                Err(CustomError{info: "Errore: Parte immaginaria non nulla".to_string()})
            }
            else {
                Ok(value.real)
            }
        }
    }

    impl PartialEq<Self> for ComplexNumber {
        fn eq(&self, other: &Self) -> bool {
            if self.real == other.real && self.imag == other.imag {true} else {false}
        }
    }

    impl Eq for ComplexNumber{}

    impl std::cmp::PartialOrd for ComplexNumber{
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            //Assuming |z1| = sqrt(real² + imag²)
            let z1:f64 = (self.real.powf(2f64) + self.imag.powf(2f64)).sqrt();
            //Assuming |z2| = sqrt(real² + imag²)
            let z2:f64 = (other.real.powf(2f64) + other.imag.powf(2f64)).sqrt();

            //if |z1| > |z2| => self > other else self < other
            if z1.is_nan() || z2.is_nan() {None} else {Some(z1.total_cmp(&z2))}
        }
    }

    impl std::cmp::Ord for ComplexNumber{
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(&other).unwrap()
        }
    }



    //Mi scrivo il mio errore:
    #[derive(Debug)]
    pub struct CustomError{
        info: String,
    }

    impl std::fmt::Display for CustomError{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.info)
        }
    }

    impl std::error::Error for CustomError{
        fn description(&self) -> &str {
            self.info.as_str()
        }
    }

}
