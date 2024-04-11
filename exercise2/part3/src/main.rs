pub struct CircularBuffer<T> where T: Default{
    vec: Vec<T>,
    head: usize,
    tail: usize
}

impl<T: Default + Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self { vec: vec![T::default(); capacity], head: 0, tail: 0 }
    }
}

pub struct ComplexNumber{
    real: f64,
    imag: f64
}


fn main() {
    let complex = ComplexNumber{real: 2.0, imag: 5.2};

}
