use std::{collections::{HashMap, HashSet}, fmt::Display};

use rand::Rng;

use crate::{read_json_response, BadResponse};

#[derive(serde::Deserialize)]
pub struct CollectionConfig{
    pub id_parts: IdParts,
    // pub reviewers: ...,
    // pub collection_template: ...,
}

#[derive(serde::Deserialize)]
pub struct IdParts{
    pub model: ModelIdParts,
    //pub dataset: ...,
    //pub  notebook: ...,
}

#[derive(serde::Deserialize)]
pub struct ModelIdParts{
    pub nouns: HashMap<String, String>,
    pub adjectives: Vec<String>,
}

impl CollectionConfig{
    pub fn request() -> http::Request<[u8; 0]>{
        http::Request::builder()
            .method(http::Method::GET)
            .uri("https://raw.githubusercontent.com/bioimage-io/collection/main/bioimageio_collection_config.json")
            .body([])
            .unwrap()
    }

    pub fn parse_response(resp: &http::Response<Vec<u8>>) -> Result<Self, BadResponse>{
        read_json_response(&resp)
    }
}

//////////////////
#[derive(serde::Deserialize)]
pub struct CollectionJson{
    collection: Vec<CollectionItem>
}

#[derive(serde::Deserialize)]
pub struct CollectionItem{
    #[serde(default)]
    nickname: Option<ZooNickname>,
}

impl CollectionJson{
    pub fn request() -> http::Request<[u8; 0]>{
        http::Request::builder()
            .method(http::Method::GET)
            .uri("https://uk1s3.embassy.ebi.ac.uk/public-datasets/bioimage.io/collection.json")
            .body([])
            .unwrap()
    }
    pub fn parse_response(response: &http::Response<Vec<u8>>) -> Result<Self, BadResponse>{
        read_json_response(&response)
    }
}
///////////////////////

#[derive(PartialEq, Eq, Clone)]
pub struct Animal{
    noun: String,
    emoji: String,
}

//////////////////////////////


#[derive(Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize, Clone)]
#[serde(try_from="String")]
#[serde(into="String")]
pub struct ZooNickname{
    noun: String,
    adjective: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ZooNickNameParsingError{
    #[error("nickname is missing a noun, like 'shark'")]
    MissingNoun,
    #[error("nickname is missing an adjective, like 'affable'")]
    MissingAdjective,
}

impl TryFrom<String> for ZooNickname{
    type Error = ZooNickNameParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.splitn(2, "-");
        let adjective = parts.next().ok_or(ZooNickNameParsingError::MissingAdjective)?;
        let noun = parts.next().ok_or(ZooNickNameParsingError::MissingNoun)?;
        Ok(ZooNickname { noun: noun.to_owned(), adjective: adjective.to_owned() })
    }
}

impl From<ZooNickname> for String{
    fn from(value: ZooNickname) -> Self {
        format!("{}-{}", value.adjective, value.noun)
    }
}

impl Display for ZooNickname{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.adjective, self.noun)
    }
}

/////////////////////////////////

pub struct ZooNicknameGenerator{
    animals: Vec<Animal>,
    adjectives: Vec<String>,
    taken_nicknames: HashSet<ZooNickname>,
}

impl ZooNicknameGenerator{
    pub fn new(collection_config: CollectionConfig, collection_json: CollectionJson) -> Self{
        let animals: Vec<Animal> = collection_config.id_parts.model.nouns.into_iter()
            .map(|(animal_noun, animal_emoji)|{
                Animal{noun: animal_noun, emoji: animal_emoji}
            })
            .collect();
        let adjectives: Vec<String> = collection_config.id_parts.model.adjectives;

        let taken_nicknames: HashSet::<ZooNickname> = collection_json.collection.into_iter()
            .filter_map(|item| item.nickname)
            .collect();

        Self{animals, adjectives, taken_nicknames}
    }
    pub fn generate_zoo_nickname(&self) -> Option<ZooNickname>{
        let animal_idx = rand::thread_rng().gen_range(0..self.animals.len());
        let adjective_idx = rand::thread_rng().gen_range(0..self.adjectives.len());
        let nickname = ZooNickname{
            noun: self.animals[animal_idx].noun.clone(),
            adjective: self.adjectives[adjective_idx].clone(),
        };
        if self.taken_nicknames.contains(&nickname){
            return None
        }
        Some(nickname)
    }
}
