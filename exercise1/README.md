# Esercizio 1

## Obiettivi:
- Gestione stringhe e slice
- Differenza tra "caratteri" (char) e "byte" (u8) in Rust
- Lettura parametri da command line

Con il termine "slug" si intende una stringa convertita in formato leggibile, composta solo dai caratteri `[a-z][0-9]-`.

Nella stringa originale i caratteri non ammissibili vengono convertiti seguendo queste regole:
- Tutti i caratteri accentati riconosciuti vengono convertiti nell'equivalente non accentato
- Tutto viene convertito in minuscolo
- Ogni altro carattere rimanente che non sia in `[a-z][0-9]` viene convertito in "-"
- Due "-" consecutivi non sono ammessi, solo il primo viene tenuto
- Un "-" finale non è ammesso a meno che non sia l'unico carattere nella stringa

L'obiettivo dell'esercizio è fare una funzione "slugify" che converta una stringa generica in uno slug.

## Passi per la soluzione:
1. Creare con cargo un nuovo package chiamato `slugify` e, in `main.rs`, definire la funzione `slugify`.

```rust
fn slugify(s: &str) -> String {}
```
2. Per convertire le lettere accentate definire una funzione che esegua la conversione:

```rust
fn conv(c: char) -> char {}
```
Conv restituisce il carattere c se è uno ammesso, la lettera non accentata
corrispondente se viene trovata, o “-” negli altri casi
All’interno usare questa tabella di conversione, dove il carattere nella posizione i
(come carattere) in SUBS_I corrisponde al carattere nella posizione i in SUBS_O:

```rust
const SUBS_I : &str =
    "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str =
    "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzz
z";
```
ATTENZIONE: SUBS_I e SUBS_O essendo degli slice di stringa non possono
essere indicizzati direttamente con [pos] (perché?), tutto quello che si può assumere
è che il carattere corrispondente alla posizione i-esima di SUBS_I è nella stessa
posizione in SUBS_O.

3. Scrivere degli unit test per le funzioni create
   (riferimento: https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
- a. creare una sezione in main.rs per ospitare i test:

```rust
#[cfg(test)]
mod tests {
    use super::*;
}
```

- b. i test sono funzioni così definite all’interno:
```rust
#[test]
fn my_first_test() {
// valore = preparazione test
    assert_eq!(valore, valore_atteso)
}
```

- c. definire almeno questi test:
  i. conversione lettera accentata
  ii. conversione lettera non accentata
  iii. conversione lettera non ammessa sconosciuta
  iv. conversione lettera accentata non compresa nella lista (es ῶ)
  v. stringa con più di una parola separata da spazio
  vi. stringa con caratteri accentati
  vii. stringa vuota
  viii. stringa con più spazi consecutivi
  ix. stringa con con più caratteri non validi consecutivi
  x. stringa con solo caratteri non validi
  xi. stringa con spazio alla fine
  xii. stringa con più caratteri non validi consecutivi alla fine
