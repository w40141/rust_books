use anyhow::Result;

#[derive(Debug)]
pub struct CardModel {
    card_id: u32,
    name: String,
    name_en: String,
    ruby: String,
    level: Option<u32>,
    attribute: String,
    race: String,
    item: String,
    attack: Option<u32>,
    defense: Option<u32>,
    text: String,
}

#[async_trait::async_trait]
pub trait CardRepository {
    async fn get_detail_from_card_id(&self, card_id: u32) -> Result<CardModel>;
    async fn get_details_from_attack(
        &self,
        attack_min: u32,
        attack_max: u32,
    ) -> Result<Vec<CardModel>>;
    async fn get_details_from_defense(
        &self,
        defense_min: u32,
        defense_max: u32,
    ) -> Result<Vec<CardModel>>;
    async fn get_details_from_level(
        &self,
        level_min: u32,
        level_max: u32,
    ) -> Result<Vec<CardModel>>;
    async fn get_details_from_attribute(&self, attributes: Vec<String>) -> Result<Vec<CardModel>>;
    async fn get_details_from_races(&self, race: Vec<String>) -> Result<Vec<CardModel>>;
    async fn get_details_from_items(&self, items: Vec<String>) -> Result<Vec<CardModel>>;
}
