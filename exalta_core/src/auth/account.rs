use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    #[serde(default)]
    #[serde(rename = "Credits")]
    pub credits: String,

    #[serde(default)]
    #[serde(rename = "FortuneToken")]
    pub fortune_token: String,

    #[serde(default)]
    #[serde(rename = "UnityCampaignPoints")]
    pub unity_campaign_points: String,

    #[serde(default)]
    #[serde(rename = "NextCharSlotPrice")]
    pub next_char_slot_price: String,

    #[serde(default)]
    #[serde(rename = "EarlyGameEventTracker")]
    pub early_game_event_tracker: String,

    #[serde(default)]
    #[serde(rename = "AccountId")]
    pub account_id: String,

    #[serde(default)]
    #[serde(rename = "CreationTimestamp")]
    pub creation_timestamp: String,

    #[serde(default)]
    #[serde(rename = "FavoritePet")]
    pub favorite_pet: Option<String>,

    #[serde(default)]
    #[serde(rename = "HasGifts")]
    pub has_gifts: String,

    #[serde(default)]
    #[serde(rename = "DecaSignupPopup")]
    pub deca_signup_popup: String,

    #[serde(default)]
    #[serde(rename = "MaxNumChars")]
    pub max_num_chars: String,

    #[serde(default)]
    #[serde(rename = "MutedUntil")]
    pub muted_until: String,

    #[serde(default)]
    #[serde(rename = "LastServer")]
    pub last_server: String,

    #[serde(default)]
    #[serde(rename = "TeleportWait")]
    pub teleport_wait: String,

    #[serde(default)]
    #[serde(rename = "Originating")]
    pub originating: String,

    #[serde(default)]
    #[serde(rename = "PetYardType")]
    pub pet_yard_type: String,

    #[serde(default)]
    #[serde(rename = "ForgeFireEnergy")]
    pub forge_fire_energy: String,

    #[serde(default)]
    #[serde(rename = "ForgeFireBlueprints")]
    pub forge_fire_blueprints: String,

    #[serde(default)]
    #[serde(rename = "Campaigns")]
    pub campaigns: Campaigns,

    #[serde(default)]
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(default)]
    #[serde(rename = "NameChosen")]
    pub name_chosen: String,

    #[serde(default)]
    #[serde(rename = "PaymentProvider")]
    pub payment_provider: String,

    #[serde(default)]
    #[serde(rename = "Converted")]
    pub converted: Option<String>,

    #[serde(default)]
    #[serde(rename = "IsAgeVerified")]
    pub is_age_verified: String,

    #[serde(default)]
    #[serde(rename = "SecurityQuestions")]
    pub security_questions: SecurityQuestions,

    #[serde(default)]
    #[serde(rename = "Stats")]
    pub stats: Stats,

    #[serde(default)]
    #[serde(rename = "Guild")]
    pub guild: Option<Guild>,

    #[serde(default)]
    #[serde(rename = "AccessToken")]
    pub access_token: String,

    #[serde(default)]
    #[serde(rename = "AccessTokenTimestamp")]
    pub access_token_timestamp: String,

    #[serde(default)]
    #[serde(rename = "AccessTokenExpiration")]
    pub access_token_expiration: String,
}

#[derive(Serialize, Deserialize)]
pub struct Campaigns {
    #[serde(default)]
    #[serde(rename = "CampaignProgress")]
    campaign_progress: Option<Vec<CampaignProgress>>,
}
impl Default for Campaigns {
    fn default() -> Self {
        Campaigns {
            campaign_progress: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CampaignProgress {
    #[serde(default)]
    #[serde(rename = "Points")]
    pub points: String,
}

#[derive(Serialize, Deserialize)]
pub struct Guild {
    #[serde(default)]
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(default)]
    #[serde(rename = "Rank")]
    pub rank: String,

    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SecurityQuestions {
    #[serde(default)]
    #[serde(rename = "HasSecurityQuestions")]
    pub has_security_questions: String,

    #[serde(default)]
    #[serde(rename = "ShowSecurityQuestionsDialog")]
    pub show_security_questions_dialog: String,

    #[serde(default)]
    #[serde(rename = "SecurityQuestionsKeys")]
    pub security_questions_keys: SecurityQuestionsKeys,
}
impl Default for SecurityQuestions {
    fn default() -> Self {
        SecurityQuestions {
            has_security_questions: String::default(),
            show_security_questions_dialog: String::default(),
            security_questions_keys: SecurityQuestionsKeys::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SecurityQuestionsKeys {
    #[serde(default)]
    #[serde(rename = "SecurityQuestionsKey")]
    pub security_questions_key: Vec<String>,
}
impl Default for SecurityQuestionsKeys {
    fn default() -> Self {
        Self {
            security_questions_key: Vec::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    #[serde(default)]
    #[serde(rename = "ClassStats")]
    pub class_stats: Vec<ClassStat>,

    #[serde(default)]
    #[serde(rename = "BestCharFame")]
    pub best_char_fame: String,

    #[serde(default)]
    #[serde(rename = "TotalFame")]
    pub total_fame: String,

    #[serde(default)]
    #[serde(rename = "Fame")]
    pub fame: String,
}
impl Default for Stats {
    fn default() -> Self {
        Self {
            class_stats: Vec::default(),
            best_char_fame: String::default(),
            total_fame: String::default(),
            fame: String::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClassStat {
    #[serde(default)]
    #[serde(rename = "BestLevel")]
    pub best_level: String,

    #[serde(default)]
    #[serde(rename = "BestBaseFame")]
    pub best_base_fame: String,

    #[serde(default)]
    #[serde(rename = "BestTotalFame")]
    pub best_total_fame: String,

    #[serde(default)]
    #[serde(rename = "objectType")]
    pub object_type: String,
}

