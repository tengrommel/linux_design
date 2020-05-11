use dotenv;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    dbg!(db_url);
    println!("hello world!");
}
