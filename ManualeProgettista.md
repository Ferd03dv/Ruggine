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

## 3. Backend Rust
### 3.1 Panoramica
- Il Backend viene sviluppato in Rust, utilizzando il Framework Axum per gestire le rotte HTTP.
- Utilizziamo un insieme di REST API per la gestione di **Autenticazione**, **Utenti**, **Messaggi**, **Gruppi** e **Inviti**

### 3.2 Struttura delle Cartelle (Backend)
#### 3.2.1 main.rs
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

#### 3.2.2 server.rs
- Configurazione del server web.  
- Configura e Inizializza il DB:
    - Cerca nel file `.env` **DATABASE_URL** e se non lo trovo crea un file SQLITE ex-novo.
    - Ottiene così la Stringa di Connessione tramite cui inizializza il Database.
    - Restituisce un Pool di Connessioni (SqlitePool) da usare in tutte le query.
- Costruzione delle Rotte e Aggiunta di CorsLayer che permette al Server di accettare richieste, metodi e headers da qualunque origine.
- Avvia il Server HTTP Axum su localhost:8000.
    - app.into_make_service() converte il router in un servizio pronto per il Server HTTP.

#### 3.2.3 config/database.rs
- Import delle Strutture necessarie per lavorare con un Pool di Connessioni SQLite.
    - Inizializzazione di un Pool di Connessioni con massimo 5 connessioni aperte.
- Gestione della connessione al database con la Funzione Asincrona init_db, che restituisce un pool di connessioni se tutto va bene, altrimenti un SQLx::Error 
- Inizializzazione delle tabelle necessarie per gestire utenti, gruppi, messaggi e inviti (solo se non esistono già).

#### 3.2.4 controllers/
- Contiene i moduli che ricevono e processano le richieste HTTP.  
    - Si usa la Libreria **Serde** per gestire la Deserializzazione e la serializzazione dei dati da/verso dati esterni.
- **auth_controller.rs**: gestisce login, registrazione, **NON USIAMO PER ORA DEI TOKEN DI SESSIONE**.  
- **message_controller.rs**: gestisce invio e ricezione di messaggi.  

#### 3.2.5 models/
- Import del Tratto FromRow che permette a una struct di essere popolata in maniera automatica dai risultati di una Query SQL.
- Definizione delle entità principali divise per dominio:  
  - **user.rs** – rappresentazione utente.  
  - **message.rs** – rappresentazione messaggio.  
  - **group.rs** – rappresentazione gruppi.
  - **invitation.rs** - rappresentazione inviti.  

#### 3.2.6 repositories/
- Import dei modelli da utilizzare e di **sqlx::query** per Query Generiche e **sqlx::query_as** per query da mappare in Struct Rust
- Strato di accesso al database che contiene le funzioni che leggono/scrivono dati su db.  
- Incapsula le query per ogni modello (`user_repository`, `message-repository`, `group_repository`.).  

#### 3.2.7 services/
- Contiene la logica applicativa vera e propria, (**SERVE SPIEGARLO?**) a differenza del Repository, che si occupa solo di leggere/scrivere su DB, sa cosa farsene dei dati.

#### 3.2.8 routes/
- Import di Axum per definire le rotte e i tipi di rotte HTTP.
- Definisce le rotte HTTP e le associa ai rispettivi controller.  
- Struttura modulare per le varie aree funzionali (auth, messages).  

#### 3.2.9 utils/
- Funzionalità di supporto, nel nostro caso contiene il logger dell'utilizzo di CPU.
- `cpu_logger.rs`: registra informazioni sulle performance della CPU.  

### 3.3 Flusso di Funzionamento del Backend
- **main.rs**: avvia il server asincrono con Tokio
- **server.rs**: configura il server e il database, abilita CORS e costruisce il Router.
- ogni risposta HTTP arriverà a una **rotta** che **invocherà il Controller** corrispondente.
- il **controller** si occuperà di **serializzare/deserializzare** i dati e chiamerà i services che contengono la Logica Applicativa.
- i **services** interagiscono con i **repositories** che si occupanp di **leggere e scrivere sul Database**
- i risultati vengono restituiti al client sotto forma di **JSON**

---

