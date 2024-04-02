use rocket::response::status;

#[get("/health")]
pub async fn health() -> status::Accepted<std::option::Option<String>> {
    status::Accepted(Some("rust banking api is up and running".to_string()))
}
