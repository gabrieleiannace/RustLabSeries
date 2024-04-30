# Esercitazione 3
## Esercizio 1
In questo esercizio affronteremo alcuni esempi di ricerca multipla dentro una struttura dati
per familiarizzare con l’uso di lifetime in Rust, funzioni lambda e iteratori.  
Non tutti gli esempi si compileranno e lo scopo è proprio capire perché il compilatore non
permette alcune operazioni apparentemente lecite.

Nel file **es301.rs** vengono fornite alcune funzioni senza annotazioni di lifetime e senza
implementazione e occorre annotarle con i lifetime corretti e completare l’implementazione
per ottenere i risultati attesi dagli esempi di utilizzo forniti.  
Suggerimento: commentare tutto il file e scommentare una sola funzione per volta per
concentrarsi su un solo problema, senza dover prima risolvere tutti i problemi di
compilazione

Nella prima (funzioni subsequence1-3) si richiede di cercare delle sottosequenze di DNA
all’interno di stringhe che rappresentano catene di DNA. Il DNA è una lunghissima catena di
molecole di 4 tipi, che si indicano con i simboli A C G e T; quindi una sequenza di DNA può
essere rappresentata da una stringa con i caratteri ACGT ripetuti a piacere.
Una particolare sottosequenza invece può essere rappresentata sinteticamente da una
stringa del tipo “A1-2,T1-3,A2-2,G2-4,C2-2”, dove il numero rappresenta il numero minimo
massimo di ripetizioni della base corrispondente. Quindi questa stringa richiede di cercare
una sequenza di lunghezza variabile con 1-2 A, 1-3 T, 2 A, 2-4 G, 2 C (esempio
ATAAGGCC, ma anche AATTAAGGGCC).

Si richiede poi di implementare tre modi alternativi per ciclare sulle sottosequenze trovate:
- una funzione lambda passata alla funzione di ricerca e chiamata per ogni
  sottosequenza trovata (subsequence4)
- un iteratore naive realizzato tramite una struct, che permette di interrompere la
  ricerca in qualsiasi momento (SimpleDNAIter)
- un iteratore compliant con rust (DNAIter)
- un iteratore creato da un generatore, senza dover definire una struct di supporto
  (subsequence5_iter)

Vedere gli esempi in es0301.rs e i commenti che guidano alla soluzione