## 4. Frontend React
### 4.1 Panormaica
- Il Frontend è sviluppato in React (JavaScript) e gira in ambiente Node.js e rappresenta la parte presentazionale del sistema.
- Serve a gestire la UI, in modo tale che l'utente possa interfacciarsi con l'applicazione di messaggistica, senza per questo essere a conoscenza delle logiche implementative dell'applicazione in se'.
- Principali Caratteristiche:
  - Realizzato in **Node.js**.  
  - Utilizza `npm` per la gestione delle dipendenze.  
  - Comunica con il backend tramite chiamate API REST (consumate con `axios`).
  - Gestisce lo stato interno con React Hooks.  
  - Avvio in Locale gestito sulla porta **3000**, configurata sul file .env.
  - Organizzazione Modulare del codice in componenti, hooks e servizi, permettendo così una separazione tra la logica di presentazione, quella di stato e quella di comunicazione con il Backend.

### 4.2 Struttura delle Cartelle (Frontend)
#### public
- Contiene al suo interno le Risorse Statiche e il file robots.txt, file per i crawler dei motiri di ricerca per definire qusli percorsi del sito possono essere indicizzati.
  - **L'ha messo marco? Eventualmente se di default tolgo la descrizione!**
#### src
- contiene al suo interno le cartelle **components**, **hooks** e **services**
  - **components**: contiene i componenti React responsabili della UI.
  - **hooks**: contiene hook React personalizzati per astrarre logiche comuni:
    - useUsername.js: mantiene una Cache Locale e Globale dei nomi utente per evitare chiamate duplicate al Backend, grazie all'ausilio di 2 metodi:
      - getUsernameById: Recupera lo username da API e aggiorna la cache.
      - geUsernamenSync: restituisce immediatamente il valore se è gia presente all'interno della cache.
    - useUsernames contiene al suo interno un altro hook che permette di caricare in maniera preventiva i nomi utente da una lista di ID (usePreloadUsernames)
  - **services**: contiene il file api.js, utile per la comunicazione con il backend, grazie all'utilizzo di funzioni centralizzate.
    - implementa vari servizi tra cui:
      - authService: utilizzato per gestire **Login** e **Registrazione**
      - userService: utilizzato per selezionare un utente sulla base del suo ID
      - groupSevice: utilizzato per la **Creazione** e il **Recupero** dei gruppi
      - invitationService: utilizzato per gestire gli inviti a gruppi: **Creazione**, **Accettazione**, **Rifiuto** e **Visione** delle richieste ricevute
      - messageService: utilizzato per **Inviare** e **Ricevere** messaggi.

### 4.3 Flusso di Funzionamento del Frontend
- L'utente accede all'app tramite browser (localhost::3000)
- I Componenti mostrano le UI interattive
- Gli Hooks servono nel nostro caso a ottimizzare le chiamate ripetute all'utente
- I Services servono a comunicare con il Backend mediante Axios.
- I dati ricevuti vengono aggiornati nello stato e presentati all'utente

## 5. Database SQLite
### 5.1 Panoramica
- Per questo progetto utilizziamo SQLite come DB Relazione per rendere i nostri dati persistenti.
- Motivi della scelta:
  - DB Leggero e Portatile
  - NON richiede un Server Dedicato
- Il DB viene configurato mediante file `.env`, sotto la voce **DATABASE_URL**

### 5.2 Struttura del Database
- **user**: presenta dati relativi agli utenti registrati:
  - Chiave Primaria: u_id
  - Campi username e password usati per fare il login
- **groups**: presenta dati relativi ai gruppi:
  - Chiave Primaria: g_id
  - Chiave Esterna: created_by che fa riferimento a u_id (tabella user)
- **user_group**: presenta dati relativi agli utenti presenti in certi gruppi
  - Chiave Primaria: ug_id
  - Chiave Esterna: user_id che fa riferimento a u_id (tabella user) e group_id che fa riferimento a g_id (tabella groups)
- **message**: presenta dati relativi ai messaggi scambiati nei vari gruppi:
  - Chiave Primaria: m_id
  - Chiave Esterna: sender_id che fa riferimento a u_id (tabella user) e group_id che fa riferimento a g_id (tabella groups)
- **invitation**: presenta dati relativi agli inviti di partecipazione a un gruppo:
  - Chiave Primaria: i_id
  - Chiave Esterna: invited_by e invited_user che fanno riferimento a u_id (tabella user) e group_id che fa riferimento a g_id (tabella groups)
  - status: si riferisce allo stato corrente dell'invito: 0 = Pending, 1 = Accepted e 2 = Rejected.

