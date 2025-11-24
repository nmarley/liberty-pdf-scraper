use anyhow::{Context, Result};
use base64::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
const API_BASE: &str = "https://libertycf2-svc.smartcmobile.com";

#[derive(Serialize)]
struct GetBillRequest {
    #[serde(rename = "accountNumber")]
    account_number: String,
    #[serde(rename = "companyGroupCode")]
    company_group_code: Option<String>,
}

#[derive(Serialize)]
struct VerifyZipRequest {
    #[serde(rename = "accountNumber")]
    account_number: String,
    #[serde(rename = "zipCode")]
    zip_code: String,
    #[serde(rename = "companyGroupCode")]
    company_group_code: Option<String>,
}

#[derive(Deserialize)]
struct PdfResponse {
    data: PdfData,
}

#[derive(Deserialize)]
struct PdfData {
    #[serde(rename = "contentArray")]
    content_array: String,
}

pub struct LibertyClient {
    client: Client,
    account_number: String,
    zip_code: String,
}

impl LibertyClient {
    pub fn new(account_number: String, zip_code: String) -> Result<Self> {
        let client = Client::builder()
            .cookie_store(true)
            .user_agent(USER_AGENT)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            account_number,
            zip_code,
        })
    }

    pub async fn fetch_bill_pdf(&self) -> Result<Vec<u8>> {
        // Step 1: Initial GetBillDetailPDF request
        let url = format!("{}/CISAPI/api/1/account/GetBillDetailPDF", API_BASE);
        let payload = GetBillRequest {
            account_number: self.account_number.clone(),
            company_group_code: None,
        };

        self.client
            .post(&url)
            .header("accept", "application/json, text/plain, */*")
            .header("content-type", "application/json;charset=UTF-8")
            .header("origin", "https://myaccount.libertyenergyandwater.com")
            .header("referer", "https://myaccount.libertyenergyandwater.com/")
            .header("st", "PL")
            .json(&payload)
            .send()
            .await
            .context("Failed to send initial GetBillDetailPDF request")?;

        // Step 2: VerifyDetailsByZip request
        let url = format!(
            "{}/UserManagementAPI/api/1/users/VerifyDetailsByZip",
            API_BASE
        );
        let payload = VerifyZipRequest {
            account_number: self.account_number.clone(),
            zip_code: self.zip_code.clone(),
            company_group_code: None,
        };

        self.client
            .post(&url)
            .header("accept", "application/json, text/plain, */*")
            .header("content-type", "application/json;charset=UTF-8")
            .header("origin", "https://myaccount.libertyenergyandwater.com")
            .header("referer", "https://myaccount.libertyenergyandwater.com/")
            .header("st", "PL")
            .json(&payload)
            .send()
            .await
            .context("Failed to send VerifyDetailsByZip request")?;

        // Step 3: Second GetBillDetailPDF request (returns PDF)
        let url = format!("{}/CISAPI/api/1/account/GetBillDetailPDF", API_BASE);
        let payload = GetBillRequest {
            account_number: self.account_number.clone(),
            company_group_code: None,
        };

        let response = self
            .client
            .post(&url)
            .header("accept", "application/json, text/plain, */*")
            .header("content-type", "application/json;charset=UTF-8")
            .header("origin", "https://myaccount.libertyenergyandwater.com")
            .header("referer", "https://myaccount.libertyenergyandwater.com/")
            .header("st", "PL")
            .json(&payload)
            .send()
            .await
            .context("Failed to send final GetBillDetailPDF request")?;

        let pdf_response: PdfResponse = response
            .json()
            .await
            .context("Failed to parse JSON response")?;

        let pdf_bytes = BASE64_STANDARD
            .decode(&pdf_response.data.content_array)
            .context("Failed to decode base64 PDF content")?;

        Ok(pdf_bytes)
    }
}
