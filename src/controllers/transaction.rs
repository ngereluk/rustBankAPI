use rocket::response::status;
use rocket::State;
use sea_orm::*;

use crate::entites::{prelude::*, *};
use crate::models::post_transaction_dto::PostTransactionDTO;
use chrono::prelude::*;
use rocket::serde::json::Json;
use std::string::ToString;

#[get("/transaction")]
pub async fn get_all_transactions(db: &State<DatabaseConnection>) -> Json<Vec<transaction::Model>> {
    let db = db as &DatabaseConnection;

    let transactions = Transaction::find().all(db).await.unwrap();

    Json(transactions)
}

#[get("/transaction/<id>")]
pub async fn get_transaction_by_id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Json<Option<transaction::Model>> {
    let db = db as &DatabaseConnection;

    let transaction = Transaction::find_by_id(id).one(db).await.unwrap();

    Json(transaction)
}

#[post("/transaction", format = "application/json", data = "<object>")]
pub async fn create_transaction(
    connection: &State<DatabaseConnection>,
    object: PostTransactionDTO,
) -> status::Accepted<std::option::Option<&str>> {
    let connection = connection as &DatabaseConnection;
    let record = transaction::ActiveModel {
        amount: Set(object.amount.to_owned()),
        sender_user_id: Set(object.sender_user_id.to_owned()),
        sender_account_id: Set(object.sender_account_id.to_owned()),
        recipient_account_id: Set(object.recipient_account_id.to_owned()),
        time_sent: Set(Local::now().to_string().to_owned()),

        ..Default::default()
    };
    let _result = Transaction::insert(record.clone()).exec(connection).await;

    status::Accepted(Some("record created"))
}
