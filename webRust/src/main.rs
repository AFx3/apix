/*use axum::{
    routing::get,
    extract::{Json, Path},
    Router,
};
use std::{net::SocketAddr};
use axum::response::IntoResponse;


#[derive(Debug, serde::Serialize)]
struct Kpi {
    name: String,
    id: u8,
    formula: String,
    unit: String,
    counter: u32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/kpis", get(move || kpis_handler(create_kpis())))
        .route("/kpi/:name", get(|params| get_kpi(params)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

fn create_kpis() -> Vec<Kpi> {
    let kpi1 = Kpi {
        name: String::from("Kpi1"),
        id: 1,
        formula: String::from("a+b"),
        unit: String::from("m/s"),
        counter: 0,
    };

    let kpi2 = Kpi {
        name: String::from("Kpi2"),
        id: 2,
        formula: String::from("c/d"),
        unit: String::from("k/m"),
        counter: 0,
    };

    let kpi3 = Kpi {
        name: String::from("Kpi3"),
        id: 3,
        formula: String::from("e+f"),
        unit: String::from("knods"),
        counter: 0,
    };

    vec![kpi1, kpi2, kpi3]
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("KPI Center"),
    })
}

async fn kpis_handler(kpis: Vec<Kpi>) -> impl IntoResponse {
    Json(kpis).into_response()
}


async fn get_kpi(Path((name,)): Path<(String,)>) -> Json<Message> {
    if let Some(kpi) = create_kpis().iter().find(|&k| k.name == name) {
        Json(Message {
            message: serde_json::to_string(kpi).unwrap(),
        })
    } else {
        Json(Message {
            message: format!("KPI {} not found", name),
        })
    }
}
*/
use axum::{
    routing::get,
    extract::{Json, Path},
    Router,
};
use std::{net::SocketAddr};
use axum::response::IntoResponse;
use mongodb::{Client, options::ClientOptions};


#[derive(Debug, serde::Serialize)]
struct Kpi {
    name: String,
    id: u8,
    formula: String,
    unit: String,
    counter: u32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/kpis", get(move || kpis_handler(create_kpis())))
        .route("/kpi/:name", get(|params| get_kpi(params)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

fn create_kpis() -> Vec<Kpi> {
    let kpi1 = Kpi {
        name: String::from("Kpi1"),
        id: 1,
        formula: String::from("a+b"),
        unit: String::from("m/s"),
        counter: 0,
    };

    let kpi2 = Kpi {
        name: String::from("Kpi2"),
        id: 2,
        formula: String::from("c/d"),
        unit: String::from("k/m"),
        counter: 0,
    };

    let kpi3 = Kpi {
        name: String::from("Kpi3"),
        id: 3,
        formula: String::from("e+f"),
        unit: String::from("knods"),
        counter: 0,
    };

    vec![kpi1, kpi2, kpi3]
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("KPI Center"),
    })
}

async fn kpis_handler(kpis: Vec<Kpi>) -> impl IntoResponse {
    Json(kpis).into_response()
}


async fn get_kpi(Path((name,)): Path<(String,)>) -> Json<Message> {
    if let Some(kpi) = create_kpis().iter().find(|&k| k.name == name) {
        Json(Message {
            message: serde_json::to_string(kpi).unwrap(),
        })
    } else {
        Json(Message {
            message: format!("KPI {} not found", name),
        })
    }
}