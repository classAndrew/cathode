use axum::{
    routing::{get, post},
    Json, Router, extract::State,
};
use tokio::sync::Mutex;
use std::{net::SocketAddr, sync::Arc, fmt::Debug, collections::HashMap, time::SystemTime, env};

use sqlx::{mysql::MySqlPool};
mod models;
mod errors;
use models::c2_s::*;
use models::s2_c::*;
use errors::CathodeError;
use dotenv;

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Config {
    db_user: String,
    db_pass: String,
    db_host: String,
    db_name: String,
    cathode_host: String
}

impl Config {
    const WAR_SUBMIT_CD: i32 = 20;
}

#[derive(Debug)]
struct CathodeState {
    db_pool: MySqlPool,
    // tower_cache so players don't submit duplicated 
    tower_cache: HashMap<Tower, i32>,
    config: Config
}

async fn get_app<'a>() -> Result<(Arc<Mutex<CathodeState>>, Router), CathodeError> {
    dotenv::dotenv().ok();

    // read configuration file
    let config = Config { 
        db_user: env::var("DB_USER")?,
        db_pass: env::var("DB_PASS")?,
        db_host: env::var("DB_HOST")?,
        db_name: env::var("DB_NAME")?,
        cathode_host: env::var("CATHODE_HOST")?
    };

    let db_url = format!("mysql://{0}:{1}@{2}/{3}", config.db_user, config.db_pass, config.db_host, config.db_name);

    // pool is thread-safe so do not need Mutex guard. 
    // Arc not needed since it's already one and is incrementing on .clone()
    let pool = MySqlPool::connect(&db_url).await?;
    let state = Arc::new(Mutex::new(CathodeState { db_pool: pool, tower_cache: HashMap::new(), config: config }));

    Ok((state.clone(), Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/submit_war_attempt", post(submit_war_attempt))
        .with_state(state)))
}

#[tokio::main]
async fn main() -> std::result::Result<(), CathodeError>{
    // initialize tracing
    tracing_subscriber::fmt::init();

    let (state, app) = get_app().await?;

    let addr = {
        let guard = state.lock().await;
        (*guard).config.cathode_host.parse::<SocketAddr>()?
    };

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Cathode Server"
}

async fn submit_war_attempt(
    State(state): State<Arc<Mutex<CathodeState>>>,
    Json(payload): Json<SubmitWarAttemptC2S>
) -> Result<Json<SubmitWarAttemptS2C>, CathodeError> {
    let mut guard = state.lock().await;
    let db_pool = (*guard).db_pool.clone();
    let tower_cache = &mut (*guard).tower_cache;

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as i32;

    let (is_fresh_tower, tower_id) = match tower_cache.get(&payload.tower) {
        Some(prev_time) => {
            // if player A and B enter war at the same time, only insert one tower.
            // discern between wars using `WAR_SUBMIT_CD` seconds
            if now - prev_time > Config::WAR_SUBMIT_CD { 
                tower_cache.insert(payload.tower.clone(), now);
                (true, now)
            } else { (false, *prev_time) }
        }

        None => {
            tower_cache.insert(payload.tower.clone(), now);
            (true, now)
        }
    };

    // insert tower record
    if is_fresh_tower {
        sqlx::query("INSERT INTO war_towers (tower_id, owner, territory, health, defense, attack_speed, damage) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(tower_id)
        .bind(payload.tower.owner)
        .bind(payload.tower.territory)
        .bind(payload.tower.health)
        .bind(payload.tower.defense)
        .bind(payload.tower.attack_speed)
        .bind(payload.tower.damage)
        .execute(&db_pool)
        .await?;
    }

    sqlx::query("INSERT INTO war_attempts (time, name, uuid, class, tower_id) VALUES (?, ?, ?, ?, ?)")
        .bind(now)
        .bind(payload.name)
        .bind(payload.uuid)
        .bind(payload.class)
        .bind(tower_id)
        .execute(&db_pool)
        .await?;

    Ok(Json(SubmitWarAttemptS2C::new(tower_id, None)))
}