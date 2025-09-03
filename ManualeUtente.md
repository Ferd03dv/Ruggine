# Manuale Utente
## Progetto G45 – Programmazione di Sistema (A.A. 2024/2025)

---

## 1. Introduzione
Il progetto **Ruggine** del Gruppo **G45** è un’applicazione web sviluppata in **Rust** per la gestione di utenti, gruppi e messaggi, con supporto ad autenticazione e logging.  
Il sistema prevede un backend realizzato in Rust e un frontend in JavaScript/Node.js.
L'applicazione è di tipo client/server e serve a gestire una chat di messaggi testuali. La chat deve inoltre prevedere la possibilità di creare gruppi di utenti per la condivisione di messaggi

---

## 2. Requisiti di Sistema

### Software
- **Rust** >= 1.85 (Rivedi perché!)
- **Cargo** (incluso in Rust)  
- **Node.js** >= 19.1.1
- **npm** >= 11.5.2 (Controllato con npx npm --v) 

### Hardware
## SW e HW da RICONTROLLARE
- CPU: Dual core o superiore  
- RAM: almeno 2 GB  
- Spazio su disco: almeno 1.60 GB (Dimensione Progetto da Proprietà)

### Dipendenze principali
- Librerie Rust definite in `Cargo.toml`
    - sqlx: ORM asincrono per utilizzare il DB sqlite in RUST:
        - sqlite: supporto per sqlite
        - runtime-tokio-native-tls: integrazione con Tokio e TLS
        - macros: permette la scrittura di query SQL scritte inline mediante la macro query!
        - chrono: integrazione del DateTime per gestione di date/tempi.
    - tokio: Runtime asincrono, usato per gestire async/await, I/O, ...
        - full: abilita tutte le feature di Tokio
    - chrono: Libreria per la gestione di date, orari, timezone e loro formattazioni.
    - axum: Framework Web per la costruzione di API Rest
    - serde: Libreria per la Serializzazione/Deserializzazione dei dati.
    - serde_json: Abilita il supporto ai file JSON per serde
    - dotenv: permette di caricare variabili d'ambiente mediante un file .env
    - tower-http: Estensioni HTTP per tower, nel nostro caso specifico serve per la gestione del Cross Origin (CORS)
    - tower: Libreria di astrazioni per middleware e servizi 
    - bcrypt: implementa l'algoritmo di hashing bcrypt, usato per proteggere le password.
- Dipendenze Node.js definite in `frontend/package.json`  
    #### Controlla se usiamo i testing-...
    - @testing-library/dom: Libreria per testare direttamente il DOM, per verificare gli elementi HTML e le interazioni senza React.
    - @testing-library/jest-dom: estensioni per Jest che permettono assert più leggibili sul DOM, tipo expect(element).toBeVisible().
    - @testing-library/react: libreria per testare componenti React come un utente reale.
    - @testing-library/user-event: simula eventi utente (click, digitazione, selezione, ecc.) nei test React.
    - axios: client HTTP basato su Promise per comunicare con il backend (GET, POST, ecc.).
    - react: libreria principale per la realazzione della UI e dei componenti riutilizzabili.
    - react-dom: permette a React di interfacciarsi con il DOM del browser e renderizzare componenti.
    - react-scripts: set di script e configurazioni di Create React App, include build, start, test e linting senza configurazione manuale.
    - web-vitals: libreria per misurare metriche di performance del web (LCP, FID, CLS) e monitorare la UX.


---

## 3. Installazione

### 3.1 Backend (Rust)
1. Clonare il repository o estrarre l’archivio:
   ```bash
   git clone https://github.com/PdS2425-C2/G45.git
   cd G45

2. Compilare il Progetto
    ```bash
    cargo run

3. Avviare il Server
    ```bash
    cargo run

### 3.2 Frontend (Node.js)
1. Entrare nella cartella frontend
    ```bash
    cd frontend

2. Installare le Dipendenze
    ```bash
    npm install

3. Avviare il Frontend
    ```bash
    npm start

## 4. Utilizzo dell'Applicazione
### Avvio dell'Applicazione
1. Avvia prima il Backend in Rust (cargo run)
2. Avvia il Frontend (npm start)
3. Aprire il Browser e collegarsi a:
    ```
    http://localhost:3000

### Funzionalità Principali
- Autenticazione: Posso creare un nuovo account mediante il form di Registrazione ed effettuare il login con Username e Password.
- Gestione Utenti: Registrazione, Login e Autenticazione.
- Messaggi: Invio e Ricezione di messaggi.
- Gruppi: Creazione e Gestione di Gruppi.
- Inviti: Inviare inviti agli altri utenti.

## 5. File di Configurazione
- .env: Variabili di ambiente per DB e server (URL del DB e porta)
- Cargo.toml: Configurazione e Dipendenze del Progetto Rust.
- package.json: Dipendenze del Frontend.

## 6. Esempio di Utilizzo delle API
- Vedi file test.http

## 7. Contatti
- Autori: Donato Mangeruca, Loris Catalano, Marco Donatucci, Ferdinando ... 
- Email: ... Opzionale
- Gruppo 45 - progetto Ruggine


    
    
