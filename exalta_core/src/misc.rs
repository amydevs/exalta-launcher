use serde::{Serialize, Deserialize};

use crate::{ExaltaClient, DEFAULT_PARAMS};

impl ExaltaClient {
    pub async fn init(
        &self,
        game_net: &str,
        access_token: Option<&str>
    ) -> Result<AppSettings, Box<dyn std::error::Error>> {
        let mut params = DEFAULT_PARAMS.to_vec();
        params[0].0 = game_net;

        if let Some(access_token) = access_token {
            params = [
                vec![("accessToken", access_token)],
                params
            ].concat();
        }

        let resp = self
            .client
            .post(self.base_url.join("app/init")?)
            .form(&params)
            .send()
            .await?;
            let resp_text = resp.text().await?;
        Ok(quick_xml::de::from_str::<AppSettings>(resp_text.as_str())?)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AppSettings {
    pub use_external_payments: String,

    pub max_stackable_potions: String,

    pub potion_purchase_cooldown: String,

    pub potion_purchase_cost_cooldown: String,

    pub potion_purchase_costs: PotionPurchaseCosts,

    pub filter_list: String,

    pub disable_regist: String,

    pub mystery_box_refresh: String,

    pub salesforce_mobile: String,

    pub ugd_open_submission: String,

    pub forge_max_ingredients: String,

    pub forge_max_energy: String,

    pub forge_initial_energy: String,

    pub forge_daily_energy: String,

    pub build_id: String,

    pub build_hash: String,

    pub build_version: String,

    pub build_cdn: String,

    pub launcher_build_id: String,

    pub launcher_build_hash: String,

    pub launcher_build_version: String,

    pub launcher_build_cdn: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PotionPurchaseCosts {
    pub cost: Vec<String>,
}
