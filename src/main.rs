use anyhow::{Context, Result};
use chrono::Local;
use liberty_pdf_scraper::LibertyClient;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let account_number =
        env::var("ACCOUNT_NUMBER").context("ACCOUNT_NUMBER not found in .env file")?;
    let zip_code = env::var("ZIP").context("ZIP not found in .env file")?;

    println!("Fetching bill PDF for account {}...", account_number);

    let client = LibertyClient::new(account_number, zip_code)?;
    let pdf_bytes = client.fetch_bill_pdf().await?;

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let filename = format!("bill_{}.pdf", timestamp);

    fs::write(&filename, pdf_bytes).context("Failed to write PDF file")?;

    println!("Successfully saved PDF to: {}", filename);

    Ok(())
}
