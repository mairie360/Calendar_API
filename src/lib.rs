// Ids are `u64` in the API and `INT4` in Postgres: convert them with `id_to_sql` / `id_from_sql`
// (API_lib), never with `as`, which silently aliases `2^32 + 1` to `1` (MAIR-422).
#![deny(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

pub mod database;
pub mod endpoints;
pub mod request_log;
