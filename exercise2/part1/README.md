# Esercizio 1 - Tratto MySlug

## Obiettivi:
- polimorfismo in Rust
- definire delle estensioni per i tipi di std
- implementazioni di default generiche dei tratti
- imporre dipendenze da altri tratti sui tipi generici


  Estendere il progetto 1 - slug di stringhe - dell'esercitazione precedente, definendo e
  implementando un tratto MySlug per stringhe e slice. Il tratto deve definire i metodi che
  permettono di compilare il seguente codice e ottenere i risultati indicati:

```rust
let s1 = String::from(“Hello String”);
let s2 = “hello-slice”;
println!(“{}”, s1.is_slug()); // false
println!(“{}”, s2.is_slug()); // true
let s3: String = s1.to_slug();
let s4: String = s2.to_slug();
println!(“s3:{} s4:{}”, s3, s4); // stampa: s3:hello-string s4:hello-slice
```
Passi soluzione:
1. Aggiungere un metodo is_slug all’interno della struttura già creata
2. Risolvere prima l’esercizio in modo semplice, seguendo questi passi:

   a. definire il tratto MySlug

   b. fornire implementazioni separate sia per String che &str (nota: Rust tratta
   “&str” come un tipo a se stante)
3. Quando il punto 1 funziona, fornire, al posto delle due precedenti, una sola
   implementazione generica per tutti i tipi che possono essere acceduti come un
   riferimento a stringa. Perché è possible? Perché se abbiamo un riferimento a stringa
   possiamo sempre a) verificare se è uno slug 2) convertirlo in una stringa slug.
   Completare questa implementazione:
```rust
impl<T> Slug for T
where: ??? {
???
}
```
Impostare la clausola where in modo che T sia un tipo che possa essere acceduto
come riferimento di str (suggerimento: T deve implementare questo tratto per str:
https://doc.rust-lang.org/std/convert/trait.AsRef.html)


Notare che con questa implementazione il tratto sarà automaticamente disponibile
per tutti i tipi che permettono di ottenere un &str (anche tipi nuovi definiti dall’utente).
L’unico requisito è importarlo.