pub mod complex_number;

use std::fmt::Error;

#[derive(Debug)]
pub struct CircularBuffer<T> where T: Default{
    vec: Vec<T>,
    head: usize,
    tail: usize,
    size: usize
}

impl<T: Default + Copy > CircularBuffer<T> {
    pub fn new(capacity: usize) ->  Self {
        Self { vec: vec![T::default(); capacity], head: 0, tail: 0, size: 0 }
    }

    pub fn head(&self) -> usize{
        self.head
    }

    pub fn tail(&self) -> usize{
        self.tail
    }

    pub fn write(&mut self, item: &T) -> Result<T, Error>{
        if self.size == self.vec.len() {Err(Error::default())}
        else {
            self.vec[self.tail] = *item;
            self.tail = (self.tail+1)%self.vec.len();
            self.size += 1;
            Ok(*item)
        }
    }

    pub fn read(&mut self) -> Option<T>{
        if self.size == 0 {return None}
        self.size -= 1;
        let value = self.vec[self.head];
        self.head = (self.head+1)%self.vec.len();
        Some(value)
    }

    pub fn clear(&mut self){
        self.vec.clear();
        self.size = 0;
        self.head = 0;
        self.tail = 0;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn overwrite(&mut self, item: &T) {
        //Se non è pieno faccio una write normale
        if self.size != self.vec.len() {let _ = self.write(item);}
        else{
            //Mi brucio il valore più vecchio
            self.vec[self.tail] = *item;
            self.tail = (self.tail+1)%self.vec.len();
            self.head = (self.head+1)%self.vec.len();
        }
    }

    /*
        Quando tail < head (tail ha raggiunto la fine ed è ritornato a zero) i valori nel buffer sono
        spezzati in due segmenti separati, una parte all’inizio, una parte alla fine dell’array, con lo
        spazio vuoto in mezzo. Non è quindi contiguo e make_contiguos() riorganizza il buffer,
        copiando in cima all’array tutti gli elementi mantenendo l’ordine di lettura, rendendolo così di
        nuovo contiguo.
     */
    pub fn make_contiguos(&mut self) {
        if self.tail < self.head {
            let mut new_c_buff:CircularBuffer<T> = CircularBuffer::new(self.vec.len());
            'inner_loop: loop{
                match self.read(){
                    None => {
                        break 'inner_loop;
                    }
                    Some(item) => {let _ = new_c_buff.write(&item);}
                }
            }
            self.vec = new_c_buff.vec;
            self.head = new_c_buff.head;
            self.tail = new_c_buff.tail;
            self.size = new_c_buff.size;
        }
    }
}

impl<T: PartialEq + Default> PartialEq for CircularBuffer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vec.eq(&other.vec) && self.head == other.head && self.tail == other.tail && self.size == other.size
    }
}
impl<T> std::cmp::Eq for CircularBuffer<T> where T: Default + Eq {  }


