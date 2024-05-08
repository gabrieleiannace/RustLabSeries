# Esercitazione 3
## Esercizio 2
Realizzare un struct **FileSystem** che permetta di gestire la struttura (nomi e relazioni) di un
file system in memoria, offrendo operazioni di: creazione, rimozione, ricerca, update di
cartelle e file.  

L’interfaccia da realizzare è fornita in file es0302.rs, comprese le indicazioni necessarie e un
esempio d’uso.  
Per compilare e far funzionare correttamente il codice sarà necessario annotare funzioni e
metodi con gli opportuni lifetime.  
Anche qui il suggerimento è commentare tutto e aggiungere un pezzo per volta in modo da
non dover aggiustare subito tutti gli errori di compilazione.

### Nota sui riferimenti mutabili
Attenzione in particolare all’uso della funzione find nell’esempio d’uso, quando si prova ad
ottenere una get_mut() del path trovato. Non compilerà e non è possibile farla compilare
senza un utilizzo diverso. Spiegare bene il motivo, tracciando i lifetime delle variabili
coinvolte.  
Successivamente commentare quel pezzo di codice e provare ad ottenere un riferimento
mutabile dai path trovati usando il suggerimento nel codice.

### Nota di teoria
Se fate attenzione la struttura filesystem è un albero, che avete realizzato senza usare un
puntatore, ma solo una collezione standard Vec.  
Questo potrebbe far pensare che un albero binario con un nodo definito così funzioni.
````rust
struct Node {
    val: i32,
    left: Node,
    right: Node
}
````
Se provate a compilare rust vi dà un errore di compilazione, qual è e come lo spiegate?  
Se invece definire così la struttura Node (che è l’approccio usato in FileSystem), funziona
```rust
struct Node {
    val: i32,
    children: Vec<Node>
}
```
Qual è la differenza tra il primo e il secondo esempio? Dove vengono allocati i dati?