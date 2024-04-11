# Esercizio 2 - ComplexNumber

## Obiettivi:
- panoramica estesa dei tratti in std
- creare un modulo in Rust

Implementare un tipo ComplexNumber che supporti gli operatori aritmetici di base (+, - ecc),
che possa essere copiato, clonato, confrontato con se stesso e un numero reale, usato
all’interno delle collezioni standard di Rust (vettori, hashmap, deque).

I tratti da implementare e le funzioni che deve realizzare sono definite dal file di test
complex_numbers.rs fornito a parte. Come procedere:

1. creare un nuovo progetto rust
2. copiare il file in tests/
3. commentare tutti i test tranne il primo (altrimenti non compila)
4. realizzare in lib.rs un modulo “solution” con all’interno la struct ComplexNumber
5. iniziare ad implementare ComplexNumber e i tratti richiesti per completare il primo
   test
6. scommentare il successivo test, seguire le indicazioni nei commenti e così via fino al
   completamento dei test (andare in ordine in quanto alcuni tratti da implementare
   sono dipendenti dai precedenti e potrebbe non compilare

Per eseguire i test di un particolare modulo:
```shell
cargo test --package [nome_package] --test [nome_modulo_di_test]
```
es per il package complex_numbers (nome del crate) ed il modulo di test
complex_numebers:
```shell
cargo test --package complex_numbers --test complex_numbers
```