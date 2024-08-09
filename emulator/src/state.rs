use std::collections::HashMap;

use artifacts_openapi::models::{
    CharacterSchema, InventorySlot, ItemSchema, MapSchema, MonsterSchema, ResourceSchema,
};
use swagger::ApiError;

#[derive(Default)]
pub struct State {
    pub characters: HashMap<String, CharacterSchema>,
    pub resources: Vec<ResourceSchema>,
    pub maps: Vec<MapSchema>,
    pub monsters: Vec<MonsterSchema>,
    pub items: Vec<ItemSchema>,
}

impl State {
    pub async fn from_seed() -> Result<State, Box<dyn std::error::Error>> {
        let data = tokio::fs::read(".seed_data/resources.json").await?;
        let resources: Vec<ResourceSchema> = serde_json::from_slice(&data)?;

        let data = tokio::fs::read(".seed_data/maps.json").await?;
        let maps: Vec<MapSchema> = serde_json::from_slice(&data)?;

        let data = tokio::fs::read(".seed_data/monsters.json").await?;
        let monsters: Vec<MonsterSchema> = serde_json::from_slice(&data)?;

        let data = tokio::fs::read(".seed_data/items.json").await?;
        let items: Vec<ItemSchema> = serde_json::from_slice(&data)?;

        Ok(State {
            resources,
            maps,
            monsters,
            items,
            ..Default::default()
        })
    }

    pub fn get_map(&self, x: i32, y: i32) -> Result<Option<&MapSchema>, ApiError> {
        Ok(self.maps.iter().find(|m| m.x == x && m.y == y))
    }

    pub fn get_resource(&self, code: &str) -> Result<Option<&ResourceSchema>, ApiError> {
        Ok(self.resources.iter().find(|r| r.code == code))
    }

    pub fn get_monster(&self, code: &str) -> Result<Option<&MonsterSchema>, ApiError> {
        Ok(self.monsters.iter().find(|m| m.code == code))
    }

    pub fn get_item(&self, code: &str) -> Result<Option<&ItemSchema>, ApiError> {
        Ok(self.items.iter().find(|i| i.code == code))
    }

    pub fn skill_level(&self, character: &str, skill: &str) -> Result<i32, ApiError> {
        let character = self.get_character(&character)?;

        Ok(match skill {
            "cooking" => character.cooking_level,
            "gearcrafting" => character.gearcrafting_level,
            "jewelrycrafting" => character.jewelrycrafting_level,
            "mining" => character.mining_level,
            "weaponcrafting" => character.weaponcrafting_level,
            "woodcutting" => character.woodcutting_level,
            _ => return Err(ApiError("unknown skill".into())),
        })
    }

    // returns the quantity of the given item
    pub fn get_inventory_item(&self, character: &str, code: &str) -> Result<i32, ApiError> {
        let character = self.get_character(&character)?;

        let inventory = match &character.inventory {
            Some(inv) => inv,
            None => {
                log::debug!("player does not have an inventory");
                return Ok(0);
            }
        };

        Ok(inventory
            .iter()
            .find(|i| i.code == code)
            .map(|i| i.quantity)
            .unwrap_or(0))
    }

    pub fn get_character(&self, name: &str) -> Result<&CharacterSchema, ApiError> {
        match self.characters.get(name) {
            Some(c) => Ok(c),
            None => Err(ApiError("Character not found".into())),
        }
    }

    pub fn get_character_mut(&mut self, name: &str) -> Result<&mut CharacterSchema, ApiError> {
        match self.characters.get_mut(name) {
            Some(c) => Ok(c),
            None => Err(ApiError("Character not found".into())),
        }
    }

    pub fn create_character(
        &mut self,
        name: impl Into<String>,
        skin: impl Into<String>,
    ) -> Result<(), ApiError> {
        let name = name.into();
        let skin = skin.into();

        let character = CharacterSchema {
            name: name.clone(),
            x: 1,
            y: 1,
            level: 1,
            mining_level: 1,
            mining_xp: 1,
            xp: 1,
            woodcutting_level: 1,
            fishing_level: 1,
            weaponcrafting_level: 1,
            gearcrafting_level: 1,
            jewelrycrafting_level: 1,
            cooking_level: 1,
            cooldown: 0,
            gold: 0,
            skin,
            inventory: Some(vec![]),
            ..Default::default()
        };

        if self.characters.contains_key(&name) {
            return Err(ApiError("character already exists".into()));
        }

        self.characters.insert(name, character);

        Ok(())
    }

    pub fn give_item(&mut self, character: &str, code: &str) -> Result<&CharacterSchema, ApiError> {
        let character = self.get_character_mut(character)?;

        let inventory = match &mut character.inventory {
            Some(inventory) => inventory,
            None => return Err(ApiError("Character has no inventory".into())),
        };

        let slot = match inventory.iter_mut().find(|slot| slot.code == code) {
            Some(slot) => slot,
            None => {
                inventory.push(InventorySlot {
                    slot: 1,
                    code: code.into(),
                    quantity: 1,
                });
                return Ok(character);
            }
        };

        slot.quantity += 1;

        Ok(character)
    }

    pub fn move_to(
        &mut self,
        character: &str,
        x: i32,
        y: i32,
    ) -> Result<&CharacterSchema, ApiError> {
        let character = self.get_character_mut(character)?;

        character.x = x;
        character.y = y;

        Ok(character)
    }
}
