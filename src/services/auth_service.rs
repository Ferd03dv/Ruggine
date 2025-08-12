use sqlx::SqlitePool;
use crate::models::User;
use crate::repositories::user_repository;

#[derive(Debug)]
pub enum AuthError {
    UserAlreadyExists(String),
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for AuthError {
    fn from(error: sqlx::Error) -> Self {
        AuthError::DatabaseError(error)
    }
}

/// Autentica un utente controllando username e password
pub async fn authenticate_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<Option<User>, sqlx::Error> {
    match user_repository::get_user_by_username(pool, username).await? {
        Some(user) => {
            // Confronta la password in chiaro (senza hash)
            if user.password == password {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

/// Registra un nuovo utente
pub async fn register_user(
    pool: &SqlitePool,
    username: &str,
    email: &str,
    password: &str,
) -> Result<i64, AuthError> {
    // Controlla se l'username esiste già
    if let Some(_) = user_repository::get_user_by_username(pool, username).await? {
        return Err(AuthError::UserAlreadyExists("Username".to_string()));
    }

    // Controlla se l'email esiste già
    if let Some(_) = user_repository::get_user_by_email(pool, email).await? {
        return Err(AuthError::UserAlreadyExists("Email".to_string()));
    }

    // Inserisce l'utente (password in chiaro)
    user_repository::insert_user(pool, username, email, password).await?;

    // Recupera l'utente appena creato per ottenere l'ID
    match user_repository::get_user_by_username(pool, username).await? {
        Some(user) => Ok(user.u_id),
        None => Err(AuthError::DatabaseError(sqlx::Error::RowNotFound)),
    }
}
