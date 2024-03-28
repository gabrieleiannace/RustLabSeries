//******************[INDIVIDUAZIONE CASI DI TEST]*************************


/*
1. Test di creazione della board vuota
   - Verificare che una nuova board sia correttamente creata con il numero di navi specificato.
   - Assicurarsi che tutte le caselle siano vuote.

2. Test di aggiunta di una nave in posizione verticale:
   - Aggiungere una nave in una posizione verticale valida e verificare che la board sia stata aggiornata correttamente.
   - Verificare che il numero di navi disponibili sia stato ridotto di conseguenza.
   - Provare ad aggiungere una nave in una posizione verticale che sovrappone una nave esistente e assicurarsi che venga gestito correttamente l'errore.

3. Test di aggiunta di una nave in posizione orizzontale:
   - Aggiungere una nave in una posizione orizzontale valida e verificare che la board sia stata aggiornata correttamente.
   - Verificare che il numero di navi disponibili sia stato ridotto di conseguenza.
   - Provare ad aggiungere una nave in una posizione orizzontale che sovrappone una nave esistente e assicurarsi che venga gestito correttamente l'errore.

4. Test di aggiunta di una nave che va fuori dallo schema:
   - Provare ad aggiungere una nave che va al di fuori dei limiti della board e assicurarsi che venga gestito correttamente l'errore.

5. Test di aggiunta di tutte le navi disponibili:
   - Aggiungere tutte le navi disponibili e verificare che la board sia stata completamente riempita correttamente.

6. Test di conversione della board in stringa:
   - Verificare che la conversione della board in una stringa da salvare su file sia corretta.

7. Test di creazione della board da una stringa:
   - Creare una board a partire da una stringa e verificare che sia stata creata correttamente.

8. Test di gestione degli errori:
   - Verificare che tutti gli errori vengano gestiti correttamente senza causare panico e che non venga modificato il file nel caso di errore.

9. Test di vari casi limite:
   - Testare la funzionalità con input estremi per assicurarsi che il programma gestisca correttamente tutti i casi possibili.
 */

//Clap https://docs.rs/clap/latest/clap/_tutorial/chapter_2/index.html#options