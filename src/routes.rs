use crate::data::{PatchUser, User};
use crate::error::DataError;
use crate::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

pub async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Result<Json<Uuid>, DataError> {
    let mut users = state
        .write()
        .expect("RwLock should not be poisoned");

    let uuid = Uuid::new_v4();
    users.insert_user(user.clone(), uuid)?;

    Ok(Json(uuid))
}

pub async fn update_user(
    Path(uuid): Path<Uuid>,
    State(state): State<AppState>,
    Json(patch): Json<PatchUser>,
) -> Result<(), DataError> {
    let mut users = state
        .write()
        .expect("RwLock should not be poisoned");

    users.get_user_mut(&uuid)?.patch(patch);

    Ok(())
}

pub async fn delete_user(
    Path(uuid): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<(), DataError> {
    let mut users = state
        .write()
        .expect("RwLock should not be poisoned");

    users.remove_user(&uuid)?;

    Ok(())
}

pub async fn get_user(
    Path(uuid): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<User>, DataError> {
    let user = state
        .read()
        .expect("RwLock should not be poisoned")
        .get_user(&uuid)?
        .clone();

    Ok(Json(user))
}
