# Manuale del Progettista
## Progetto G45 – Programmazione di Sistema (A.A. 2024/2025)

---

## 1. Introduzione
Questo documento è una **descrizione tecnica** del Progetto Ruggine ed esplora le scelte implementative fatte dal Gruppo **G45**.  
È rivolto a sviluppatori e manutentori per comprendere la struttura interna del sistema, le logiche applicative e le possibili estensioni future.

---

## 2. Architettura del Sistema
Il progetto è una **Applicazione Web** basata sull'architettura **client-server**:

- **Backend**: sviluppato in **Rust**, fornisce un'interfaccia di tipo REST API per la gestione di autenticazione, utenti, messaggi, gruppi e inviti.  
- **Frontend**: sviluppato in **Node.js/JavaScript**, gestisce l’interfaccia utente e comunica con il backend tramite richieste HTTP/JSON, che verranno consumate mediante la libreria **Axios**.
- **Database**: configurato tramite il file `.env`, che fornisce URL del Database e Porta di Ingresso, il Database è utilizzato per persistenza di utenti, gruppi e messaggi, mediante l'utilizzo di un file sviluppato con **SQLite**.

### Struttura delle cartelle principali
```
/ (root)
- data/
- src/ (backend)
  - config/
  - controllers/
  - models/
  - repositories/
  - routes/
  - services/
  - utils/
- frontend/
  - public/
  - src/
    - components/
    - services/
```


---

## 3. Componenti Principali (Backend Rust)

### 3.1 main.rs
- Entry Point dell’applicazione.  
- Inizializza tutti i moduli necessari:
    - config/database: Gestisce la connessione al DB e crea le tabelle necessarie al primo avvio.
    - models: Definisce le entità della nostra applicazione (Gruppi, Inviti, Messaggi e Utenti)
    - repositories: contiene al proprio interno le query al DB utili al funzionamento dell'applicazione.
    - routes: Definisce le REST API mediante AXUM il cui compito è quello di smistare le richieste HTTP del client verso il giusto Controller.
    - server: Avvia e Configura il Server Web.
    - controllers: Gestisce la Logica delle Richieste.
    - services: Contiene la logica applicativa dell'applicazione
    - utils: Contiene principalmente il cpu_logger per monitorare le prestazioni della CPU.
- Avvia un Logger della CPU per monitorare le risorse durante l'utilizzo del Server.
- Lancia un Server Web Asincrono (Tokio), che rimane in esecuzione per poter gestire le REST API del progetto.

### 3.2 server.rs
- Configurazione del server web.  
- Configura e Inizializza il DB:
    - Cerca nel file `.env` **DATABASE_URL** e se non lo trovo crea un file SQLITE ex-novo.
    - Ottiene così la Stringa di Connessione tramite cui inizializza il Database.
    - Restituisce un Pool di Connessioni (SqlitePool) da usare in tutte le query.
- Costruzione delle Rotte e Aggiunta di CorsLayer che permette al Server di accettare richieste, metodi e headers da qualunque origine.
- Avvia il Server HTTP Axum su localhost:8000.
    - app.into_make_service() converte il router in un servizio pronto per il Server HTTP.

### 3.3 config/database.rs
- Import delle Strutture necessarie per lavorare con un Pool di Connessioni SQLite.
    - Inizializzazione di un Pool di Connessioni con massimo 5 connessioni aperte.
- Gestione della connessione al database con la Funzione Asincrona init_db, che restituisce un pool di connessioni se tutto va bene, altrimenti un SQLx::Error 
- Inizializzazione delle tabelle necessarie per gestire utenti, gruppi, messaggi e inviti (solo se non esistono già).

### 3.4 controllers/
- Contiene i moduli che ricevono e processano le richieste HTTP.  
    - Si usa la Libreria **Serde** per gestire la Deserializzazione e la serializzazione dei dati da/verso dati esterni.
- **auth_controller.rs**: gestisce login, registrazione, **NON USIAMO PER ORA DEI TOKEN DI SESSIONE**.  
- **message_controller.rs**: gestisce invio e ricezione di messaggi.  

