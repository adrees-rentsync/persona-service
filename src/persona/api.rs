use std::convert::{From};

use super::consts::INQUIRY_API_URL;
use super::types::{
    PersonaAcctRefId, PersonaApiKey, PersonaAttributeMap, PersonaConfig, PersonaInquiryData,
    PersonaInquiryMeta, PersonaInquiryPayload, PersonaInquiryResponse, PersonaTemplateId,
};
use reqwest::header::{HeaderMap, HeaderValue};

use reqwest::Method;
use reqwest::RequestBuilder;

pub struct PersonaApi {
    api_key: PersonaApiKey,
}

impl PersonaApi {
    pub fn persona_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        let auth_value = format!("Bearer {}", &self.api_key);
        
        headers.insert("accept", HeaderValue::from_str("application/json").unwrap());
        headers.insert("Persona-Version", HeaderValue::from_str("2023-01-05").unwrap());
        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
        headers.insert("Authorization", HeaderValue::from_str(auth_value.as_str()).unwrap());

        headers
    }

    pub async fn create_inquiry(
        &self,
        account_ref: &PersonaAcctRefId,
        template_id: &PersonaTemplateId,
    ) -> Result<PersonaInquiryResponse, ()> {
        let client = reqwest::Client::new();

        let payload = PersonaInquiryPayload {
            data: PersonaInquiryData {
                attributes: PersonaAttributeMap {
                    inquiry_template_id: template_id.to_string(),
                },
            },
            meta: PersonaInquiryMeta {
                auto_create_account_reference_id: account_ref.to_string(),
            },
        };

        let url = match reqwest::Url::parse(format!("{}", INQUIRY_API_URL).as_str()) {
            Result::Ok(url) => url,
            Result::Err(_) => {
                println!("Parsing Persona API URL failed.");
                return Result::Err(())
            }
        };

        let request = RequestBuilder::from_parts(client, reqwest::Request::new(Method::POST, url))
            .headers(self.persona_headers())
            .json(&payload);

        let response = match request.send().await {
            Ok(response) => response,
            Err(_) => {
                println!("Inquiry Failed.");
                return Result::Err(());
            }
        };

        match response.json::<PersonaInquiryResponse>().await {
            Result::Ok(resp) => Result::Ok(resp),
            Result::Err(e) => {
                println!("{:}", e);
                Result::Err(())
            }
        }
    }
}

impl From<&PersonaConfig> for PersonaApi {
    fn from(config: &PersonaConfig) -> Self {
        Self {
            api_key: config.api_key.clone(),
        }
    }
}
