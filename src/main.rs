use mev_magic::run;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let res = run().await;
    println!("{:?}", res);
}
