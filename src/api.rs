use axum::{http::StatusCode, routing::{get, post, put, delete}, Json, Router};
use serde::{Deserialize, Serialize};
use axum::extract::State;
use crate::db::KvDb;
use std::sync::{Arc, Mutex};

//these types should be structs to account for CRUD data

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyVal {
    key: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchKey {
    key: String,
}

#[derive(Debug, Serialize)]
pub struct MinResponse {
    result: bool,
}

//create 
pub async fn create_kv(State(state): State<Arc<Mutex<KvDb>>>, Json(payload): Json<KeyVal>) -> (StatusCode, Json<MinResponse>) {
    let key = &payload.key.clone();
    let val = &payload.value.clone();
    let create_res = state.lock().unwrap().create(key, val);
    match create_res {
        Ok(res) => {
            if res {
                (StatusCode::CREATED, Json(MinResponse{result:true}))
            }
            else {
                (StatusCode::BAD_REQUEST, Json(MinResponse{result:false})) //TODO: find a better status code
            }
        }
        Err(_) => {
            (StatusCode::BAD_REQUEST, Json(MinResponse{result:false})) //TODO: find a better status code
        }
    }
}
//read
pub async fn read_kv(State(state): State<Arc<Mutex<KvDb>>>, Json(payload): Json<SearchKey>) -> (StatusCode, Json<KeyVal>){
    let key = &payload.key.clone();
    let read_res = state.lock().unwrap().read(key);
    match read_res {
        Ok((res_key, res_val)) => {
            (StatusCode::OK, Json(KeyVal{key:res_key, value: res_val}))
        }
        Err(_) => {
            (StatusCode::BAD_REQUEST, Json(KeyVal{key: "".to_string(),value:"".to_string()})) //TODO: find a better status code
        }
    }
}
//update
pub async fn update_kv(State(state): State<Arc<Mutex<KvDb>>>, Json(payload): Json<KeyVal>) -> (StatusCode, Json<MinResponse>) {
    let key = &payload.key.clone();
    let val = &payload.value.clone();
    let update_res = state.lock().unwrap().update(key, val);
    match update_res {
        Ok(res) => {
            if res {
                (StatusCode::CREATED, Json(MinResponse{result:true}))
            }
            else {
                (StatusCode::BAD_REQUEST, Json(MinResponse{result:false})) //TODO: find a better status code
            }
        }
        Err(_) => {
            (StatusCode::BAD_REQUEST, Json(MinResponse{result:false})) //TODO: find a better status code
        }
    }
}
//delete
pub async fn delete_kv(State(state): State<Arc<Mutex<KvDb>>>, Json(payload): Json<SearchKey>) -> (StatusCode, Json<MinResponse>) {
    let key = &payload.key.clone();
    let delete_res = state.lock().unwrap().delete(key);
    match delete_res {
        Ok(res) => {
            if res {
                (StatusCode::CREATED, Json(MinResponse{result:true}))
            }
            else {
                (StatusCode::BAD_REQUEST, Json(MinResponse{result:false})) //TODO: find a better status code
            }
        }
        Err(_) => {
            (StatusCode::BAD_REQUEST, Json(MinResponse{result:false})) //TODO: find a better status code
        }
    }
}

pub async fn start_api_server(){
    let state = Arc::new(Mutex::new(KvDb::init("test.db".to_string()).unwrap()));

    let app = Router::new()
    .route("/kv/create", post(create_kv))
    .route("/kv/read",   get(read_kv))
    .route("/kv/update", put(update_kv))
    .route("/kv/delete", delete(delete_kv))
    .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap(); // TODO: look into making this into something like a builder that returns the listner
}