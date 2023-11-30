use std::env;

use serde::{Serialize, Deserialize};

//config field types
pub type PersonaApiKey = String;
pub type PersonaUrlTemplate = String;

//persona destructuring types

//account
pub type PersonaAcctRefId = String;

//inquiry
pub type PersonaInquiryId = String;

//template
pub type PersonaTemplateId = String;

//general
pub type PersonaType = String;

#[derive(Serialize)]
pub struct PersonaAttributeMap {
    pub inquiry_template_id: PersonaTemplateId,
}

#[derive(Serialize)]
pub struct PersonaInquiryData {
    pub attributes: PersonaAttributeMap,
}

#[derive(Serialize)]
pub struct PersonaInquiryMeta {
    pub auto_create_account_reference_id: PersonaAcctRefId,
}

#[derive(Serialize)]
pub struct PersonaInquiryPayload {
    pub data: PersonaInquiryData,
    pub meta: PersonaInquiryMeta,
}

pub struct PersonaInquiry {
    r#type: PersonaType,
    id: PersonaInquiryId,
}

pub struct PersonaConfig {
    pub api_key: PersonaApiKey,
    pub template: PersonaTemplateId,
}

impl PersonaConfig {
    pub fn new(template: Option<PersonaTemplateId>) -> Self {
        Self {
            api_key: get_api_key(),
            template: match template {
                Some(tmpl) => tmpl,
                None => get_default_template(),
            },
        }
    }
}

pub fn get_api_key() -> String {
    match env::var("API_KEY") {
        Ok(val) => val,
        Err(_e) => panic!("API KEY NOT SET!"),
    }
}

pub fn get_default_template() -> String {
    match env::var("DEFAULT_TEMPLATE") {
        Ok(val) => val,
        Err(_e) => panic!("DEFAULT TEMPLATE NOT SET!"),
    }
}

#[derive(Serialize, Deserialize)]
pub struct PersonaInquiryResponseData {
        pub r#type: PersonaType,
        pub id: PersonaInquiryId
}

#[derive(Serialize, Deserialize)]
pub struct PersonaInquiryResponse {
    pub data: PersonaInquiryResponseData
}
