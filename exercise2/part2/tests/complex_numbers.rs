use std::collections::VecDeque;
use part2::solution::ComplexNumber;

// for this execise see https://doc.rust-lang.org/beta/std/primitive.f64.html
// you can find examples for all the traits that must be implemented

#[test]
pub fn test_create() {
    let a = ComplexNumber::new(1.0, 2.0);
    assert_eq!(a.real(), 1.0);
    assert_eq!(a.imag(), 2.0);
}

#[test]
pub fn test_create_from_real() {
    let a = ComplexNumber::from_real(10.0);
    assert_eq!(a.real(), 10.0);
    assert_eq!(a.imag(), 0.0);
}

#[test]
pub fn test_add() {
    // implement Add trait
    // rember to set: type Output = Self;
    // see: https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md#add--addassign

    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(1.0, 2.0);

    let c = a + b;

    assert_eq!(c.to_tuple(), (2.0, 4.0));
}

#[test]
pub fn test_add_with_real() {
    // set RHS (rihgt hand side) type for Add!!! It's default value is Self, but it can be changed to anything
    let a = ComplexNumber::new(1.0, 2.0);
    let b = a + 10.0;

    assert_eq!(b.to_tuple(), (11.0, 2.0))
}

#[test]
pub fn test_inc_add() {
    let mut a = ComplexNumber::new(1.0, 2.0);
    a +=  ComplexNumber::new(2.0, 4.0);

    assert_eq!(a.to_tuple(), (3.0, 6.0))
}

#[test]
pub fn test_add_with_reference() {
    // references for Rust are new types: you must define the trait for them as RHS
    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(1.0, 2.0);

    let c = a + &b;

    assert_eq!(c.to_tuple(), (2.0, 4.0))
}


#[test]
pub fn test_add_reference_with_reference() {
    // write yourself the test and adjust traits
    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(1.0, 2.0);

    let c = &a + &b;
    assert_eq!(c.to_tuple(), (2.0, 4.0))
    //assert!(true);
}


#[test]
pub fn test_enable_copy(){
    // why this code won't compile? Read carefully the error message
    // what do we need to do to make it work?
    let a = ComplexNumber::new(1.0, 2.0);

    let b = a + a;

    assert_eq!(b.to_tuple(), (2.0, 4.0));
}


#[test]
pub fn test_default_values() {
    // If we want to create an array of complex numbers we need to initialize values with something
    // Arrays can't be not initialized in Rust
    let array: [ComplexNumber; 10] = Default::default();

    for el in array.as_slice() {
        assert_eq!(el.to_tuple(), (0.0, 0.0));
    }
}


#[test]
pub fn test_convert_into_real() {
    let a = ComplexNumber::from_real(1.0);
    let b: f64 = a.into();

    assert_eq!(b, 1.0);
}

#[test]
pub fn test_panic_when_impossible_to_convert_to_real() {
    // we can convert into a real only if imag is 0
    let a = ComplexNumber::new(1.0, 2.0);

    let result = std::panic::catch_unwind(|| {
        let _: f64 = a.into();
    });

    assert!(result.is_err());
}

#[test]
pub fn test_try_into_f64() {
    // write trait and a test for the Trait TryInto for converting into f64
    // the test must check both success and error conditions

    // Test success condition
    let complex_success = ComplexNumber::from_real(1.0);
    assert_eq!(1.0, complex_success.try_into().unwrap());

    // Test failure condition
    let complex_failure = ComplexNumber::new(1.0, 1.0);
    assert_eq!(true, <ComplexNumber as TryInto<f64>>::try_into(complex_failure).is_err());
}

#[test]
pub fn test_try_form_f64() {
    // write a trait allowing let complex = f64.into()
    // and write test
    let complex: ComplexNumber = ComplexNumber::new(1.0, 0.0);
    let number = f64::try_from(complex).unwrap();
    assert_eq!(number, 1.0)
}


