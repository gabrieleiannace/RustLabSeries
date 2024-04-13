use std::ops::Deref;
use part3::CircularBuffer;
use part3::complex_number::solution::ComplexNumber;



//inserire elemento e controllare dimensione buffer
#[test]
fn insert_and_dimension(){
    let mut c_buffer: CircularBuffer<ComplexNumber> = CircularBuffer::new(4);
    let _ = c_buffer.write(&ComplexNumber::new(1.0, 2.0));

    assert_eq!(c_buffer.size(), 1)
}

//inserire elemento, leggerlo e controllare che sia lo stesso
#[test]
fn write_and_read(){
    let mut c_buffer: CircularBuffer<ComplexNumber> = CircularBuffer::new(4);
    let num = ComplexNumber::new(7.0, 2.0);

    let _ = c_buffer.write(&num);
    let valore_letto = c_buffer.read().unwrap();
    assert_eq!(valore_letto, num);
}

// ripetere per n elementi e leggerli
#[test]
fn n_wirte_n_read(){
    let num1 = ComplexNumber::new(1.0, 2.0);
    let num2 = ComplexNumber::new(7.0, 2.0);
    let num3 = ComplexNumber::new(5.0, 2.0);

    let mut c_buff: CircularBuffer<ComplexNumber> = CircularBuffer::new(4);
    let _ = c_buff.write(&num1);
    let _ = c_buff.write(&num2);
    let _ = c_buff.write(&num3);

    let read1 = c_buff.read().unwrap();
    assert_eq!(read1, num1);
    let read2 = c_buff.read().unwrap();
    assert_eq!(read2, num2);
    let read3 = c_buff.read().unwrap();
    assert_eq!(read3, num3);
}

// controllare che head e tail ritornino correttamente a zero
#[test]
fn head_and_tail_reset(){
    let mut c_buff: CircularBuffer<ComplexNumber> = CircularBuffer::new(2);
    let num1 = ComplexNumber::new(1.0, 2.0);

    let _ = c_buff.write(&num1);
    let _ = c_buff.write(&num1);

    assert_eq!(c_buff.tail(), 0);
    let _ = c_buff.read();
    let _ = c_buff.read();
    assert_eq!(c_buff.head(), 0);
}

// leggere da buffer vuoto
#[test]
fn empty_read(){
    let mut c_buff: CircularBuffer<ComplexNumber> = CircularBuffer::new(4);
    match c_buff.read(){
        None => {assert!(true)}
        Some(_) => {assert!(false)}
    }
}

// scrivere su buffer pieno
#[test]
fn write_while_full(){
    let mut c_buff:CircularBuffer<ComplexNumber> = CircularBuffer::new(2);
    let num = ComplexNumber::new(1.0, 2.0);
    let _ = c_buff.write(&num);
    let _ = c_buff.write(&num);
    assert!(c_buff.write(&num).is_err())
}

// fare overwrite su buffer pieno (se non è pieno si deve comportare come write)
#[test]
fn overwrite_full(){
    let mut c_buff: CircularBuffer<ComplexNumber> = CircularBuffer::new(2);
    let num = ComplexNumber::new(1.0, 2.0);

    //Not full: Just a write
    c_buff.overwrite(&num);
    assert_eq!(c_buff.tail(), 1);
    c_buff.overwrite(&num);
    assert_eq!(c_buff.tail(), 0);

    //Full: overwrite last element
    let ow_item = ComplexNumber::new(11111.0, 22222.0);
    c_buff.overwrite(&ow_item);
    let _ = c_buff.read().unwrap();
    let read = c_buff.read().unwrap();
    assert_eq!(ow_item, read)
}

