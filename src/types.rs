use std::convert::{From, Into};
use std::fmt;

use serde::{Serialize, Deserialize};
use serde_json::{Map as JsonMap, Value};

use crate::consts::{RENTALSCA, RENTBOARD};
use crate::persona::types::PersonaAcctRefId;

#[derive(Clone)]
pub enum SiteSource {
    RentalsCa,
    RentBoard,
}

#[derive(Deserialize, fmt::Debug)]
pub struct PersonaUrlPayload {
    id: String,
    src: String,
}

pub struct UserId {
    id: String,
    source: SiteSource,
}

impl UserId {
    fn get_persona_refid(&self) -> String {
        format!("{}-{}", self.id, source_to_string(&self.source))
    }
}

impl From<PersonaUrlPayload> for UserId {
    fn from(payload: PersonaUrlPayload) -> Self {
        UserId {
            id: payload.id,
            source: string_to_source(&payload.src).unwrap(),
        }
    }
}

impl Into<PersonaAcctRefId> for UserId {
    fn into(self) -> PersonaAcctRefId {
        self.get_persona_refid()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_persona_refid())
    }
}

fn string_to_source(source_string: &String) -> Option<SiteSource> {
    match source_string.as_str() {
        RENTALSCA => Some(SiteSource::RentalsCa),
        RENTBOARD => Some(SiteSource::RentBoard),
        &_ => None
    }
}

fn source_to_string(source: &SiteSource) -> String {
    match source {
        SiteSource::RentalsCa => RENTALSCA.to_string(),
        SiteSource::RentBoard => RENTBOARD.to_string(),
    }
}

#[derive(Serialize)]
pub struct RcaResponse {
    pub meta: JsonMap<String, Value>,
    pub data: JsonMap<String, Value>,
}
