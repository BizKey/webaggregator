use crate::api::models::{
    PgConnection, PgStatStatements, PgStatTableSize, PgTableIndex, PgTableInfo,
};
use crate::api::templates::PgTemplate;
use actix_web::{HttpResponse, Result as ActixResult, web};
use askama::Template;

use sqlx::PgPool;
use std::time::Instant;

pub async fn pg(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start: Instant = Instant::now();

    let pg_stats_connections: Vec<PgConnection> = sqlx::query_as::<_, PgConnection>(
        r#"
        SELECT count(*) AS total_connections, count(*)
        FILTER (WHERE state = 'active') AS active_connections
        FROM pg_stat_activity;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let pg_stats_table_info: Vec<PgTableInfo> = sqlx::query_as::<_, PgTableInfo>(
        r#"
        SELECT schemaname, relname, seq_scan, seq_tup_read, idx_scan, idx_tup_fetch, n_tup_ins, n_tup_upd, n_tup_del, n_live_tup, n_dead_tup
        FROM pg_stat_user_tables;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e|{
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let pg_stats_table_index: Vec<PgTableIndex> = sqlx::query_as::<_, PgTableIndex>(
        r#"
        SELECT schemaname, relname, idx_scan, idx_tup_read, idx_tup_fetch
        FROM pg_stat_user_indexes;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let pg_stat_statements: Vec<PgStatStatements> = sqlx::query_as::<_, PgStatStatements>(
        r#"
        SELECT query, calls, total_exec_time, mean_exec_time, rows
        FROM pg_stat_statements
        ORDER BY total_exec_time
        DESC LIMIT 100;
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    let pg_stat_table_size: Vec<PgStatTableSize> = sqlx::query_as::<_, PgStatTableSize>(
            r#"
            SELECT schemaname, relname, pg_size_pretty(pg_total_relation_size(schemaname || '.' || relname)) AS total_size, pg_size_pretty(pg_relation_size(schemaname || '.' || relname)) AS table_size, pg_size_pretty(pg_indexes_size(schemaname || '.' || relname)) AS indexes_size
            FROM pg_stat_user_tables;
            "#,
        )
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e|{
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Template render error")
        })?;

    let elapsed_ms: u128 = start.elapsed().as_millis();

    let html: String = PgTemplate {
        pg_stats_connections,
        pg_stats_table_info,
        pg_stats_table_index,
        pg_stat_statements,
        pg_stat_table_size,
        elapsed_ms,
    }
    .render()
    .map_err(|e| {
        log::error!("Template render error: {}", e);
        actix_web::error::ErrorInternalServerError("Template render error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
