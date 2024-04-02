use crate::entites::{prelude::*, *};
use crate::models::post_account_dto::PostAccountDTO;
use chrono::prelude::*;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::*;
use std::string::ToString;

#[get("/account")]
pub async fn get_all_accounts(db: &State<DatabaseConnection>) -> Json<Vec<account::Model>> {
    let db = db as &DatabaseConnection;

    let accounts = Account::find().all(db).await.unwrap();

    Json(accounts)
}

#[get("/account/<id>")]
pub async fn get_account_by_id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Json<Option<account::Model>> {
    let db = db as &DatabaseConnection;

    let account = Account::find_by_id(id).one(db).await.unwrap();

    Json(account)
}

#[post("/account", format = "application/json", data = "<object>")]
pub async fn create_account(
    connection: &State<DatabaseConnection>,
    object: PostAccountDTO,
) -> status::Accepted<std::option::Option<&str>> {
    let connection = connection as &DatabaseConnection;
    let record = account::ActiveModel {
        account_balance: Set(object.account_balance.to_owned()),
        updated_at: Set(Local::now().to_string().to_owned()),
        created_at: Set(Local::now().to_string().to_owned()),
        deleted: Set(false.to_owned()),
        deleted_at: Set(Local::now().to_string().to_owned()),

        ..Default::default()
    };
    let _result = Account::insert(record.clone()).exec(connection).await;

    status::Accepted(Some("record created"))
}
