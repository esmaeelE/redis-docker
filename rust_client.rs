// Cargo.toml
// [package]
// name = "redis"
// version = "0.1.0"
// edition = "2024"

// [dependencies]
// redis = "1.0.2"


use redis::Commands;

fn main() {
    // Try opening the connection with the username and password (for Redis 6+ with ACLs)
    let client = match redis::Client::open("redis://myusername:mypassword@localhost:6379") {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to connect to Redis: {}", err);
            return;
        }
    };

    // Try to get a connection to Redis
    let mut con = match client.get_connection() {
        Ok(con) => con,
        Err(err) => {
            eprintln!("Failed to get connection: {}", err);
            return;
        }
    };

    // Set a key-value pair in Redis
    match con.set::<&str, &str, ()>("key", "Hello World") {
        Ok(_) => println!("Key set successfully!"),
        Err(err) => eprintln!("Failed to set key: {}", err),
    }

    // Get the value of the key from Redis
    match con.get::<&str, String>("key") {
        Ok(key) => println!("key: {}", key),
        Err(err) => eprintln!("Failed to get key: {}", err),
    }
}


