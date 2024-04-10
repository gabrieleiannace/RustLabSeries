use clap::{arg, Parser};

/// Slugify program from Exercise 1
#[derive(Parser, Debug)]
struct Args {
    //The input string
    #[arg(short, long, value_delimiter = ' ')]
    slug_in: Vec<String>,

    //Number of main's iteration
    #[arg(short, long, default_value_t = 1)]
    repeat: isize,

    //Print as much as possible
    #[arg(short, long)]
    verbose: bool,
}


//Costanti di conversione
const SUBS_I : &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";



//Funzione entry
fn slugify(s: &str) -> String {
    let mut slug = String::new();

    let string = s.to_string().to_lowercase();
    for char in string.chars(){
        //Check if the char is [a-z][0-9]
        if (char >= 'a' && char <= 'z') || (char >= '0' && char <= '9'){
            slug.push(char);
        }
        else {
            let new_char = conv(char);
            if !slug.is_empty() && slug.chars().last().unwrap() == '-' {continue;}
            else {slug.push(new_char);}
        }
    }

    if slug.len() > 1 && slug.chars().last().unwrap() == '-'{
        slug.pop();
    }

    println!("{}", slug);
    slug
}
//
// trait MySlug {
//     fn is_slug(&self) -> bool;
//     fn to_slug(&self) -> String;
//
// }
//
// impl MySlug for String {
//     fn is_slug(&self) -> bool {
//         let string_copy = slugify(self.as_str());
//         string_copy == self.to_lowercase()
//     }
//
//     fn to_slug(&self) -> String {
//         slugify(self)
//     }
// }
//
// impl MySlug for &str {
//     fn is_slug(&self) -> bool {
//         let string_copy = slugify(&self);
//         string_copy.as_str().eq(self.to_lowercase().as_str())
//     }
//
//     fn to_slug(&self) -> String {
//         slugify(self)
//     }
// }


//IMPLEMENTAZIONE GENERICA
trait Slug {
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

impl<T> Slug for T
    where
        T: AsRef<str>,
{
    fn is_slug(&self) -> bool {
        let slug = slugify(self.as_ref());
        slug.eq(self.as_ref())
    }

    fn to_slug(&self) -> String{
        slugify(self.as_ref())
    }
}


fn conv(c: char) -> char {
    let mut value = '-';
    for (iteration_count, accented_char) in SUBS_I.chars().enumerate() {
        if accented_char == c {
            value = SUBS_O.chars().nth(iteration_count).unwrap();
            break;
        }
    }
    value
}

fn main() {
    let s1 = String::from("Hello String");
    let s2 = "hello-slice";

    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true

    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();
    println!("s3:{} s4:{}", s3, s4); // stampa: s3:hello-string s4:hello-slice

}