#[test]
pub fn test_comparison() {
    let c = ComplexNumber::new(3.0, 6.0);
    let mut v = vec![ComplexNumber::new(1.0, 2.0), ComplexNumber::new(2.0, 4.0), c];

    v.retain(|el| *el == c);

    assert_eq!(v.len(), 1);
}


#[test]
pub fn test_sorting() {
    // for sorting we can use the modulus of a complex number
    //https://www.cuemath.com/algebra/modulus-of-complex-number/
    // if |a| > |b| than a > b

    // Be careful: f64 does not implement Ord since NaN != NaN and you can't
    // use cmp from f64 to implement Ord for ComplexNumber
    // However f64 has total_cmp which produces total ordering
    // https://doc.rust-lang.org/beta/std/primitive.f64.html#method.total_cmp

    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(2.0, 4.0);
    let c = ComplexNumber::new(3.0, 6.0);
    let mut v = vec![c, b, a];

    v.sort();

    assert_eq!(v[0], a);
    assert_eq!(v[1], b);
    assert_eq!(v[2], c);
}

#[test]
pub fn test_as_ref() {
    // implement AsRef<f64> for ComplexNumber
    // allow a mutable ref to real part as &f64

    let a = ComplexNumber::new(1.0, 2.0);
    let r = a.as_ref();

    assert_eq!(*r, 1.0);
}

#[test]
pub fn test_as_mut() {
    // implement AsMut<f64> for ComplexNumber
    // allow a mutable ref to real part as &mut f64

    let mut a = ComplexNumber::new(1.0, 2.0);
    let r = a.as_mut();

    *r = 10.0;

    assert_eq!(a.real(), 10.0);
}

#[test]
pub fn test_hash_with_hash_map() {
    // in order to use comeplex numbers in a hash map we need to implement Hash
    // https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md#hash
    // we can use the to_bits method from f64 to get a u64 representation of the float
    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(2.0, 4.0);
    let c: ComplexNumber = 3f64.into();

    let mut map = std::collections::HashMap::new();

    // first insert must return None: not present
    match map.insert(a, b) {
        None => assert!(true),
        Some(_) => assert!(false)
    };

    // try ro replace value with c
    match map.insert(a, c) {
        None => assert!(false),
        Some(x) => assert_eq!(x.to_tuple(), (2.0, 4.0)) // should return the old value, b
    };


}


#[test]
pub fn test_deque() {
    // implement VecDeque for ComplexNumber
    // 1. create a VecDeque with capacity 10
    let mut vec:VecDeque<ComplexNumber> = VecDeque::with_capacity(10);
    // 2. push 10 values in the deque
    let value1 = ComplexNumber::new(2.0, 0.2);
    vec.push_back(value1);
    let value2 = ComplexNumber::new(1.0, 1.2);
    vec.push_back(value2);
    let value3 = ComplexNumber::new(7.0, 3.2);
    vec.push_back(value3);
    let value4 = ComplexNumber::new(12.0, 2.2);
    vec.push_back(value4);
    let value5 = ComplexNumber::new(3.0, 5.2);
    vec.push_back(value5);
    let value6 = ComplexNumber::new(22.0, 4.2);
    vec.push_back(value6);
    let value7 = ComplexNumber::new(24.0, 3.2);
    vec.push_back(value7);
    let value8 = ComplexNumber::new(19.0, 4.2);
    vec.push_back(value8);
    let value9 = ComplexNumber::new(10.0, 5.2);
    vec.push_back(value9);
    let value10 = ComplexNumber::new(55.0, 6.2);
    vec.push_back(value10);
    // 4. find the index of a value with binary_search: it works only if the deque is sorted!!!
    let result = vec.binary_search(&ComplexNumber::new(10.0, 5.2));
    // 5. check the result: it should be meaningless
    println!("{:?}", result);
    // 6. sort the deque and check afain the result of binary_search, now it should be meaningful
    let mut vec2:Vec<ComplexNumber> = vec.into();
    vec2.sort();
    vec = vec2.into();
    let result = vec.binary_search(&ComplexNumber::new(10.0, 5.2));
    assert_eq!(4, result.unwrap());
}
