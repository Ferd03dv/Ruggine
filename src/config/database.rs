use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use sqlx::SqlitePool;

enum status {
    Pending = 0,
    Accepted = 1,
    Rejected = 2,
}

pub async fn init_db(db_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // Creazione delle Tabelle al Primo Avvio
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS user (
        u_id INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT NOT NULL UNIQUE,
        email TEXT NOT NULL UNIQUE,
        password TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS groups (
        g_id INTEGER PRIMARY KEY AUTOINCREMENT,
        name_group TEXT NOT NULL UNIQUE,
        created_by INTEGER NOT NULL,
        FOREIGN KEY(created_by) REFERENCES user(u_id)
    );

    CREATE TABLE IF NOT EXISTS user_group (
        ug_id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        group_id INTEGER NOT NULL,
        is_admin BOOLEAN NOT NULL,
        FOREIGN KEY(user_id) REFERENCES user(u_id),
        FOREIGN KEY(group_id) REFERENCES groups(g_id)
    );

    CREATE TABLE IF NOT EXISTS message (
        m_id INTEGER PRIMARY KEY AUTOINCREMENT,
        content TEXT NOT NULL,
        sent_at DATE NOT NULL,
        sender_id INTEGER NOT NULL,
        group_id INTEGER NOT NULL,
        FOREIGN KEY(sender_id) REFERENCES user(u_id),
        FOREIGN KEY(group_id) REFERENCES groups(g_id)
    );

    CREATE TABLE IF NOT EXISTS invitation (
        i_id INTEGER PRIMARY KEY AUTOINCREMENT,
        status INTEGER NOT NULL CHECK(status IN (0,1,2)),
        sent_at DATE NOT NULL,
        invited_by INTEGER NOT NULL,
        invited_user INTEGER NOT NULL,
        group_id INTEGER NOT NULL,
        FOREIGN KEY(invited_by) REFERENCES user(u_id),
        FOREIGN KEY(invited_user) REFERENCES user(u_id),
        FOREIGN KEY(group_id) REFERENCES groups(g_id)
    );
    "#
    )
        .execute(&pool)
        .await?;
    Ok(pool)
}


