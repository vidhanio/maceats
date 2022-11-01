mod cache;
pub mod locations;
mod macros;
pub mod restaurants;

use cache::Cache;
use macros::handlers;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| Mutex::new(Cache::new()));
