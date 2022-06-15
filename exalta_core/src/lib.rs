use auth::{account::Account, AuthController};
use reqwest::{Client, Url, header::{HeaderMap, HeaderValue}};

pub mod config;
pub mod auth;

const BASE_URL: &str = "https://www.realmofthemadgod.com/";
const CLIENT_TOKEN: &str = "6f97fc3698b237db27591d6b431a9532b14d1922";
const DEFAULT_PARAMS: [(&str, &str); 3] = [
    ("game_net", "Unity"),
    ("play_platform", "Unity"),
    ("game_net_user_id", ""),
];

pub trait ExaltaClientTrait {
    fn new() -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
}

pub struct ExaltaClient {
    pub client: Client,
    pub base_url: Url
}

impl ExaltaClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let base_url = Url::parse(BASE_URL)?;

        let mut defheaders = HeaderMap::new();
        defheaders.insert("Host", base_url.host_str().unwrap().parse()?);
        defheaders.insert("Accept", "*/*".parse()?);
        defheaders.insert( "Accept-Encoding", HeaderValue::from_static("gzip, deflate"));
        defheaders.insert( "X-Unity-Version", HeaderValue::from_static("2020.3.30f1"));
        let client = Client::builder()
            .http1_title_case_headers()
            .user_agent("UnityPlayer/2020.3.30f1 (UnityWebRequest/1.0, libcurl/7.80.0-DEV)")
            .default_headers(defheaders)
            .build()?;
        
        Ok(Self {
            client,
            base_url
        })
    }

    pub async fn login(mut self, username: &str, password: &str) -> Result<AuthController, Box<dyn std::error::Error>> {
        let tokenparams = vec![
            ("clientToken", CLIENT_TOKEN)
        ];

        let userpassparams = [
            tokenparams.clone(),
            DEFAULT_PARAMS.to_vec(),
            vec![
                ("guid", username),
                ("password", password),
            ]
        ].concat();
        let resp = self.client
            .post(self.base_url.join("account/verify")?)
            .form(&userpassparams)
            .send()
            .await?;
        let account = quick_xml::de::from_str::<Account>(resp.text().await?.as_str())?;
        
        Ok(AuthController{
            account,
            base_url: self.base_url,
            client: self.client
        })
    }
}
