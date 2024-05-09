# Esercitazione 4
## Esercizio 1
Il file `es0401.rs` contiene due moduli List1 e List2, con due modelli alternativi per
implementare una linked list.  
Il primo modello si basa su una enum, come visto a lezione, con una definizione ricorsiva
della lista. Il secondo modello invece utilizza un layout basato su una struct Node con un
approccio più simile ad una implementazione C like.  
Implementare entrambe le interfacce come descritte nei commenti del file.

Analizzare le differenze di allocazione in memoria:
```rust
let mut l1 = List1::List::<i32>::new();
l1.push(10);
let mut l2 = List2::List::<i32>::new();
l2.push(10)
```
Dove sono allocate le head di l1 e l2?  
Che differenze ci sono nell’ultimo nodo tra l1 e l2?  

Infine modificare List2 per renderla una lista doppio linkata:
- si avranno due puntatori, alla testa (head) e alla coda (tail) della lista e ogni nodo
deve puntare anche al precedente nella catena, oltre che al successivo.
- si dovrà poter fare push e pop da cima e coda della lista
- inoltre aggiungere in metodo ```fn popn(&mut self, n: usize) -> Option<T>``` che
rimuove l’elemento ennesimo della lista e lo restituisce.  
Attenzione ad aggiornare correttamente head o tail se il nodo eliminato è all’inizio o
alla fine.

Può ancora essere usato Box per puntare ai nodi adiacenti? Come va modificata la struttura
di Node per avere più riferimenti? Vedere i suggerimenti nei commenti del file allegato
