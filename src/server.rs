use axum::{Router, Server};
use sqlx::SqlitePool;
use std::{env, fs};
use std::net::SocketAddr;
use dotenv::dotenv;
use tower_http::cors::{CorsLayer, Any};

// crate dei repositories poi da rimuover!
// Le query create NON verrano usate qui
use crate::config::database::init_db;
use crate::repositories::user_repository::get_all_users;
use crate::repositories::message_repository::{send_message, get_messages_by_group};
use crate::repositories::group_repository::create_group;
use crate::routes;

/// Funzione che si occupa di tutto: init DB, router, avvio server
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    //carico il file di environment
    dotenv().ok();

    // Ottieni la directory corrente
    // Leggi DATABASE_URL da .env oppure costruisci manualmente!
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            // Se non esiste, costruisci come prima (fallback)
            let mut db_path = env::current_dir()?;
            db_path.push("data");
            if !db_path.exists() {
                fs::create_dir_all(&db_path)?;
            }
            db_path.push("ruggine.db");
            if !db_path.exists() {
                fs::File::create(&db_path)?;
                println!("File creato: {:?}", db_path);
            }
            format!("sqlite:{}", db_path.to_str().unwrap())
        }
    };
    println!("DB URL: {}", db_url);

    // Inizializza il DB
    let db_pool: SqlitePool = init_db(&db_url).await?;

    println!("✅ DB INIZIALIZZATO! ✅");

    //PROVO DELLE QUERY -> Poi verranno usate nei routes
    /*
    let users = get_all_users(&db_pool).await?;

    println!("Utenti nel DB:");
    for user in users {
        println!("{} - {} - {}", user.u_id, user.username, user.email);
    }
    */

    /*
    // 🔹 PROVA: Inserisci un messaggio di test
    send_message(&db_pool, 1, 1, "Messaggio di test")
        .await
        .expect("Errore nell'invio messaggio");

    println!("✅ Messaggio inviato");

    // 🔹 PROVA: Leggi tutti i messaggi del gruppo 1
    let messages = get_messages_by_group(&db_pool, 1)
        .await
        .expect("Errore nel recupero messaggi");

    println!("📨 Messaggi nel gruppo 1:");
    for msg in messages {
        println!(
            "[{}] {} -> {}",
            msg.sent_at.format("%Y-%m-%d %H:%M:%S"),
            msg.sender_id,
            msg.content
        );
    }
    
    create_group(&db_pool, "Axum Group", 1).await?;
    */

    // Costruisci il router
    let app: Router = routes::build_router(db_pool.clone())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );

    // Indirizzo di bind (porta 8000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running on {}", addr);

    // Avvia server Axum
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;



    Ok(())
}
