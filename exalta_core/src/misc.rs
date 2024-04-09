use serde::{Deserialize, Serialize};

use crate::{coll_to_owned, get_base_url, CLIENT, DEFAULT_PARAMS};
use anyhow::Result;

pub async fn init(game_net: Option<&str>, access_token: Option<&str>) -> Result<AppSettings> {
    let mut params = DEFAULT_PARAMS.read().await.clone();

    if let Some(game_net) = game_net {
        params[0].0 = game_net.to_owned();
    }

    if let Some(access_token) = access_token {
        params = [coll_to_owned(vec![("accessToken", access_token)]), params].concat();
    }

    let resp = CLIENT
        .post(
            get_base_url()
                .await
                .join("app/init?platform=standalonewindows64&key=9KnJFxtTvLu2frXv")?,
        )
        .form(&params)
        .send()
        .await?;
    let resp_text = resp.text().await?;
    Ok(quick_xml::de::from_str::<AppSettings>(resp_text.as_str())?)
}

#[derive(Serialize, Deserialize, Debug)]
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

    #[serde(rename = "UGDOpenSubmission")]
    pub ugd_open_submission: String,

    pub forge_max_ingredients: String,

    pub forge_max_energy: String,

    pub forge_initial_energy: String,

    pub forge_daily_energy: String,

    pub build_id: String,

    pub build_hash: String,

    pub build_version: String,

    #[serde(rename = "BuildCDN")]
    pub build_cdn: String,

    pub launcher_build_id: String,

    pub launcher_build_hash: String,

    pub launcher_build_version: String,

    #[serde(rename = "LauncherBuildCDN")]
    pub launcher_build_cdn: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PotionPurchaseCosts {
    pub cost: Vec<String>,
}