### 3.5 models/
- Import del Tratto FromRow che permette a una struct di essere popolata in maniera automatica dai risultati di una Query SQL.
- Definizione delle entità principali divise per dominio:  
  - **user.rs** – rappresentazione utente.  
  - **message.rs** – rappresentazione messaggio.  
  - **group.rs** – rappresentazione gruppi.
  - **invitation.rs** - rappresentazione inviti.  

### 3.6 repositories/
- Import dei modelli da utilizzare e di **sqlx::query** per Query Generiche e **sqlx::query_as** per query da mappare in Struct Rust
- Strato di accesso al database che contiene le funzioni che leggono/scrivono dati su db.  
- Incapsula le query per ogni modello (`user_repository`, `message-repository`, `group_repository`.).  

### 3.7 services/
- Contiene la logica applicativa vera e propria, (**SERVE SPIEGARLO?**) a differenza del Repository, che si occupa solo di leggere/scrivere su DB, sa cosa farsene dei dati.

### 3.8 routes/
- Import di Axum per definire le rotte e i tipi di rotte HTTP.
- Definisce le rotte HTTP e le associa ai rispettivi controller.  
- Struttura modulare per le varie aree funzionali (auth, messages).  

### 3.9 utils/
- Funzionalità di supporto, nel nostro caso contiene il logger dell'utilizzo di CPU.
- `cpu_logger.rs`: registra informazioni sulle performance della CPU.  

---

## 4. Frontend - Aspetto FINISCA MARCO
- Realizzato in **Node.js**.  
- Utilizza `npm` per la gestione delle dipendenze.  
- Comunica con il backend tramite chiamate API (`axios`).  
- Porta di default: **3000**.  

- **PARTE FRONTEND - ASPETTO FINISCA MARCO**


---

## 5. Scelte Implementative
- **Rust** scelto per il backend: Garantisce **sicurezza della memoria, performance e concorrenza**.  
- **Pattern MVC semplificato**:  
  - **Controller**: Gestisce le richieste dell'utente.  
  - **Model**: Rappresenta le entità persistenti dell'applicazione divise per dominio.  
  - **Repository**: Permette l'accesso e l'interazione con il DB.  
  - **Service**: Contiene la logica applicativa.
  - **View**: Non utilizzato. La parte presentazionale dell'applicazione è demandata al Frontend. 
- **Database SQLite**: configurabile tramite `.env`, garantendo così flessibilità e portabilità. 
- **Pool di Connessioni**: per gestire efficientemente le risorse (Memoria e CPU), riducendo inoltre il tempo di Connessione al Database.
- **Logging CPU**: utile per il benchmark e il monitoraggio delle performance della CPU.  
- **INSERISCI PARTE FRONTEND QUANDO FINISCE MARCO** 

---

## 6. Analisi delle Prestazioni
- **Backend scritto in Rust**: ottimizzato per concorrenza e gestione di richieste multiple.  
- **Testing**: Possibilità di Testing delle API tramite strumenti HTTP (`test.http` incluso nel progetto).  
- **Logger CPU integrato**: analisi delle prestazioni della CPU durante l'utilizzo dell'applicazione.  

---

### 7 e 8 le lascio? ##

## 7. Problemi Riscontrati e Soluzioni
- **Gestione della concorrenza**: risolto utilizzando async/await e librerie Rust per networking.  
- **Configurazione variabile del DB**: soluzione con file `.env`.  

---

## 8. Possibili Estensioni Future
- Aggiunta di **cifratura end-to-end** per i messaggi.  
- Interfaccia web più avanzata con framework frontend (React/Vue).  
- Deployment in container **Docker** per portabilità.  
- Test unitari e di integrazione più estesi.  

---

## 9. Contatti
- Autori:
  - [Donato Mangeruca](https://github.com/DonatoMangeruca)
  - [Loris Catalano](https://github.com/loris-catalano)
  - [Marco Donatucci](https://github.com/marcodonatucci)
  - [Ferdinando Del Vecchio](https://github.com/Ferd03dv)
- Gruppo 45 - progetto Ruggine
