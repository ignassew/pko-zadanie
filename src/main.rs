mod data;
mod error;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::{Arc, RwLock};

use data::{AppState, Users};
use routes::{create_user, delete_user, get_user, update_user};

#[tokio::main]
async fn main() {
    // Zdecydowałem się na użycie 'RwLock' a nie 'Mutex', pozwalając na wiele odczytów naraz.
    // Dalej nie jest to idealne rozwiązanie, ponieważ użytkownik modyfikując swoje dane blokuje
    // dostęp do danych wszystkim innym użytkownikowi.
    //
    // Idealnie każdy użytkownik powinien mieć swój własny 'RwLock', ale takie rozwiązanie utrudnia
    // tworzenie nowych / usuwanie użytkowników (dostęp do całej 'HashMap' byłby read-only).
    // Jeśli jest to potrzebne, proszę o informację i mogę zaimplementować takie rozwiązanie.
    //
    // Używam RwLock ze std::sync zamiast asynchronicznego odpowiednika z tokio::sync,
    // ponieważ lock nie jest utrzymywany między punktami '.await'
    let state: AppState = Arc::new(RwLock::new(Users::new()));

    // axum jest moim preferowanym serwerem HTTP. Jest tworzony przez zespół tokio.
    let app = Router::new()
        .route("/user", post(create_user))
        .route(
            "/user/:uuid",
            get(get_user)
                .patch(update_user)
                .delete(delete_user),
        )
        .with_state(state);

    let addr = "127.0.0.1:2137";
    println!("Starting server at {addr}");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Server should start");
}