### 5.3 Gestione delle Connessioni
- Gestione delle Connessioni implementata mediante SqlitePool, che nel nostro caso ci permette di configurare e mantenere attive 5 Connessioni Contemporanee, in modo tale da ridurre il numero di chiamate al DB, con conseguente ottimizzazione di Memoria e Performance.
- Accediamo ai dati in maniera Centralizzata tramite i Repository
- Abbiamo un Repository per ogni entità, ad eccezione di group_repository che gestisce sia la tabella groups che user_group.

### 5.4 Flusso di Accesso al Database
- Un **Service** **richiede una Operazione**.
- Il **Repository** corrispondente esegue la **query SQL** tramite sqlx::query o sqlx::query_as
- I dati vengono successivamente **mappati** nelle struct definite nei corrispondenti **Models**
- I risultati vengono **restituiti al Service** che li ha chiamati e successivamente al **Controller**.

---

## 6. Scelte Implementative
- **Rust** scelto per il backend: Garantisce **sicurezza della memoria, performance e concorrenza**.  
  - **Sicurezza** della Memoria: Rust garantisce che non ci siano errori di memoria comuni (ES. Null Pointer) grazie al suo sistema di Ownership e al Borrow Checker.
  - **Performance**: Rust non dispone di Garbage Collector, ma il controllo della memoria è Deterministico, dato che ogni variabile (se non gestita opportunamente) cessa di esistere nel momento in cui esce dal suo scope. Ciò garantisce Alte Prestazioni dato che NON avrò overhead dovuti al tracciamento degli oggetti tipici del Garbage Collector, avendo così una Latenza più bassa e un Throughput più stabile.
  - **Concorrenza Sicura**: Rust permette di scrive codice che sfrutti i Multithread senza rischi di Data Race
  - **Errori a Compile-Time**: Il Compilatore di Rust verifica molte proprietà a Compile-Time, riducendo così i Bug a Run-Time. Ciò garantisce meno errori in produzione un Codice generalmente più Robusto.
- **Pattern MVC semplificato**:  
  - **Controller**: Gestisce le richieste dell'utente.  
  - **Model**: Rappresenta le entità persistenti dell'applicazione divise per dominio.  
  - **Repository**: Permette l'accesso e l'interazione con il DB.  
  - **Service**: Contiene la logica applicativa.
  - **View**: Non utilizzato. La parte presentazionale dell'applicazione è demandata al Frontend. 
- **Database SQLite**: configurabile tramite `.env`, garantendo così flessibilità e portabilità. 
  - Leggero e Portatile
  - Non richiede un Server Dedicato, Installazioni o Configurazioni particolari.
- **Pool di Connessioni**: per gestire efficientemente le risorse (Memoria e CPU), riducendo inoltre il tempo di Connessione al Database.
- **Logging CPU**: utile per il benchmark e il monitoraggio delle performance della CPU.  
- **React** scelto per il frontend: 
  - L'interfaccia strutturata in Componenti Modulari permette il riutilizzo dei componenti e una maggior semplicità nell'isolamento dei bug.
  - React mantiene un Virtual DOM in memoria, aggiornando nel DOM reale solo le parti cambiate realmente, migliorando così le performance generali della nostra Applicazione.
  - Il Flusso Unidirezionale dei dati dal padre verso i figli, semplifica il debugging e ci permette di gestire in maniera più appropriata gli Effetti Collaterali.
  - Portabilità: React NON è legato a un backend specifico, si potrebbe in futuro scegliere di cambiare il Backend senza dover necessariamente riscrivere la UI.

---

## 7. Analisi delle Prestazioni
- **Backend scritto in Rust**: ottimizzato per concorrenza e gestione di richieste multiple.  
- **Testing**: Possibilità di Testing delle API tramite strumenti HTTP (`test.http` incluso nel progetto).  
- **Logger CPU integrato**: analisi delle prestazioni della CPU durante l'utilizzo dell'applicazione.  

---

## 8. Contatti
- Autori:
  - [Donato Mangeruca](https://github.com/DonatoMangeruca)
  - [Loris Catalano](https://github.com/loris-catalano)
  - [Marco Donatucci](https://github.com/marcodonatucci)
  - [Ferdinando Del Vecchio](https://github.com/Ferd03dv)
- Gruppo 45 - progetto Ruggine
