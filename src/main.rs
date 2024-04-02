use rocket::launch;
use sea_orm::*;

mod controllers;
mod entites;
mod models;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let connection = match Database::connect(format!(
        "postgresql://postgres:password@localhost:5432/postgres"
    ))
    .await
    {
        Ok(connection) => connection,
        Err(e) => {
            panic!("Error connecting to database: {}", e)
        }
    };
    rocket::build().manage(connection).mount(
        "/",
        routes![
            controllers::account::get_all_accounts,
            controllers::account::create_account,
            controllers::account::get_account_by_id,
            controllers::transaction::get_all_transactions,
            controllers::transaction::create_transaction,
            controllers::transaction::get_transaction_by_id,
            controllers::user::get_all_users,
            controllers::user::get_user_by_id,
            controllers::user::create_user,
            controllers::health::health
        ],
    )
}
