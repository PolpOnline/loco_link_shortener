use loco_rs::prelude::DateTime;
use serde::Serialize;

use crate::models::_entities::{clicks, links};

#[derive(Serialize)]
pub struct InfoLinkView {
    pub original: String,
    pub clicks: Vec<InfoClick>,
    pub created_at: DateTime,
}

#[derive(Serialize)]
pub struct InfoClick {
    pub clicked_at: String,
    pub address: String,
    pub user_agent: Option<String>,
}

impl InfoLinkView {
    pub fn new(link: links::Model, clicks: Vec<clicks::Model>) -> InfoLinkView {
        let original = link.original;
        let created_at = link.created_at;
        let clicks = clicks.into_iter().map(InfoClick::from).collect();

        InfoLinkView {
            original,
            clicks,
            created_at,
        }
    }
}

impl InfoClick {
    pub fn new(clicked_at: String, address: String, user_agent: Option<String>) -> InfoClick {
        InfoClick {
            clicked_at,
            address,
            user_agent,
        }
    }
}

impl From<clicks::Model> for InfoClick {
    fn from(click: clicks::Model) -> Self {
        InfoClick {
            clicked_at: click.clicked_at.to_string(),
            address: click.address,
            user_agent: click.user_agent,
        }
    }
}
