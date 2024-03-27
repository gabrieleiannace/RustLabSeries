mod args;

use std::fmt::{Display, Pointer};
use std::fs::{read_to_string};
use std::fs::write;
use std::time::SystemTime;
use clap::builder::TypedValueParser;
use crate::MulErr::{NegativeNumber, Overflow};
use clap::Parser;
use crate::args::{CommandsOption};

//-------------[ESERCIZI PROPEDEUTICI]-------------



//**********{PROPEDEUTICO 1}**************


//  [RISULTATO]:    Qui è messo in evidenza che bisogna codificare
//                  come stringa e non come byte poichè i caratteri
//                  possono non essere tutti UTF8
fn read_file(file_name: &str) -> (){

    //Implemento read_to_string e non read
    let read_operation = read_to_string(file_name);
    match read_operation {
        Ok(read_str) => {
            //========================================================
            //[ATTENZIONE]:     Questa parte commentata serve per notare
            //                   quanto detto nel testo dell'esercizio.

            // for byte in read_str{print!("{:02X} ", byte);}
            // print!("\n");
            // println!("c i   a   o \\n");
            //========================================================

            //Build 10times String
            let mut ten_times_string = String::new();
            for _ in 0..10{
                ten_times_string.push_str(read_str.as_str());
            }

            // //Write the string
            let write_operation = write(file_name, ten_times_string);
            match write_operation{
                Ok(_) => {println!("File successfully written!");}
                Err(error_msg) => {println!("[ERROR]: {}", error_msg)}
            }
        }
        Err(error_msg) => {
            println!("[ERROR]: {}", error_msg);
        }
    }
}

//**********{PROPEDEUTICO 2}**************
enum Errore{
    Simple(SystemTime),
    Complex(SystemTime, String)
}


fn print_error(e: Errore) -> (){
    match e{
        Errore::Simple(_) => {println!("[SIMPLE ERROR]");}
        Errore::Complex(_, error_str) => {println!("[COMPLEX ERROR]: {}", error_str);}
    }
}

//**********{PROPEDEUTICO 3}**************
pub enum MulErr{
    Overflow,
    NegativeNumber
}

pub fn mul(a: i32, b: i32) -> Result<u32, MulErr> {
    let res = a*b;
    return if res < 0 { Err(NegativeNumber) } else if res > u32::MAX as i32 { Err(Overflow) } else { Ok(res as u32) }
}

//**********{PROPEDEUTICO 4}**************

struct Node {
    name: String,
    size: u32,
    count: u32,
}
impl Node {

    //[ATTENZIONE]:     Con questi 3 metodi il comando Node::new("ciao".to_string()).size(10).count(5);
    //                  deve creare 3 Nodi diversi, il che è sicuramente molto oneroso. Perciò ho deciso
    //                  di utilizzare la versione con mut self
    pub fn new(name: String) -> Self {
        Node {name, size: 0, count: 0 }
    }

    // pub fn size(&self, size: u32) -> Self{
    //     Node{name: self.name.clone(), size, count: self.count}
    // }
    pub fn size(mut self, size: u32) -> Self{
        self.size = size;
        self
    }

    // pub fn count(self, count: u32) -> Self{
    //     Node{name: self.name, size: self.size, count}
    // }
    pub fn count(mut self, count: u32) -> Self{
        self.count = count;
        self
    }
    //-------------------------------------------------------------------------------------------------

    pub fn to_string(&self) -> String{
        let mut string = String::new();
        string.push_str("name:");
        string.push_str(self.name.as_str());
        string.push_str(" size:");
        string.push_str(self.size.to_string().as_str());
        string.push_str(" count:");
        string.push_str(self.count.to_string().as_str());
        string
    }

    pub fn grow(mut self){
        self.size += 1;
    }

    pub fn inc(mut self){
        self.count += 1;
    }

}
//----------------------------------------------------------

//---------------------[ESERCIZIO 2]---------------------

const BSIZE: usize = 20;
const BOAT_NUM: usize = 4;
pub struct Board {
    boats: [u8; BOAT_NUM],                    //Numero di navi
    data: [[u8; BSIZE]; BSIZE],               //Matrice
}

pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}

pub enum Boat {
    Vertical(usize),
    Horizontal(usize)
}

impl Board{
    /** crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8]) -> Board {
        let mut boat_array: [u8; BOAT_NUM] = [0; BOAT_NUM];
        for i in 0..boats.len(){
            boat_array[i] = boats[i];
        }
        Board{ boats: boat_array, data: [[0;BSIZE]; BSIZE] }
    }

    /*  crea una board a partire da una stringa che rappresenta tutto
        il contenuto del file board.txt                                 */
    pub fn from(s: String)->Board {
        // Split in righe
        let mut rows_string: Vec<&str> = s.split('\n').collect();
        let first_row: Vec<&str> = rows_string[0].split(" ").collect();
        let mut matrix_rows = &rows_string[1..];

        let mut boat_array: [u8; BOAT_NUM] = [0; BOAT_NUM];
        for i in 0..BOAT_NUM {
            if let Ok(num) = first_row[i].parse::<u8>() {
                boat_array[i] = num;
            }
        }

        let mut matrix_data: [[u8; BSIZE]; BSIZE] = [[0; BSIZE]; BSIZE];
        for i in 0..BSIZE {
            for (counter, c) in matrix_rows[i].chars().enumerate() {
                println!("{:?}{:?}", counter, c);
                if c == ' ' { matrix_data[i][counter] = 0u8} else { matrix_data[i][counter] = 1u8}
            }
        }

        Board{ boats: boat_array, data: matrix_data }
    }

    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String {
        // Riga delle navi
        let mut board_string = String::new();
        for i in 0..BOAT_NUM-1 {
            board_string.push_str(self.boats[i].to_string().as_str());
            board_string.push_str(" ");
        }
        board_string.push_str(self.boats[BOAT_NUM-1].to_string().as_str());
        board_string.push_str("\n");

        //Riga della matrice
        for riga in self.data {
            for val in riga {
                if val == 0u8 {board_string.push_str(" ")} else {board_string.push_str("B")}
            }
            board_string.push_str("\n");
        }
        board_string.pop();
        board_string
    }
}

//----------------------------------------------------------

fn main() {
    //read_file("src/test.txt");

    // let node = Node::new("ciao".to_string()).size(10).count(5);
    // println!("{}", node.to_string());

    let board = Board::new(&[2, 3, 4, 5]);
    let pippo = board.to_string();
    let new_board = Board::from(pippo);

    let args = CommandsOption::parse();
    println!("{:?}", args);
    match args {
        CommandsOption::CreateBoard(_) => {}
        CommandsOption::AddBoat(_) => {}
    }
}
