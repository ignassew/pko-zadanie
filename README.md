# Zadanie dla PKOBP
## Instalacja
1. Wpisz `cargo run` w konsoli.
2. Serwer powinien działać na 127.0.0.1:2137

## Użytkowanie

### POST /user
Tworzy nowego użytkownika.
- Request: Dane użytkownika w formacie JSON (e.g. `{"first_name": "Andrzej", "last_name": "Głębocki", "balance": 1000000}`)
- Response: Zwraca uuid w formacie JSON. (e.g. `"b2e7a780-e392-4104-9ef3-954b8176f4da"`), status code 200

### GET /user/:uuid
Wysyła dane użytkownika.
- Request: uuid w URL
- Response: Zwraca dane użytkownika w formacie JSON. (e.g. `{"first_name": "Andrzej", "last_name": "Głębocki", "balance": 1_000_000}`), status code 200

### PATCH /user/:uuid
Aktualizuje dane użytkownika.
- Request: Dane użytkownika do zaaktualizowania w formacie JSON (e.g. `{"first_name": "Ignacy"}`), uuid w URL
- Response: status code 200

### DELETE /user/:uuid
Usuwa użytkownika.
- Request: uuid w URL
- Response: status code 200
