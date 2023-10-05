use crate::error::{DataError, DataResult};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

pub type AppState = Arc<RwLock<Users>>;

// Zadanie nie precyzowało rodzaju danych, więc zdecydowałem się na 'bazę danych' użytkowników.
// Takie rzeczy jak logowanie znajdują się poza zakresem zadania,
// więc użytkownik nie posiada hasła, ale mogę dodać logowanie jeśli jest taka potrzeba.
#[derive(Serialize, Clone, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub balance: f64,
}

impl User {
    pub fn patch(&mut self, patch: PatchUser) {
        macro_rules! patch {
            ($self:ident, $patch:ident, $field:ident) => {
                if let Some($field) = $patch.$field {
                    $self.$field = $field;
                };
            };
        }

        patch!(self, patch, first_name);
        patch!(self, patch, last_name);
        patch!(self, patch, balance);
    }
}

#[derive(Deserialize)]
pub struct PatchUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub balance: Option<f64>,
}

pub struct Users {
    inner: HashMap<Uuid, User>,
}

impl Users {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert_user(&mut self, user: User, uuid: Uuid) -> DataResult<()> {
        self.inner.insert(uuid, user);

        Ok(())
    }

    pub fn get_user(&self, uuid: &Uuid) -> DataResult<&User> {
        if let Some(user) = self.inner.get(uuid) {
            Ok(user)
        } else {
            Err(DataError::UserNotFound)
        }
    }

    pub fn get_user_mut(
        &mut self,
        username: &Uuid,
    ) -> DataResult<&mut User> {
        if let Some(user) = self.inner.get_mut(username) {
            Ok(user)
        } else {
            Err(DataError::UserNotFound)
        }
    }

    pub fn remove_user(&mut self, uuid: &Uuid) -> DataResult<()> {
        match self.inner.remove(uuid) {
            Some(_) => Ok(()),
            None => Err(DataError::UserNotFound),
        }
    }
}
