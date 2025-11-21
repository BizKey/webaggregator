use crate::models::{PgConnection, PgStatStatements, PgStatTableSize, PgTableIndex, PgTableInfo};
use crate::templates::PgTemplate;
use actix_web::{HttpResponse, Result, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;
pub async fn pg(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // pg

    // time start
    let start = Instant::now();

    let pg_stats_connections = sqlx::query_as::<_, PgConnection>(
        "SELECT count(*) AS total_connections, count(*) FILTER (WHERE state = 'active') AS active_connections FROM pg_stat_activity;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let pg_stats_table_info = sqlx::query_as::<_, PgTableInfo>(
        "SELECT schemaname, relname, seq_scan, seq_tup_read, idx_scan, idx_tup_fetch, n_tup_ins, n_tup_upd, n_tup_del, n_live_tup, n_dead_tup FROM pg_stat_user_tables;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let pg_stats_table_index = sqlx::query_as::<_, PgTableIndex>(
        "SELECT schemaname, relname, idx_scan, idx_tup_read, idx_tup_fetch FROM pg_stat_user_indexes;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let pg_stat_statements = sqlx::query_as::<_, PgStatStatements>(
        "SELECT query, calls, total_exec_time, mean_exec_time, rows FROM pg_stat_statements ORDER BY total_exec_time DESC LIMIT 100;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let pg_stat_table_size = sqlx::query_as::<_, PgStatTableSize>(
        "SELECT schemaname, relname, pg_size_pretty(pg_total_relation_size(schemaname || '.' || relname)) AS total_size, pg_size_pretty(pg_relation_size(schemaname || '.' || relname)) AS table_size, pg_size_pretty(pg_indexes_size(schemaname || '.' || relname)) AS indexes_size FROM pg_stat_user_tables;",
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let template = PgTemplate {
        pg_stats_connections: pg_stats_connections,
        pg_stats_table_info: pg_stats_table_info,
        pg_stats_table_index: pg_stats_table_index,
        pg_stat_statements: pg_stat_statements,
        pg_stat_table_size: pg_stat_table_size,
        elapsed_ms: start.elapsed().as_millis(),
    };
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}
