# Liberty PDF Scraper

A Rust library and CLI tool to download electric bill PDFs from Liberty Energy and Water's customer portal.

## Why?

The Liberty Energy portal renders bill PDFs as blob URLs (`blob:https://...`) in the browser. When attempting to download via the browser's save function, the download often fails. The only reliable way to save the PDF through the browser is via Print → Save as PDF, which is cumbersome.

This tool bypasses the browser entirely by calling the underlying API directly, decoding the base64-encoded PDF response, and saving it to disk.

## Prerequisites

- Rust 1.85+
- Liberty Energy account number
- ZIP code associated with your account

## Setup

1. Clone this repository
2. Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```
3. Edit `.env` and add your credentials:
   ```
   ACCOUNT_NUMBER=your_account_number
   ZIP=your_zip_code
   ```

## Usage

```bash
cargo run
```

The PDF will be saved in the current directory with a timestamp: `bill_YYYY-MM-DD_HH-MM-SS.pdf`

## How It Works

The scraper makes three API calls:

1. Initial request to set up the session
2. ZIP code verification (establishes cookies)
3. Final request that returns the base64-encoded PDF

The PDF is then decoded and saved to disk.

## Disclaimer

This tool is for personal use to access your own billing data. Use in accordance with Liberty Energy's Terms of Service. Not affiliated with or endorsed by Liberty Energy and Water.

## License

MIT
