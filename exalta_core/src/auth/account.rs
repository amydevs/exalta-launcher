use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "Credits")]
    pub credits: String,

    #[serde(rename = "FortuneToken")]
    pub fortune_token: String,

    #[serde(rename = "UnityCampaignPoints")]
    pub unity_campaign_points: String,

    #[serde(rename = "NextCharSlotPrice")]
    pub next_char_slot_price: String,

    #[serde(rename = "EarlyGameEventTracker")]
    pub early_game_event_tracker: String,

    #[serde(rename = "AccountId")]
    pub account_id: String,

    #[serde(rename = "CreationTimestamp")]
    pub creation_timestamp: String,

    #[serde(rename = "FavoritePet")]
    pub favorite_pet: Option<String>,

    #[serde(rename = "HasGifts")]
    pub has_gifts: String,

    #[serde(rename = "DecaSignupPopup")]
    pub deca_signup_popup: String,

    #[serde(rename = "MaxNumChars")]
    pub max_num_chars: String,

    #[serde(rename = "MutedUntil")]
    pub muted_until: String,

    #[serde(rename = "LastServer")]
    pub last_server: String,

    #[serde(rename = "TeleportWait")]
    pub teleport_wait: String,

    #[serde(rename = "Originating")]
    pub originating: String,

    #[serde(rename = "PetYardType")]
    pub pet_yard_type: String,

    #[serde(rename = "ForgeFireEnergy")]
    pub forge_fire_energy: String,

    #[serde(rename = "ForgeFireBlueprints")]
    pub forge_fire_blueprints: String,

    #[serde(rename = "Campaigns")]
    pub campaigns: Campaigns,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "NameChosen")]
    pub name_chosen: String,

    #[serde(rename = "PaymentProvider")]
    pub payment_provider: String,

    #[serde(rename = "Converted")]
    pub converted: Option<String>,

    #[serde(rename = "IsAgeVerified")]
    pub is_age_verified: String,

    #[serde(rename = "SecurityQuestions")]
    pub security_questions: SecurityQuestions,

    #[serde(rename = "Stats")]
    pub stats: Stats,

    #[serde(rename = "Guild")]
    pub guild: Option<Guild>,

    #[serde(rename = "AccessToken")]
    pub access_token: String,

    #[serde(rename = "AccessTokenTimestamp")]
    pub access_token_timestamp: String,

    #[serde(rename = "AccessTokenExpiration")]
    pub access_token_expiration: String,
}

#[derive(Serialize, Deserialize)]
pub struct Campaigns {
    #[serde(rename = "CampaignProgress")]
    campaign_progress: Option<Vec<CampaignProgress>>,
}

#[derive(Serialize, Deserialize)]
pub struct CampaignProgress {
    #[serde(rename = "Points")]
    pub points: String,
}

#[derive(Serialize, Deserialize)]
pub struct Guild {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Rank")]
    pub rank: String,

    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SecurityQuestions {
    #[serde(rename = "HasSecurityQuestions")]
    pub has_security_questions: String,

    #[serde(rename = "ShowSecurityQuestionsDialog")]
    pub show_security_questions_dialog: String,

    #[serde(rename = "SecurityQuestionsKeys")]
    pub security_questions_keys: SecurityQuestionsKeys,
}

#[derive(Serialize, Deserialize)]
pub struct SecurityQuestionsKeys {
    #[serde(rename = "SecurityQuestionsKey")]
    pub security_questions_key: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "ClassStats")]
    pub class_stats: Vec<ClassStat>,

    #[serde(rename = "BestCharFame")]
    pub best_char_fame: String,

    #[serde(rename = "TotalFame")]
    pub total_fame: String,

    #[serde(rename = "Fame")]
    pub fame: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClassStat {
    #[serde(rename = "BestLevel")]
    pub best_level: String,

    #[serde(rename = "BestBaseFame")]
    pub best_base_fame: String,

    #[serde(rename = "BestTotalFame")]
    pub best_total_fame: String,

    #[serde(rename = "objectType")]
    pub object_type: String,
}

