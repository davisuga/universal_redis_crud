wai_bindgen_rust::export!("universal-redis-crud.wai");
use redis::RedisError;
use std::env;
use universal_redis_crud::CreateError;
use universal_redis_crud::User;

extern crate redis;
pub struct UniversalRedisCrud;

fn store_user(u: &User) -> Result<(), RedisError> {
    // connect to redis
    let client =
        redis::Client::open(env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_owned()))?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let mut hkey = String::from("user::");
    hkey.push_str(&u.email);

    redis::pipe()
        .atomic()
        .hset(&hkey, "emal", u.email.as_bytes())
        .hset(&hkey, "name", u.name.as_bytes())
        .query(&mut con)
}

impl universal_redis_crud::UniversalRedisCrud for UniversalRedisCrud {
    fn create_user(u: User) -> Result<User, CreateError> {
        store_user(&u).map_err(|e| e.to_string()).map(|_| u)
    }
}
