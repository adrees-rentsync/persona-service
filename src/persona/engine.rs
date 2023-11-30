use super::api::PersonaApi;
use super::consts::URL_TEMPLATE;
use super::types::{PersonaAcctRefId, PersonaConfig, PersonaInquiryId, PersonaUrlTemplate, PersonaInquiryResponse};

pub struct PersonaEngine {
    config: PersonaConfig,
    api: PersonaApi,
}

impl PersonaEngine {
    pub fn new(config: PersonaConfig, api: PersonaApi) -> Self {
        Self {
            config: config,
            api: api,
        }
    }

    pub async fn create_inquiry(&self, refid: PersonaAcctRefId) -> Result<PersonaInquiryResponse, ()> {
        self.api.create_inquiry(&refid, &self.config.template).await
    }

    pub fn get_inquiry_url(
        &self,
        inquiry: PersonaInquiryId,
        template: Option<PersonaUrlTemplate>,
    ) -> String {
        let url_template = match template {
            Some(tmpl) => tmpl,
            None => URL_TEMPLATE.to_string(),
        };

        format!("{}{}", url_template, inquiry)
    }
}

pub async fn get_persona_engine() -> PersonaEngine {
    let config = PersonaConfig::new(None);
    let api = PersonaApi::from(&config);
    PersonaEngine::new(config, api)
}
