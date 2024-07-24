use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::common::{AppState, EndpointRet, ServerError};

#[get("13/sql")]
async fn test_sql(state: web::Data<AppState>) -> EndpointRet {
    let test_run: i32 = sqlx::query_scalar("SELECT 20231213")
        .fetch_one(&state.pool)
        .await
        .map_err(|_ /*TOOD make the error enum accept text and pass internal error to the server error*/| ServerError::InternalError)?;

    Ok(HttpResponse::Ok().body(test_run.to_string()))
}

#[derive(Deserialize, Debug)]
struct Order {
    // Realistically, all i32 here shoul be usize or other unsigned types
    // but sqlx can bind unsigned
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[post("13/reset")]
async fn reset_orders(state: web::Data<AppState>) -> EndpointRet {
    match sqlx::query("DELETE FROM orders;")
        .execute(&state.pool)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(ServerError::InternalError),
    }
}

#[post("13/orders")]
async fn insert_orders(state: web::Data<AppState>, body: web::Json<Vec<Order>>) -> EndpointRet {
    let orders = body.into_inner();

    let mut transaction = state.pool.begin().await.unwrap(); // handle error
    for order in orders {
        sqlx::query(
            "INSERT INTO orders (id, region_id, gift_name, quantity)
            VALUES ($1, $2, $3, $4);",
        )
        .bind(order.id)
        .bind(order.region_id)
        .bind(order.gift_name)
        .bind(order.quantity)
        .execute(transaction.as_mut())
        .await
        .unwrap();
    }
    transaction.commit().await.unwrap();

    Ok(HttpResponse::Ok().finish())
}

#[get("13/orders/total")]
async fn get_total(state: web::Data<AppState>) -> EndpointRet {
    match sqlx::query_scalar::<sqlx::Postgres, i64>("SELECT SUM(quantity) FROM orders;")
        .fetch_one(&state.pool)
        .await
    {
        Ok(sum) => Ok(HttpResponse::Ok().json(json!({"total": sum}))),
        Err(_) => Err(ServerError::InternalError),
    }
}

#[get("13/orders/popular")]
async fn get_popular(state: web::Data<AppState>) -> EndpointRet {
    match sqlx::query_scalar::<sqlx::Postgres, String>("SELECT gift_name FROM public.orders GROUP BY gift_name ORDER BY SUM(quantity) DESC LIMIT 1").fetch_optional(&state.pool).await {
        Ok(name) => Ok(HttpResponse::Ok().json(json!({"popular": name}))),
        Err(_) => Err(ServerError::InternalError)
    }
}