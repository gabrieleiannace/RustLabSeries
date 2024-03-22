

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
    let args = Args::parse();

    for _ in 0..args.repeat.clone(){
        for string in args.slug_in.clone(){
            slugify(string.as_str());
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::slugify;

    // i. conversione lettera accentata
    #[test]
    fn i() {
        let valor = "e";
        assert_eq!(valor, slugify("è"))
    }

    // ii. conversione lettera non accentata
    #[test]
    fn ii() {
        let valor = "p";
        assert_eq!(valor, slugify("p"))
    }

    // iii. conversione lettera non ammessa sconosciuta
    #[test]
    fn iii() {
        let valor = "-";
        assert_eq!(valor, slugify("|"))
    }

    // iv. conversione lettera accentata non compresa nella lista (es ῶ)
    #[test]
    fn iv() {
        let valor = "-";
        assert_eq!(valor, slugify("ῶ"))
    }

    // v. stringa con più di una parola separata da spazio
    #[test]
    fn v() {
        let valor = "test-string";
        assert_eq!(valor, slugify("test-string"));
    }

    // vi. stringa con caratteri accentati
    #[test]
    fn vi() {
        let valor = "stringa";
        assert_eq!(valor, slugify("strìngá"))
    }

    // vii. stringa vuota
    #[test]
    fn vii() {
        let valor = "";
        assert_eq!(valor, slugify(""))
    }

    // viii. stringa con più spazi consecutivi
    #[test]
    fn viii() {
        let valor = "space-strin-g";
        assert_eq!(valor, slugify("space  strin  g"))
    }

    // ix. stringa con con più caratteri non validi consecutivi
    #[test]
    fn ix() {
        let valor = "strin-g";
        assert_eq!(valor, slugify("strin||g"))
    }

    // x. stringa con solo caratteri non validi
    #[test]
    fn x() {
        let valor = "-";
        assert_eq!(valor, slugify("{{{{{{{{{{{{{{{{{"))
    }

    // xi. stringa con spazio alla fine
    #[test]
    fn xi() {
        let valor = "string";
        assert_eq!(valor, slugify("string "))
    }

    // xii. stringa con più caratteri non validi consecutivi alla fine
    #[test]
    fn xii() {
        let valor = "string";
        assert_eq!(valor, slugify("string{{{"))
    }
}