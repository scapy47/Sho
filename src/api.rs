use clap::ValueEnum;
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;
use std::{collections::HashMap, time::Duration};
use ureq::{Agent, RequestBuilder, typestate::WithoutBody};

use crate::decrypt_url;

//  NOTE: Response from search_anime()
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnimeEdge {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,

    pub english_name: Option<String>,
    pub available_episodes: Option<HashMap<String, Value>>,
    pub thumbnail: String,
    pub description: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Deserialize, Debug)]
pub struct ShowsData {
    pub edges: Vec<AnimeEdge>,
}

#[derive(Deserialize, Debug)]
pub struct DataWrapper {
    pub shows: ShowsData,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub data: DataWrapper,
}

//  NOTE: Response for get_episode_links()
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceUrl {
    pub source_url: String,
    pub source_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeData {
    pub episode_string: String,
    pub source_urls: Vec<SourceUrl>,
}

#[derive(Deserialize, Debug)]
pub struct EpisodeDataWrapper {
    pub episode: EpisodeData,
}

#[derive(Deserialize, Debug)]
pub struct EpisodeResponse {
    pub data: EpisodeDataWrapper,
}

//  NOTE: Response for get_episode_list()
#[derive(Deserialize, Debug)]
pub struct ShowDetail {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "availableEpisodesDetail")]
    pub available_episodes_detail: HashMap<String, Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct ShowDetailData {
    pub show: ShowDetail,
}

#[derive(Deserialize, Debug)]
pub struct EpisodeListResponse {
    pub data: ShowDetailData,
}

#[derive(Debug)]
pub struct Api {
    pub base_api: &'static str,
    pub referer: &'static str,
    pub user_agent: &'static str,
    pub mode: &'static str,
    pub debug: bool,
    agent: Agent,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Mode {
    Sub,
    Dub,
    Raw,
}

impl Api {
    pub fn new(mode: Mode, debug: bool) -> Self {
        let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Gecko/20100101 Firefox/121.0";
        let config = Agent::config_builder()
            .timeout_per_call(Some(Duration::from_secs(12)))
            .user_agent(user_agent)
            .https_only(true)
            .build();

        Api {
            base_api: "https://api.allanime.day/api",
            referer: "https://allmanga.to",
            user_agent,
            mode: match mode {
                Mode::Sub => "sub",
                Mode::Dub => "dub",
                Mode::Raw => "raw",
            },
            debug,
            agent: Agent::new_with_config(config),
        }
    }

    fn request_api<T: DeserializeOwned>(
        &self,
        variables: &str,
        gql: &str,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let body = serde_json::json!({
        "query": gql,
        "variables": serde_json::from_str::<serde_json::Value>(variables)
            .unwrap_or(serde_json::json!({}))
        });

        let resp = self
            .agent
            .post(self.base_api)
            .header("Referer", self.referer)
            .header("Content-Type", "application/json")
            .send_json(&body)?;

        let parsed: T = resp.into_body().read_json()?;
        Ok(parsed)
    }

    /// Search for anime with its name
    pub fn search_anime(&self, query: &str) -> Result<SearchResponse, Box<dyn std::error::Error>> {
        let gql = "query( $search: SearchInput $limit: Int $page: Int $translationType: VaildTranslationTypeEnumType $countryOrigin: VaildCountryOriginEnumType ) { shows( search: $search limit: $limit page: $page translationType: $translationType countryOrigin: $countryOrigin ) { edges { _id name englishName availableEpisodes __typename thumbnail description } }}";

        let variables_json = &format!(
            r#"{{"search":{{"allowAdult":false,"allowUnknown":false,"query":"{}"}},"limit":40,"page":1,"translationType":"{}","countryOrigin":"ALL"}}"#,
            query, self.mode
        );

        let resp: SearchResponse = self.request_api(variables_json, gql)?;

        Ok(resp)
    }

    /// Get the links that can be played/download
    pub fn get_episode_links(
        &self,
        id: &str,
        ep: &str,
    ) -> Result<(String, Vec<(String, String)>), Box<dyn std::error::Error>> {
        let gql = "query ($showId: String!, $translationType: VaildTranslationTypeEnumType!, $episodeString: String!) { episode( showId: $showId translationType: $translationType episodeString: $episodeString ) { episodeString sourceUrls }}";

        let variables_json = &format!(
            r#"{{"showId":"{}","translationType":"{}","episodeString":"{}"}}"#,
            id, self.mode, ep
        );

        let resp: EpisodeResponse = self.request_api(variables_json, gql)?;

        let mut vec = Vec::new();
        for source in resp.data.episode.source_urls {
            let provider_name = source.source_name;
            let raw_uri = source.source_url;

            let uri = if let Some(stripped) = raw_uri.strip_prefix("--") {
                decrypt_url(stripped)
            } else if let Some(stripped) = raw_uri.strip_prefix("//") {
                format!("https:{}", stripped)
            } else {
                raw_uri
            };

            let uri = if uri.contains("/clock") && !uri.contains("/clock.json") {
                uri.replace("/clock", "/clock.json")
            } else {
                uri
            };

            let uri = if uri.starts_with("/apivtwo/") {
                format!("https://allanime.day{}", uri)
            } else {
                uri
            };

            if self.debug {
                unimplemented!()
            }

            vec.push((provider_name, uri));
        }

        Ok((resp.data.episode.episode_string, vec))
    }

    pub fn resolve_clock_urls(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        print!("running");

        let resp = self.agent.get(url).call()?;
        let json: serde_json::Value = resp.into_body().read_json()?;

        if let Some(links_array) = json["links"].as_array() {
            if let Some(first_item) = links_array.first() {
                if let Some(wixmp_url) = first_item["link"].as_str() {
                    return Ok(wixmp_url.to_string());
                }
            }
        }

        // json["links"]
        //     .as_array()
        //     .and_then(|arr| arr.first())
        //     .and_then(|item| item["link"].as_str())
        //     .map(|s| s.to_string());

        Err("Could not find 'link' field in clock.json response".into())
    }

    /// Get list of episodes available from api
    pub fn get_episode_list(
        &self,
        id: &str,
    ) -> Result<(String, Vec<String>, String), Box<dyn std::error::Error>> {
        let gql =
            "query ($showId: String!) { show( _id: $showId ) { _id name availableEpisodesDetail }}";
        let variables_json = &format!(r#"{{"showId":"{}"}}"#, id);

        let resp: EpisodeListResponse = self.request_api(variables_json, gql)?;

        let mut show = resp.data.show;

        let mut episodes = show
            .available_episodes_detail
            .remove(self.mode)
            .ok_or(format!("No episodes found for mode '{}'", self.mode))?;

        episodes.sort_by(|a, b| {
            let a_num = a.parse::<f64>().unwrap_or(0.0);
            let b_num = b.parse::<f64>().unwrap_or(0.0);
            a_num
                .partial_cmp(&b_num)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if self.debug {
            unimplemented!()
        }

        Ok((show.name, episodes, show.id))
    }
}
