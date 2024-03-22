Esercitazione 1

Esercizio 1

Obiettivi:

> ● gestione stringhe e slice
>
> ● differenza tra “caratteri” (char) e “byte” (u8) in Rust ● lettura
> parametri da command line

Con il termine “slug” si intende una stringa convertita in formato
leggibile, composta solo dai caratteri \[a-z\]\[0-9\]-.

Nella stringa originale i caratteri non ammissibili vengono convertiti
seguendo queste regole: ● tutti i caratteri accentati riconosciuti
vengono convertiti nell’equivalente non accentato ● tutto viene
convertito in minuscolo

> ● ogni altro carattere rimanente che non sia in \[a-z\]\[0-9\] viene
> convertito in “-” ● due “-” consecutivi non sono ammessi, solo il
> primo viene tenuto
>
> ● un “-” finale non è ammesso a meno che non sia l’unico carattere
> nella stringa

L’obiettivo dell’esercizio è fare un funzione “slugify” che converta una
stringa generica in uno slug.

PASSI PER LA SOLUZIONE

> 1\. Creare con cargo un nuovo package chiamato **slugify** e, in
> main.rs, definire la funzione
>
> fn slugify(s: &str) -\> String {}
>
> 2\. Per convertire le lettere accentate definire una funzione che
> esegua la conversione:
>
> fn conv(c: char) -\> char {}
>
> Conv restituisce il carattere c se è uno ammesso, la lettera non
> accentata corrispondente se viene trovata, o “-” negli altri casi
>
> All’interno usare questa tabella di conversione, dove il carattere
> nella posizione i (<u>come carattere)</u> in SUBS_I corrisponde al
> carattere nella posizione i in SUBS_O:
>
> const SUBS_I : &str =
> "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
> const SUBS_O: &str =
> "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzz
> z";
>
> **ATTENZIONE**: SUBS_I e SUBS_O essendo degli slice di stringa non
> possono essere indicizzati direttamente con \[pos\] (**perché?**),
> tutto quello che si può assumere è che il carattere corrispondente
> alla posizione i-esima di SUBS_I è nella stessa
>
> posizione in SUBS_O.
>
> 3\. Scrivere degli unit test per le funzioni create
>
> (riferimento:
> <u><https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html>)</u>
>
> a\. creare una sezione in main.rs per ospitare i test \#\[cfg(test)\]
>
> mod tests {
>
> use super::\*; }
>
> b\. i test sono funzioni così definite all’interno: \#\[test\]
>
> fn my_first_test() {
>
> // valore = preparazione test assert_eq!(valore, valore_atteso)
>
> }
>
> c\. definire almeno questi test:
>
> i\. conversione lettera accentata
>
> ii\. conversione lettera non accentata
>
> iii\. conversione lettera non ammessa sconosciuta
>
> iv\. conversione lettera accentata non compresa nella lista (es ῶ) v.
> stringa con più di una parola separata da spazio
>
> vi\. stringa con caratteri accentati vii. stringa vuota
>
> viii\. stringa con più spazi consecutivi
>
> ix\. stringa con con più caratteri non validi consecutivi x. stringa
> con solo caratteri non validi
>
> xi\. stringa con spazio alla fine
>
> xii\. stringa con più caratteri non validi consecutivi alla fine d. i
> test possono venire lanciati con
>
> cargo test
>
> 4\. Rendere la funzione invocabile dalla command line
>
> Nel main() leggere una sequenza di parole come argomento da command
> line, invocare la funzione e stampare il risultato.
>
> Esempio:
>
> cargo run -- “Questo che slug sarà???”
>
> risultato: “slug: questo-che-slug-sara“
>
> (Attenzione: doppio “--” dopo run, serve per separare i parametri di
> cargo da quelli del comando; la stringa tra apici doppi viene letta
> come unico parametro e non come n parametri. Non usate copia incolla
> dal testo, perché l’editor usato per comporre il doc sostituisce gli
> apici doppi con altri caratteri)
>
> Per il parsing degli argomenti di command line si suggerisce di
> inserire nel progetto la libreria clap:
> [<u>https://docs.rs/clap/latest/clap/</u>](https://docs.rs/clap/latest/clap/)
>
> Clap è una libreria per leggere e validare i parametri da command line
> e ha due modalità di funzionamento, “derive” e “build”. La prima
> permette di definire i parametri da leggere mediante gli attributi di
> una struct, la seconda è più flessibile ma meno immediata da usare e
> consente di costruire i parametri da leggere in modo imperativo con
> una serie di istruzioni.
>
> In questo progetto useremo la modalità “derive” (tutorial completo con
> esempi a questo indirizzo
> <u><https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html>)</u>
>
> a\. aggiungere la libreria cargo, editando il file cargo.toml od
> eseguendo: cargo add clap --features derive
>
> verrà aggiunto a cargo.toml: \[dependencies\]
>
> clap = { version = "4.5.3", features = \["derive"\] }
>
> b\. in questo esempio non abbiamo opzioni con nome (es --verbose) e
> quindi leggiamo tutti i valori passati come una sola stringa, quindi
> definire una struct Args derivata da clap::Parser in questo modo:
>
> \#\[derive(Parser, Debug)\] struct Args {
>
> // input string slug_in: String,
>
> }
>
> c\. la sintassi di clap è semplice: si definisce un attributo (in
> questo caso slug_in) per ogni parametro che si vuole leggere. Clap
> cercherà di fare la conversione automatica dei parametri passati al
> tipo indicato, mentre darà errore nel caso in cui non sia possibile
>
> d\. invocando nel main Args::parse() si effettua il parsing della
> command line e restituisce un oggetto di tipo Args con i parametri
> richiesti; clap aggiunge automaticamente l’opzione di --help e la
> gestione degli errori
>
> e\. provare ad invocare cargo run -- --help per verificare che clap
> funzioni in modo corretto
>
> f\. a questo punto l’invocazione cargo run -- “Questo sarà uno slug?”
> dovrebbe inserire in args.slug_intutta la stringa “Questo sarà uno
> slug!” (notare i doppi apici intorno alla stringa, perché vogliamo che
> tutte le parole siano interpretate come un unico parametro)
>
> g\. per impratichirsi con clap aggiungere due parametri opzionali con
> nome --repeat=n e --verbose. Repeat è di tipo intero, verbose boolean.
>
> Usare l’annotazione \#\[arg()\] come negli esempi al link indicato per
> impostare il comportamento del parser.
>
> Inoltre verificare, invocando il comando, che le conversioni, vengano
> lette in modo corretto, in particolare quella ad intero di --repeat
>
> h\. come potrei modificare il tipo di slug_inper leggere tutte le
> parole della stringa da convertire in un vettore di stringhe anziché
> in un’unica stringa?