
# Esercizio 2

## Obiettivi:
- Utilizzo di array
- parsing di stringhe
- mutabilità
- utilizzo di struct ed enum
- gestire i valori di ritorno e gli errori
- lettura/scrittura da file

ESERCIZI Propedeutici  


Dopo avere creato un nuovo progetto rust provare questi tre brevi task (in funzioni dedicate)
prima di passare alla soluzione dell’esercizio descritto in seguito.  
1. Aprire, leggere e salvare un file: leggere un file “test.txt” con dentro del testo e
salvare il testo ripetuto 10 volte nello stesso file
Usare le funzioni read_to_string e write definite in std::fs
(https://doc.rust-lang.org/std/fs/)

  - a. Testare cosa capita quando il file o il path non esiste, gestire gli errori
  - b. che differenza c’è tra read e read_to_string? Provare a leggere un file con
   delle lettere accentate con read e stampare in esadecimale l’array di byte
   letto con read, allineato con il testo sulla riga sopra.  
  Esempio:  
  
    c i a o \n    

    63 69 61 6f 0a    

    Cosa notate se nel file è scritto “così\n” al posto di “ciao\n”?
2. Enum con “valore”  
A differenza del C le enum sono tipi che possono ospitare all’interno valore associato
agli elementi della enum.  
Ogni elemento di una enum può essere di un tipo diverso e con match si può estrarre
il contenuto della enum in una variabile.  
Definire quindi una enum Error con dentro due valori: Simple(SystemTime) e
Complex(SystemTime, String) e fare una funzione print_error(e: Error) che
stampi il tipo di errore e le informazioni contenute (senza usare {:?} debug, ma
gestendo i valori della enum in modo opportuno)
3. Funzioni che possono restituire errori.
In rust una funzione per segnalare un errore può usare la enum Result, che è definita
in questo modo, analogamente a Option
pub enum Result<T, E> {
Ok(T),
Err(E)
}  
T è il tipo del valore restituito come risultato, mentre E quello dell’errore, che può
essere una enum qualsiasi  
Implementare questa funzione:  
pub mul(a: i32, b: i32) -> Result<u32, MulErr> {}  
dove MulErr è  
pub enum MulErr {Overflow, NegativeNumber};  
mul quindi deve restituire:  
- il risultato se possibile
- MyErr::NegativeNumber se uno tra a e b è negativo
- MyErr::Overflow se il risultato è più alto del valore massimo di u32  
4. Uso di self, &self e &mut self
Nei metodi di una struct si può utilizzare self per avere un riferimento all’oggetto su
cui si sta operando: che differenza di comportamento c’è tra self, &self e &mut self?  
Ipotizziamo di avere la struttura
```rust
struct Node {
  name: String,
  size: String,
  count: u32,
}
impl Node {
  pub fn new(name: String) -> Node {
    Node {name, 0, 0}
  }
}
```
Aggiungere due metodi size() e count() in modo che questo codice  
```rust
let node = Node(String::new(“nodo”)).size(10).count(5);
```
crei il nodo {“nodo”, 10, 5}  
Quante struct di tipo Node costruisco in tutto? Ci sono penalità dal punto di vista
dell’efficienza?  
Questo modo di creare l’oggetto, un pezzo per volta a partire dai default, viene
definito “builder pattern”. Ciò permette di ovviare al problema della mancanza di
valori default e del polimorfismo nei metodi rust: nel caso di molti parametri opzionali
posso usare il pattern per creare una versione “base” dell’oggetto con i valori di
default e modificare solo gli attributi che hanno un valore diverso.  
Aggiungere un metodo to_string() che lo trasformi in stringa “name:node size:10
count:5”. Come deve essere definito self in questo caso? Con o senza “&”?
Infine aggiungere due metodi grow() e inc() che aumentano rispettivamente la size e
il count di 1 senza creare un nuovo oggetto. Qui self come va definito?  

## TESTO ESERCIZIO
Un programma deve gestire la costruzione di uno schema di battaglia navale 20x20 salvato
su file.  
Il formato del file è il seguente (21 righe):  
- LINEA 1: N1 N2 N3 N4, 4 interi separati da spazio che indicano il numero di navi
rispettivamente di lunghezza 1, 2 , 3 e 4, che si possono ancora aggiungere alla
board  
- LINEE 2..21, 20 righe di 20 caratteri con “ “ (spazio) per le caselle vuote e “B” per
quelle con navi  
La costruzione della board avviene per passi, invocando il programma con dei parametri
- cargo run -- new board.txt 4,3,2,1  
questo crea una nuova board vuota nel file board.txt e può ospitare 4 navi da 1, 3
navi da 2, 2 navi da 3, e una da 4.  
- cargo run -- add board.txt 3V 10,10  
legge la board in board.txt, aggiunge una nave da 3 caselle in verticale, partendo
dalla casella (10,10) e andando giù 3 caselle, fino a (12,10). Possibili direzioni: H e V.
Aggiunta la nave, salva il risultato in board, aggiornando anche le navi disponibili
nella prima linea.  
Gli indici iniziano da 1 fino a 20  
L’operazione di add deve essere “safe” e stampare errore, senza panic e senza modificare il
file nel caso in cui non si possa aggiungere la nave. Casi in cui la nave non si può
aggiungere:  
- sono già state aggiunte tutte le navi possibili  
- una nave si sovrappone o ha almeno un casella che “tocca” una nave esistente
(anche di angolo)  
- la nave va fuori dallo schema  
(per la gestione della command line utilizzare sempre clap, come nell’esercizio precedente,
notare gli spazi che separano file, dimensione boat e posizione start: provare a leggere tutti
gli arg in un vettore di stringhe)  
Per gestire la board utilizzare la seguente struttura ed implementare i metodi indicati senza
modificare la signature dei metodi presenti:
```rust
const bsize : usize = 20;
pub struct Board {
  boats : [u8; 4], data : [[u8; bsize]; bsize],
} pub enum Error {
  Overlap,
  OutOfBounds,
  BoatCount,
} pub enum Boat {
  Vertical(usize) Horizontal(usize)
} impl Board {
  /** crea una board vuota con una disponibilità di navi */
  pub fn new (boats : &[u8])->Board {}
  /* crea una board a partire da una stringa che rappresenta tutto
  il contenuto del file board.txt */
  pub fn from(s : String)->Board {}
  /* aggiunge la nave alla board, restituendo la nuova board se
  possibile */
  /* bonus: provare a *non copiare* data quando si crea e restituisce
  una nuova board con la barca, come si può fare? */
  pub fn add_boat(self, boat
                  : Boat, pos
                  : (usize, usize))
      ->Result<Board, Error> {}
  /* converte la board in una stringa salvabile su file */
  pub fn to_string(&self)->String
}
```

## Bonus
1. identificare e scrivere i test per struct Board  
2. modificare Board in modo che add_boat alteri la struttura Board anziché crearne una
nuova; per fare questo prestare attenzione al parametro self, come va modificato?  
3. modificare il parse di clap in modo che gestisca i parametri in modo più espressivo e
provare ad utilizzare la modalità builder anziché derive  
(https://docs.rs/clap/latest/clap/_tutorial/chapter_0/index.html):  
__cargo run -- add --file=board.txt --boat=3V --start=10,10__