// rendere contiguo buffer non contiguo e controllare posizione di head e tail
#[test]
fn check_contigous(){
    let mut c_buff:CircularBuffer<ComplexNumber> = CircularBuffer::new(4);

    let _ = c_buff.write(&ComplexNumber::new(1.0,1.0));
    let _ = c_buff.write(&ComplexNumber::new(2.0,2.0));
    let _ = c_buff.write(&ComplexNumber::new(3.0,3.0));
    let _ = c_buff.write(&ComplexNumber::new(4.0,4.0));
    //Tail in 0 - Head in 0
    c_buff.read();
    c_buff.read();
    c_buff.read();
    //Tail in 0 - Head in 3
    let _ = c_buff.write(&ComplexNumber::new(5.0, 5.0));
    //Tail in 1 - Head in 3
    println!("Normal: {:?}", c_buff);
    c_buff.make_contiguos();
    println!("Make contigous: {:?}", c_buff);

    let mut cmp_buff:CircularBuffer<ComplexNumber> = CircularBuffer::new(4);
    let _ = cmp_buff.write(&ComplexNumber::new(4.0, 4.0));
    let _ = cmp_buff.write(&ComplexNumber::new(5.0, 5.0));
    println!("CMPBYF{:?}", cmp_buff);
    println!("CBUF{:?}", c_buff);
    assert_eq!(cmp_buff, c_buff);
}


#[derive(Debug)]
enum Tipi{
    I32(i32),
    ComplexNumber(ComplexNumber)
}

impl Default for Tipi{
    fn default() -> Self {
        Tipi::I32(i32::default())
    }
}

impl Clone for Tipi {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Tipi{ }

impl PartialEq for Tipi{
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl Eq for Tipi {  }

#[test]
#[ignore]
fn check_enum(){
    let num = Tipi::I32(2);
    let c_num = Tipi::ComplexNumber(ComplexNumber::new(1.0, 2.0));
    let mut buffer:CircularBuffer<Tipi> = CircularBuffer::new(2);

    let _ = buffer.write(&num);
    let _ = buffer.write(&c_num);

    println!("{:?}", buffer);
}

#[test]
#[ignore]
fn check_index(){
    let num = Tipi::I32(2);
    let c_num = Tipi::ComplexNumber(ComplexNumber::new(1.0, 2.0));
    let mut buffer:CircularBuffer<Tipi> = CircularBuffer::new(2);

    let _ = buffer.write(&num);
    let _ = buffer.write(&c_num);

    println!("{:?}", buffer[1]);
    println!("{:?}", c_num);
}

#[test]
#[ignore]
fn check_mut_index(){
    let num = Tipi::I32(2);
    let c_num = Tipi::ComplexNumber(ComplexNumber::new(1.0, 2.0));
    let mut buffer:CircularBuffer<Tipi> = CircularBuffer::new(2);

    let _ = buffer.write(&num);
    let _ = buffer.write(&c_num);

    println!("{:?}", buffer[0]);
    buffer[0] = Tipi::I32(100);
    println!("{:?}", buffer[0]);
}

#[test]
#[ignore]
fn check_deref(){
    let mut buffer:CircularBuffer<i32> = CircularBuffer::new(3);

    //Non contiguo
    let _ = buffer.write(&1);
    let _ = buffer.write(&2);
    let _ = buffer.write(&3);
    //Head 0 - Tail 0 = Buffer pieno
    let _ = buffer.read();
    let _ = buffer.read();
    //Head 2 - Tail 0
    let _ = buffer.write(&4);
    //Ora sono nella situazione [4, -, 3] Head 2 e Tail 1

    //let reference = &*buffer;
    //println!("{:?}", reference);

    //Contiguo
    buffer.clear();
    let _ = buffer.write(&1);
    let _ = buffer.write(&2);
    let reference = &*buffer;
    println!("{:?}", reference);


}

#[test]
fn check_try_deref(){
    let mut buffer:CircularBuffer<i32> = CircularBuffer::new(3);
    //Non contiguo
    let _ = buffer.write(&1);
    let _ = buffer.write(&2);
    let _ = buffer.write(&3);
    //Head 0 - Tail 0 = Buffer pieno
    let _ = buffer.read();
    let _ = buffer.read();
    //Head 2 - Tail 0
    let _ = buffer.write(&4);
    //Ora sono nella situazione [4, -, 3] Head 2 e Tail 1

    let mut reference = &mut *buffer;
    println!("{:?}", reference);
}
