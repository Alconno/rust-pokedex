
use serde::{Deserialize, Serialize};
use validr::{error::ValidationError, modifier_lowercase, modifier_uppercase, rule_in, Modifier, Rule, Validation};

const SORTABLE: &'static [&'static str; 2] = &[
    "created_at",
    "updated_at",
];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestPokedexAttributes {
    pub sort_by: Option<String>,
    pub sort: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokedexAttributes {
    pub sort_by: String,
    pub sort: String,
    pub page: i64,
    pub per_page: i64,
}

impl Validation for RequestPokedexAttributes{
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec! [
            modifier_uppercase!(sort),
            modifier_lowercase!(sort_by),
        ]
    }

    fn rules(&self) -> Vec<validr::Rule<Self>> {
        vec![
            rule_in!(sort_by, SORTABLE.to_vec()),
            rule_in!(sort, ["ASC".to_string(), "DESC".to_string()].to_vec()),
            Rule::new("page_range", |obj: &Self, error: &mut ValidationError| {
                if let Some(page) = obj.page {
                    if page < 1 || page > i64::MAX {
                        error.add("Page must be between 1 and i64::MAX.");
                    }
                }
            }),
    
            Rule::new("per_page_range", |obj: &Self, error: &mut ValidationError| {
                if let Some(per_page) = obj.per_page {
                    if per_page < 1 || per_page > i64::MAX {
                        error.add("Per page must be between 1 and i64::MAX.");
                    }
                }
            }),
        ]
    }
}

impl From<RequestPokedexAttributes> for PokedexAttributes{
    fn from(source: RequestPokedexAttributes) -> Self {
        Self { 
            sort_by: source.sort_by.unwrap_or("first_name".to_string()), 
            sort: source.sort.unwrap_or("DESC".to_string()), 
            page: source.page.unwrap_or(1), 
            per_page: source.per_page.unwrap_or(3), 
        }
    }
}