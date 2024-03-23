use std::fs::{read, read_to_string};
use std::fs::write;
use std::time::SystemTime;


//-------------[ESERCIZI PROPEDEUTICI]-------------



//**********{PARTE 1}**************


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

//**********{PARTE 2}**************
enum Error{
    Simple(SystemTime),
    Complex(SystemTime, String)
}

fn print_error(e: Error) -> (){
    
}

//**********{PARTE 3}**************


//----------------------------------------------------------

fn main() {
    read_file("src/test.txt");
}
