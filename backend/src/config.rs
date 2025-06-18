use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub jwt_secret: String,
    pub database_url: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenvy::dotenv().ok();

    Config {
        jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET is not set"),
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL is not set"),
    }
});
