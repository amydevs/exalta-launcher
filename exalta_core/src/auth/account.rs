use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    pub credits: String,

    pub fortune_token: String,

    pub unity_campaign_points: String,

    pub next_char_slot_price: String,

    pub early_game_event_tracker: String,

    pub account_id: String,

    pub creation_timestamp: String,

    pub verified_email: String,

    pub deca_signup_popup: String,

    pub max_num_chars: String,

    pub muted_until: String,

    pub last_server: String,

    pub teleport_wait: String,

    pub originating: String,

    pub pet_yard_type: String,

    pub forge_fire_energy: String,

    pub forge_fire_blueprints: String,

    pub campaigns: String,

    pub name: String,

    pub name_chosen: String,

    pub payment_provider: String,

    pub is_age_verified: String,

    pub security_questions: SecurityQuestions,

    pub stats: Stats,

    pub access_token: String,

    pub access_token_timestamp: String,

    pub access_token_expiration: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SecurityQuestions {
    pub has_security_questions: String,

    pub show_security_questions_dialog: String,

    pub security_questions_keys: SecurityQuestionsKeys,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SecurityQuestionsKeys {
    pub security_questions_key: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Stats {
    pub class_stats: ClassStats,

    pub best_char_fame: String,

    pub total_fame: String,

    pub fame: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ClassStats {
    pub best_level: String,

    pub best_base_fame: String,

    pub best_total_fame: String,

    #[serde(rename = "objectType")]
    pub object_type: String,
}
