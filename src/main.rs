mod cli;
mod ikill;

#[tokio::main]
async fn main() {
    cli::new().get_matches();

    ikill::run().await;
}
