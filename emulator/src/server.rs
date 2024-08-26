//! Main library entry point for artifacts_openapi implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use chrono::{Duration, Utc};
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::header::GetAll;
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use predicates::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use std::f32::consts::E;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::{EmptyContext, Nullable};
use swagger::{Has, XSpanIdString};
use tokio::net::TcpListener;

use artifacts_openapi::models::{
    self, ActionItemBankResponseSchema, BankItemSchema, CharacterFightDataSchema, CharacterFightResponseSchema, CharacterMovementDataSchema, CharacterMovementResponseSchema, CharacterResponseSchema, CharacterSchema, CooldownSchema, CraftSchema, CraftingSchema, DataPageActiveEventSchema, DataPageCharacterSchema, DataPageGeItemSchema, DataPageItemSchema, DataPageLogSchema, DataPageMapSchema, DataPageMonsterSchema, DataPageResourceSchema, DataPageSimpleItemSchema, DropSchema, EquipRequestSchema, EquipmentResponseSchema, FightSchema, GeItemResponseSchema, GeTransactionListSchema, GeTransactionResponseSchema, GeTransactionSchema, GoldBankResponseSchema, GoldResponseSchema, GoldSchema, GoldTransactionSchema, InventorySlot, ItemResponseSchema, ItemSchema, LogSchema, MapResponseSchema, MapSchema, MonsterResponseSchema, MonsterSchema, MyCharactersListSchema, RecyclingDataSchema, RecyclingItemsSchema, RecyclingResponseSchema, ResourceResponseSchema, ResourceSchema, SimpleItemSchema, SingleItemSchema, SkillDataSchema, SkillInfoSchema, SkillResponseSchema, StatusResponseSchema, TaskDataSchema, TaskResponseSchema, TaskRewardDataSchema, TaskRewardResponseSchema, TaskRewardSchema, TaskSchema
};

pub async fn create(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse().expect("Failed to parse bind address");

    let state = Arc::new(Mutex::new(State::from_seed().await?));

    for name in vec!["emulated1", "emulated2", "emulated3", "emulated4", "emulated5"] {
        state
            .lock()
            .unwrap()
            .create_character(name, "men1")?;
    }

    let server = Server::new(state.clone());

    let service = MakeService::new(server);

    let service =
        artifacts_openapi::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    info!("Starting a server (over http, so no TLS)");
    // Using HTTP
    hyper::server::Server::bind(&addr).serve(service).await?;

    Ok(())
}

fn empty_cooldown() -> CooldownSchema {
    CooldownSchema {
        total_seconds: 3,
        remaining_seconds: 3,
        started_at: Utc::now(),
        expiration: Utc::now() + Duration::seconds(3),
        reason: "EMPTY_COOLDOWN".into(),
        cooldown_expiration: Utc::now() + Duration::seconds(3),
    }
}

#[derive(Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
    state: Arc<Mutex<State>>,
}

impl<C> Server<C> {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Server {
            marker: PhantomData,
            state,
        }
    }
}

use crate::server_auth;
use crate::state::State;
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use swagger::auth::Authorization;

use artifacts_openapi::server::MakeService;
use artifacts_openapi::*;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    async fn delete_character_characters_delete_post(
        &self,
        delete_character_schema: models::DeleteCharacterSchema,
        context: &C,
    ) -> Result<DeleteCharacterCharactersDeletePostResponse, ApiError> {
        unimplemented!()
    }

    async fn get_resource_resources_code_get(
        &self,
        code: String,
        context: &C,
    ) -> Result<GetResourceResourcesCodeGetResponse, ApiError> {
        info!(
            "get_resource_resources_code_get({:?}) - X-Span-ID: {:?}",
            code,
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();

        let resource = match state.resources.iter().find(|r| r.code == code) {
            Some(r) => r,
            None => return Ok(GetResourceResourcesCodeGetResponse::RessourceNotFound),
        };

        Ok(
            GetResourceResourcesCodeGetResponse::SuccessfullyFetchedResource(
                ResourceResponseSchema {
                    data: resource.clone(),
                },
            ),
        )
    }

    /// Create Account
    async fn create_account_accounts_create_post(
        &self,
        add_account_schema: models::AddAccountSchema,
        context: &C,
    ) -> Result<CreateAccountAccountsCreatePostResponse, ApiError> {
        info!(
            "create_account_accounts_create_post({:?}) - X-Span-ID: {:?}",
            add_account_schema,
            context.get().0.clone()
        );
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Create Character
    async fn create_character_characters_create_post(
        &self,
        add_character_schema: models::AddCharacterSchema,
        context: &C,
    ) -> Result<CreateCharacterCharactersCreatePostResponse, ApiError> {
        info!(
            "create_character_characters_create_post({:?}) - X-Span-ID: {:?}",
            add_character_schema,
            context.get().0.clone()
        );

        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get All Characters
    async fn get_all_characters_characters_get(
        &self,
        sort: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllCharactersCharactersGetResponse, ApiError> {
        info!(
            "get_all_characters_characters_get({:?}, {:?}, {:?}) - X-Span-ID: {:?}",
            sort,
            page,
            size,
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();
        
        if state.characters.is_empty() {
            return Ok(GetAllCharactersCharactersGetResponse::CharactersNotFound);
        }

        Ok(GetAllCharactersCharactersGetResponse::SuccessfullyFetchedCharactersDetails(DataPageCharacterSchema {
            data: state.characters.iter().map(|(_, v)| v.clone()).collect(),
            pages: Some(Nullable::Present(1)),
            ..Default::default()
        }))
    }

    /// Get Character
    async fn get_character_characters_name_get(
        &self,
        name: String,
        context: &C,
    ) -> Result<GetCharacterCharactersNameGetResponse, ApiError> {
        info!(
            "get_character_characters_name_get(\"{}\") - X-Span-ID: {:?}",
            name,
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();
        let character = state.characters.get(&name);

        Ok(match character {
            Some(character) => GetCharacterCharactersNameGetResponse::SuccessfullyFetchedCharacter(CharacterResponseSchema {
                data: character.clone(),
            }),
            None => GetCharacterCharactersNameGetResponse::CharacterNotFound,
        })
    }

    /// Get Status
    async fn get_status_get(&self, context: &C) -> Result<GetStatusGetResponse, ApiError> {
        info!(
            "get_status_get() - X-Span-ID: {:?}",
            context.get().0.clone()
        );

        Ok(GetStatusGetResponse::SuccessfulResponse(StatusResponseSchema {
            data: Default::default(),
        }))
    }

    /// Get All Events
    async fn get_all_events_events_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllEventsEventsGetResponse, ApiError> {
        info!(
            "get_all_events_events_get({:?}, {:?}) - X-Span-ID: {:?}",
            page,
            size,
            context.get().0.clone()
        );

        Ok(
            GetAllEventsEventsGetResponse::SuccessfullyFetchedEventsDetails(
                DataPageActiveEventSchema {
                    data: vec![],
                    pages: Some(Nullable::Present(1)),
                    ..Default::default()
                },
            ),
        )
    }

    /// Get All Ge Items
    async fn get_all_ge_items_ge_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllGeItemsGeGetResponse, ApiError> {
        info!(
            "get_all_ge_items_ge_get({:?}, {:?}) - X-Span-ID: {:?}",
            page,
            size,
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();
        let ge_items = state.ge_items.clone();

        Ok(GetAllGeItemsGeGetResponse::FetchGrandExchangeItemsDetails(DataPageGeItemSchema {
            data: ge_items,
            pages: Some(Nullable::Present(1)),
            ..Default::default()
        }))
    }

    /// Get Ge Item
    async fn get_ge_item_ge_code_get(
        &self,
        code: String,
        context: &C,
    ) -> Result<GetGeItemGeCodeGetResponse, ApiError> {
        info!(
            "get_ge_item_ge_code_get(\"{}\") - X-Span-ID: {:?}",
            code,
            context.get().0.clone()
        );

        let state =  self.state.lock().unwrap();
        let ge_item = state.ge_items.iter().find(|i| i.code == code);

        Ok(match ge_item {
            Some(ge_item) => GetGeItemGeCodeGetResponse::SuccessfullyFetchedGrandExchangeItem(GeItemResponseSchema {
                data: ge_item.clone(),
            }),
            None => GetGeItemGeCodeGetResponse::ItemNotFound,
        })
    }

    /// Get All Items
    async fn get_all_items_items_get(
        &self,
        min_level: Option<i32>,
        max_level: Option<i32>,
        name: Option<String>,
        r#type: Option<String>,
        craft_skill: Option<String>,
        craft_material: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllItemsItemsGetResponse, ApiError> {
        info!("get_all_items_items_get({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", min_level, max_level, name, r#type, craft_skill, craft_material, page, size, context.get().0.clone());

        let state = self.state.lock().unwrap();

        let items = state
            .items
            .iter()
            .filter(|i| {
                min_level
                    .as_ref()
                    .map(|v| i.level >= *v as u32)
                    .unwrap_or(true)
            })
            .filter(|i| {
                max_level
                    .as_ref()
                    .map(|v| i.level <= *v as u32)
                    .unwrap_or(true)
            })
            .filter(|i| name.as_ref().map(|v| i.name == *v).unwrap_or(true))
            .filter(|i| r#type.as_ref().map(|v| i.r#type == *v).unwrap_or(true))
            .filter(|i| match &craft_skill {
                Some(skill) => match &i.craft {
                    Some(Nullable::Present(craft)) => match &craft.skill {
                        Some(craft_skill) => skill == craft_skill,
                        _ => false,
                    },
                    _ => false,
                },
                None => true,
            })
            .filter(|i| match &craft_material {
                Some(material) => match &i.craft {
                    Some(Nullable::Present(craft)) => match &craft.items {
                        Some(craft_items) => craft_items.iter().any(|ci| ci.code == *material),
                        _ => false,
                    },
                    _ => false,
                },
                None => true,
            })
            .cloned()
            .collect();

        Ok(GetAllItemsItemsGetResponse::FetchItemsDetails(
            DataPageItemSchema {
                data: items,
                pages: Some(Nullable::Present(1)),
                ..Default::default()
            },
        ))
    }

    /// Get Item
    async fn get_item_items_code_get(
        &self,
        code: String,
        context: &C,
    ) -> Result<GetItemItemsCodeGetResponse, ApiError> {
        info!(
            "get_item_items_code_get(\"{}\") - X-Span-ID: {:?}",
            code,
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();
        let item = match state.get_item(&code)? {
            Some(item) => item,
            None => return Err(ApiError("item not found".into())),
        };

        let ge = state.ge_items.iter().find(|ge_item| ge_item.code == code);

        Ok(GetItemItemsCodeGetResponse::SuccessfullyFetchedItem(
            ItemResponseSchema {
                data: SingleItemSchema {
                    item: item.clone(),
                    ge: ge.map(|ge| Nullable::Present(ge.clone())),
                },
            },
        ))
    }

    /// Get All Maps
    async fn get_all_maps_maps_get(
        &self,
        content_type: Option<String>,
        content_code: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllMapsMapsGetResponse, ApiError> {
        info!(
            "get_all_maps_maps_get({:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}",
            content_type,
            content_code,
            page,
            size,
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();

        Ok(GetAllMapsMapsGetResponse::SuccessfullyFetchedMapsDetails(DataPageMapSchema {
            data: state.maps.iter().map(|m| m.clone()).collect(),
            size: Nullable::Present(state.maps.len() as u32),
            pages: Some(Nullable::Present(1)),
            ..Default::default()
        }))
    }

    /// Get Map
    async fn get_map_maps_xy_get(
        &self,
        x: i32,
        y: i32,
        context: &C,
    ) -> Result<GetMapMapsXyGetResponse, ApiError> {
        info!(
            "get_map_maps_xy_get({}, {}) - X-Span-ID: {:?}",
            x,
            y,
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();
        let map = state.maps.iter().find(|map| map.x == x && map.y == y);

        Ok(match map {
            Some(map) => GetMapMapsXyGetResponse::SuccessfullyFetchedMap(MapResponseSchema {
                data: map.clone(),
            }),
            None => GetMapMapsXyGetResponse::MapNotFound,
        })
    }

    /// Get All Monsters
    async fn get_all_monsters_monsters_get(
        &self,
        min_level: Option<i32>,
        max_level: Option<i32>,
        drop: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllMonstersMonstersGetResponse, ApiError> {
        info!(
            "get_all_monsters_monsters_get({:?}, {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}",
            min_level,
            max_level,
            drop,
            page,
            size,
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();

        let monsters = state.monsters.iter()
            .filter(|monster| min_level.map(|min_level| monster.level > min_level).unwrap_or(true))
            .filter(|monster| max_level.map(|max_level| monster.level <= max_level).unwrap_or(true))
            // TODO: other filters
            .cloned()
            .collect();

        Ok(GetAllMonstersMonstersGetResponse::SuccessfullyFetchedMonstersDetails(DataPageMonsterSchema {
            data: monsters,
            pages: Some(Nullable::Present(1)),
            ..Default::default()
        }))
    }

    /// Get Monster
    async fn get_monster_monsters_code_get(
        &self,
        code: String,
        context: &C,
    ) -> Result<GetMonsterMonstersCodeGetResponse, ApiError> {
        info!(
            "get_monster_monsters_code_get(\"{}\") - X-Span-ID: {:?}",
            code,
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();

        let monster = match state.monsters.iter().find(|monster| monster.code == code) {
            Some(m) => m,
            None => return Ok(GetMonsterMonstersCodeGetResponse::MonsterNotFound),
        };

        Ok(
            GetMonsterMonstersCodeGetResponse::SuccessfullyFetchedMonster(MonsterResponseSchema {
                data: monster.clone(),
            }),
        )
    }

    /// Change Password
    async fn change_password_my_change_password_post(
        &self,
        change_password: models::ChangePassword,
        context: &C,
    ) -> Result<ChangePasswordMyChangePasswordPostResponse, ApiError> {
        info!(
            "change_password_my_change_password_post({:?}) - X-Span-ID: {:?}",
            change_password,
            context.get().0.clone()
        );
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get Bank Golds
    async fn get_bank_golds_my_bank_gold_get(
        &self,
        context: &C,
    ) -> Result<GetBankGoldsMyBankGoldGetResponse, ApiError> {
        info!(
            "get_bank_golds_my_bank_gold_get() - X-Span-ID: {:?}",
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();

        Ok(GetBankGoldsMyBankGoldGetResponse::SuccessfullyFetchedGolds(
            GoldBankResponseSchema {
                data: GoldSchema {
                    quantity: state.gold,
                },
            },
        ))
    }

    /// Get Bank Items
    async fn get_bank_items_my_bank_items_get(
        &self,
        item_code: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetBankItemsMyBankItemsGetResponse, ApiError> {
        info!(
            "get_bank_items_my_bank_items_get({:?}, {:?}, {:?}) - X-Span-ID: {:?}",
            item_code,
            page,
            size,
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();

        Ok(
            GetBankItemsMyBankItemsGetResponse::SuccessfullyFetchedItems(
                DataPageSimpleItemSchema {
                    data: state.bank_items.clone(),
                    pages: Some(Nullable::Present(if state.bank_items.is_empty() { 0 } else { 1 })),
                    ..Default::default()
                },
            ),
        )
    }

    /// Action Accept New Task
    async fn action_accept_new_task_my_name_action_task_new_post(
        &self,
        name: String,
        context: &C,
    ) -> Result<ActionAcceptNewTaskMyNameActionTaskNewPostResponse, ApiError> {
        info!(
            "action_accept_new_task_my_name_action_task_new_post(\"{}\") - X-Span-ID: {:?}",
            name,
            context.get().0.clone()
        );

        let mut state = self.state.lock().unwrap();

        let character = state.get_character_mut(&name)?;
        character.task = "chicken".into();
        character.task_progress = 0;
        character.task_total = 100;
        character.task_type = "monster".into();

        Ok(ActionAcceptNewTaskMyNameActionTaskNewPostResponse::NewTaskSuccessfullyAccepted(TaskResponseSchema {
            data: TaskDataSchema {
                cooldown: empty_cooldown(),
                task: TaskSchema {
                    code: "chicken".into(),
                    r#type: "monster".into(),
                    total: 100,
                },
                character: character.clone(),
            }
        }))
    }

    /// Action Complete Task
    async fn action_complete_task_my_name_action_task_complete_post(
        &self,
        name: String,
        context: &C,
    ) -> Result<ActionCompleteTaskMyNameActionTaskCompletePostResponse, ApiError> {
        info!(
            "action_complete_task_my_name_action_task_complete_post(\"{}\") - X-Span-ID: {:?}",
            name,
            context.get().0.clone()
        );

        let mut state = self.state.lock().unwrap();

        let character = state.get_character_mut(&name)?;

        if character.task.is_empty() {
            return Ok(ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterHasNoTask);
        }

        if character.task_progress != character.task_total {
            return Ok(ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterHasNotCompletedTheTask);
        }

        character.task = "".into();
        character.task_progress = 0;
        character.task_total = 0;
        character.task_type = "".into();

        Ok(ActionCompleteTaskMyNameActionTaskCompletePostResponse::TheTaskHasBeenSuccessfullyCompleted(TaskRewardResponseSchema {
            data: TaskRewardDataSchema {
                cooldown: empty_cooldown(),
                reward: TaskRewardSchema {
                    code: "golden_egg".into(),
                    quantity: 1,
                },
                character: character.clone(),
            }
        }))
    }

    /// Action Crafting
    async fn action_crafting_my_name_action_crafting_post(
        &self,
        name: String,
        crafting_schema: models::CraftingSchema,
        context: &C,
    ) -> Result<ActionCraftingMyNameActionCraftingPostResponse, ApiError> {
        info!(
            "action_crafting_my_name_action_crafting_post(\"{}\", {:?}) - X-Span-ID: {:?}",
            name,
            crafting_schema,
            context.get().0.clone()
        );

        let CraftingSchema { code, quantity } = crafting_schema;

        let item = {
            let state = self.state.lock().unwrap();

            match state.get_item(&code)? {
                Some(item) => item.clone(),
                None => return Ok(ActionCraftingMyNameActionCraftingPostResponse::CraftNotFound),
            }
        };

        let CraftSchema {
            items,
            level,
            skill,
            quantity: quantity_crafted,
        } = match item
            .craft
            .as_ref()
            .map(|craft| craft.as_ref().map(|craft| Some(craft)).unwrap_or(None))
            .flatten()
        {
            Some(s) => s,
            None => return Ok(ActionCraftingMyNameActionCraftingPostResponse::CraftNotFound),
        };

        // TODO: check theyre at the correct workshop!

        let skill = if let Some(skill) = skill {
            let level = match level {
                Some(level) => level,
                None => {
                    return Ok(
                        ActionCraftingMyNameActionCraftingPostResponse::NotSkillLevelRequired,
                    )
                }
            };

            let state = self.state.lock().unwrap();
            // check character has correct skill level

            let character_level = state.skill_level(&name, &skill)?;

            if character_level < *level {
                return Ok(ActionCraftingMyNameActionCraftingPostResponse::NotSkillLevelRequired);
            }

            skill
        } else {
            return Err(ApiError("item doesnt require a skill wtf".into()));
        };

        if let Some(items) = items {
            // check inventory has required items
            let state = self.state.lock().unwrap();

            for item in items {
                let has_quantity = state.get_inventory_item(&name, &item.code)?;

                if has_quantity < item.quantity as i32 {
                    log::error!(
                        "player does not have enough has={} need={}",
                        has_quantity,
                        item.quantity
                    );
                    return Ok(ActionCraftingMyNameActionCraftingPostResponse::MissingItemOrInsufficientQuantityInYourInventory);
                }
            }

            // remove items from inventory
        }

        let mut state = self.state.lock().unwrap();
        state.give_item(&name, &code)?;

        // return item * quantity_crafted

        let character = state.get_character_mut(&name)?;

        match skill.as_str() {
            "gearcrafting" => character.gearcrafting_level += 1,
            "weaponcrafting" => character.weaponcrafting_level += 1,
            "jewelrycrafting" => character.jewelrycrafting_level += 1,
            _ => {}
        }

        Ok(
            ActionCraftingMyNameActionCraftingPostResponse::TheItemWasSuccessfullyCrafted(
                SkillResponseSchema {
                    data: SkillDataSchema {
                        cooldown: empty_cooldown(),
                        character: character.clone(),
                        details: SkillInfoSchema {
                            xp: 0,
                            items: vec![DropSchema {
                                code,
                                quantity: quantity_crafted.as_ref().map(|q| *q).unwrap_or(1),
                            }],
                        },
                    },
                },
            ),
        )
    }

    /// Action Delete Item
    async fn action_delete_item_my_name_action_delete_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
        context: &C,
    ) -> Result<ActionDeleteItemMyNameActionDeletePostResponse, ApiError> {
        info!(
            "action_delete_item_my_name_action_delete_post(\"{}\", {:?}) - X-Span-ID: {:?}",
            name,
            simple_item_schema,
            context.get().0.clone()
        );
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Action Deposit Bank Gold
    async fn action_deposit_bank_gold_my_name_action_bank_deposit_gold_post(
        &self,
        name: String,
        deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
        context: &C,
    ) -> Result<ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse, ApiError> {
        info!("action_deposit_bank_gold_my_name_action_bank_deposit_gold_post(\"{}\", {:?}) - X-Span-ID: {:?}", name, deposit_withdraw_gold_schema, context.get().0.clone());

        let mut state = self.state.lock().unwrap();
        state.gold += deposit_withdraw_gold_schema.quantity;

        Ok(ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::GoldsSuccessfullyDepositedInYourBank(GoldResponseSchema {
            data: GoldTransactionSchema {
                cooldown: empty_cooldown(),
                bank: GoldSchema {
                    quantity: state.gold,
                },
                character: state.get_character(&name)?.clone(),
            },
        }))
    }

    /// Action Deposit Bank
    async fn action_deposit_bank_my_name_action_bank_deposit_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
        context: &C,
    ) -> Result<ActionDepositBankMyNameActionBankDepositPostResponse, ApiError> {
        info!(
            "action_deposit_bank_my_name_action_bank_deposit_post(\"{}\", {:?}) - X-Span-ID: {:?}",
            name,
            simple_item_schema,
            context.get().0.clone()
        );

        let mut state = self.state.lock().unwrap();

        let already_exists = if let Some(bank_item) = state.bank_items.iter_mut().find(|bank_item| bank_item.code == simple_item_schema.code) {
            bank_item.quantity += simple_item_schema.quantity;
            true
        } else {
            false
        };

        if !already_exists {
            state.bank_items.push(simple_item_schema.clone());
        }

        for _ in 0..simple_item_schema.quantity {
            state.remove_item(&name, &simple_item_schema.code)?;
        }

        Ok(ActionDepositBankMyNameActionBankDepositPostResponse::ItemSuccessfullyDepositedInYourBank(ActionItemBankResponseSchema {
            data: BankItemSchema {
                cooldown: empty_cooldown(),
                item: state.get_item(&simple_item_schema.code)?.unwrap().clone(),
                character: state.get_character(&name)?.clone(),
                bank: state.bank_items.clone(),
            },
        }))
    }

    /// Action Equip Item
    async fn action_equip_item_my_name_action_equip_post(
        &self,
        name: String,
        equip_schema: models::EquipSchema,
        context: &C,
    ) -> Result<ActionEquipItemMyNameActionEquipPostResponse, ApiError> {
        info!(
            "action_equip_item_my_name_action_equip_post(\"{}\", {:?}) - X-Span-ID: {:?}",
            name,
            equip_schema,
            context.get().0.clone()
        );


        {

            let item = {
                let state = self.state.lock().unwrap();

                let item = match state.get_item(&equip_schema.code)? {
                    Some(i) => i,
                    None => return Ok(ActionEquipItemMyNameActionEquipPostResponse::ItemNotFound),
                };

                item.clone()
            };

            let mut state = self.state.lock().unwrap();
            let character = state.get_character_mut(&name)?;

            let effects = item.effects.clone().unwrap_or_default();
            for effect in effects {
                match effect.name.as_str() {
                    "attack_air" => character.attack_air += effect.value,
                    "attack_water" => character.attack_water += effect.value,
                    "attack_earth" => character.attack_earth += effect.value,
                    "attack_fire" => character.attack_fire += effect.value,
                    "boost_dmg_air" => character.dmg_air += effect.value,
                    "boost_dmg_earth" => character.dmg_earth += effect.value,
                    "boost_dmg_fire" => character.dmg_fire += effect.value,
                    "boost_dmg_water" => character.dmg_water += effect.value,
                    "boost_hp" => character.hp += effect.value,
                    _ => {}
                }
            }
        }

        {
            let mut state = self.state.lock().unwrap();

            let character = state.get_character_mut(&name)?;

            match equip_schema.slot.as_str() {
                "weapon" => character.weapon_slot = equip_schema.code.clone(),
                "shield" => character.shield_slot = equip_schema.code.clone(),
                "helmet" => character.helmet_slot = equip_schema.code.clone(),
                "body_armor" => character.body_armor_slot = equip_schema.code.clone(),
                "leg_armor" => character.leg_armor_slot = equip_schema.code.clone(),
                "boots" => character.boots_slot = equip_schema.code.clone(),
                "ring1" => character.ring1_slot = equip_schema.code.clone(),
                "ring2" => character.ring2_slot = equip_schema.code.clone(),
                "amulet" => character.amulet_slot = equip_schema.code.clone(),
                "artifact1" => character.artifact1_slot = equip_schema.code.clone(),
                "artifact2" => character.artifact2_slot = equip_schema.code.clone(),
                "artifact3" => character.artifact3_slot = equip_schema.code.clone(),
                "consumable1" => character.consumable1_slot = equip_schema.code.clone(),
                "consumable2" => character.consumable2_slot = equip_schema.code.clone(),
                _ => return Err(ApiError("Api-Error: invalid slot".into())),
            }


            state.remove_item(&name, &equip_schema.code)?;
        }

        let state = self.state.lock().unwrap();

        Ok(ActionEquipItemMyNameActionEquipPostResponse::TheItemHasBeenSuccessfullyEquippedOnYourCharacter(EquipmentResponseSchema {
            data: EquipRequestSchema {
                cooldown: empty_cooldown(),
                character: state.get_character(&name)?.clone(),
                item: state.get_item(&equip_schema.code)?.unwrap().clone(),
                slot: equip_schema.slot.clone(),
            },
        }))
    }

    /// Action Fight
    async fn action_fight_my_name_action_fight_post(
        &self,
        name: String,
        context: &C,
    ) -> Result<ActionFightMyNameActionFightPostResponse, ApiError> {
        info!(
            "action_fight_my_name_action_fight_post(\"{}\") - X-Span-ID: {:?}",
            name,
            context.get().0.clone()
        );

        let monster = {
            let state = self.state.lock().unwrap();

            let character = state.get_character(&name)?;

            let map = match state.get_map(character.x, character.y)? {
                Some(map) => map,
                None => return Err(ApiError("no map at this location".into())),
            };

            let code = match &map.content {
                Nullable::Present(content) => content.code.clone(),
                _ => return Err(ApiError("no content at this location".into())),
            };

            match state.get_monster(&code)? {
                Some(m) => m.clone(),
                None => return Err(ApiError("could not find monster".into())),
            }
        };

        {
            let mut state = self.state.lock().unwrap();
            let character = state.get_character_mut(&name)?;
            character.level += 1;
            character.hp += 10;

            if character.task == monster.code {
                character.task_progress += 1;
            }
        }

        let mut state = self.state.lock().unwrap();
        let mut character = None;
        for drop in monster.drops {
            character = Some(state.give_item(&name, &drop.code)?);
        }

        let character = match character {
            Some(c) => c,
            None => return Err(ApiError("monster has no drops".into())),
        };

        Ok(
            ActionFightMyNameActionFightPostResponse::TheFightEndedSuccessfully(
                CharacterFightResponseSchema {
                    data: CharacterFightDataSchema {
                        character: character.clone(),
                        cooldown: empty_cooldown(),
                        fight: FightSchema {
                            ..Default::default()
                        },
                    },
                },
            ),
        )
    }

    /// Action Gathering
    async fn action_gathering_my_name_action_gathering_post(
        &self,
        name: String,
        context: &C,
    ) -> Result<ActionGatheringMyNameActionGatheringPostResponse, ApiError> {
        info!(
            "action_gathering_my_name_action_gathering_post(\"{}\") - X-Span-ID: {:?}",
            name,
            context.get().0.clone()
        );

        // TODO: check player is high enough level!

        let code = {
            let mut state = self.state.lock().unwrap();

            let character = state.get_character(&name)?;

            let map = match state.get_map(character.x, character.y)? {
                Some(map) => map,
                _ => return Err(ApiError("location does not exist".into())),
            };

            match &map.content {
                Nullable::Present(content) => content.code.clone(),
                _ => return Err(ApiError("resource not found at this location".into())),
            }
        };

        let resource = {
            let state = self.state.lock().unwrap();

            match state.get_resource(&code)? {
                Some(r) => r.clone(),
                None => return Err(ApiError("resource does not exist".into())),
            }
        };

        {
            let mut state = self.state.lock().unwrap();
            let character = state.get_character_mut(&name)?;

            match resource.skill.as_str() {
                "mining" => character.mining_level += 1,
                "woodcutting" => character.woodcutting_level += 1,
                "fishing" => character.fishing_level += 1,
                "cooking" => character.cooking_level += 1,
                _ => return Err(ApiError("unknwon skill".into())),
            }
        }

        let mut state = self.state.lock().unwrap();

        let mut character = None;
        for drop in &resource.drops {
            character = Some(state.give_item(&name, &drop.code)?);
        }

        let character = match character {
            Some(c) => c,
            None => return Err(ApiError("resource had no drops!".into())),
        };

        Ok(ActionGatheringMyNameActionGatheringPostResponse::TheResourceHasBeenSuccessfullyGathered(SkillResponseSchema {
            data: SkillDataSchema {
                cooldown: empty_cooldown(),
                details: models::SkillInfoSchema { xp: 1, items: vec![] },
                character: character.clone(),
            }
        }))
    }

    /// Action Ge Buy Item
    async fn action_ge_buy_item_my_name_action_ge_buy_post(
        &self,
        name: String,
        ge_transaction_item_schema: models::GeTransactionItemSchema,
        context: &C,
    ) -> Result<ActionGeBuyItemMyNameActionGeBuyPostResponse, ApiError> {
        info!(
            "action_ge_buy_item_my_name_action_ge_buy_post(\"{}\", {:?}) - X-Span-ID: {:?}",
            name,
            ge_transaction_item_schema,
            context.get().0.clone()
        );


        // TODO: add ge stock

        let mut state = self.state.lock().unwrap();

        for _ in 0..ge_transaction_item_schema.quantity {
            state.give_item(&name, &ge_transaction_item_schema.code)?;
        }

        let character = state.get_character_mut(&name)?;
        character.gold -= (ge_transaction_item_schema.price * ge_transaction_item_schema.quantity as u32) as i32;

        Ok(ActionGeBuyItemMyNameActionGeBuyPostResponse::ItemSuccessfullyBuyFromTheGrandExchange(GeTransactionResponseSchema {
            data: GeTransactionListSchema {
                cooldown: empty_cooldown(),
                transaction: GeTransactionSchema { 
                    code: ge_transaction_item_schema.code,
                    quantity: ge_transaction_item_schema.quantity as i32,
                    price: ge_transaction_item_schema.price as i32,
                    total_price: (ge_transaction_item_schema.price * ge_transaction_item_schema.quantity as u32) as i32,
                },
                character: character.clone(),
            },
        }))
    }

    /// Action Ge Sell Item
    async fn action_ge_sell_item_my_name_action_ge_sell_post(
        &self,
        name: String,
        ge_transaction_item_schema: models::GeTransactionItemSchema,
        context: &C,
    ) -> Result<ActionGeSellItemMyNameActionGeSellPostResponse, ApiError> {
        info!(
            "action_ge_sell_item_my_name_action_ge_sell_post(\"{}\", {:?}) - X-Span-ID: {:?}",
            name,
            ge_transaction_item_schema,
            context.get().0.clone()
        );


        // TODO: remove ge stock

        let mut state = self.state.lock().unwrap();

        for _ in 0..ge_transaction_item_schema.quantity {
            state.remove_item(&name, &ge_transaction_item_schema.code)?;
        }

        let character = state.get_character_mut(&name)?;
        character.gold += (ge_transaction_item_schema.price * ge_transaction_item_schema.quantity as u32) as i32;

        Ok(ActionGeSellItemMyNameActionGeSellPostResponse::ItemSuccessfullySellAtTheGrandExchange(GeTransactionResponseSchema {
            data: GeTransactionListSchema {
                cooldown: empty_cooldown(),
                transaction: GeTransactionSchema { 
                    code: ge_transaction_item_schema.code,
                    quantity: ge_transaction_item_schema.quantity as i32,
                    price: ge_transaction_item_schema.price as i32,
                    total_price: (ge_transaction_item_schema.price * ge_transaction_item_schema.quantity as u32) as i32,
                },
                character: character.clone(),
            },
        }))
    }

    /// Action Move
    async fn action_move_my_name_action_move_post(
        &self,
        name: String,
        destination_schema: models::DestinationSchema,
        context: &C,
    ) -> Result<ActionMoveMyNameActionMovePostResponse, ApiError> {
        info!(
            "action_move_my_name_action_move_post(\"{}\", {:?}) - X-Span-ID: {:?}",
            name,
            destination_schema,
            context.get().0.clone()
        );

        let mut state = self.state.lock().unwrap();
        let character = state.move_to(&name, destination_schema.x, destination_schema.y)?;

        Ok(
            ActionMoveMyNameActionMovePostResponse::TheCharacterHasMovedSuccessfully(
                CharacterMovementResponseSchema {
                    data: CharacterMovementDataSchema {
                        cooldown: empty_cooldown(),
                        destination: MapSchema {
                            name: "name".into(),
                            x: destination_schema.x,
                            y: destination_schema.y,
                            ..Default::default()
                        },
                        character: character.clone(),
                    },
                },
            ),
        )

        // Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Action Recycling
    async fn action_recycling_my_name_action_recycling_post(
        &self,
        name: String,
        recycling_schema: models::RecyclingSchema,
        context: &C,
    ) -> Result<ActionRecyclingMyNameActionRecyclingPostResponse, ApiError> {
        info!(
            "action_recycling_my_name_action_recycling_post(\"{}\", {:?}) - X-Span-ID: {:?}",
            name,
            recycling_schema,
            context.get().0.clone()
        );

        // randomly gives you one or many of the items required to craft this item
        // we'll just return the first crafting item as rates arent disclosed


        let mut state = self.state.lock().unwrap();


        let item = match state.get_item(&recycling_schema.code)? {
            Some(item) => item,
            None => return Ok(ActionRecyclingMyNameActionRecyclingPostResponse::ItemNotFound),
        };

        let craft_items = match &item.craft {
            None | Some(Nullable::Null) => return Ok(ActionRecyclingMyNameActionRecyclingPostResponse::ThisItemCannotBeRecycled),
            Some(Nullable::Present(craft)) => craft.items.clone().unwrap_or_default(),
        };

        if craft_items.is_empty() {
            return Ok(ActionRecyclingMyNameActionRecyclingPostResponse::ThisItemCannotBeRecycled);
        }

        let reward_item = &craft_items[0];

        for _ in 0..recycling_schema.quantity.unwrap_or(1) {
            state.remove_item(&name, &recycling_schema.code)?;
            state.give_item(&name, &reward_item.code)?;
        }

        Ok(ActionRecyclingMyNameActionRecyclingPostResponse::TheItemsWereSuccessfullyRecycled(RecyclingResponseSchema {
            data: RecyclingDataSchema {
                cooldown: empty_cooldown(),
                character: state.get_character(&name)?.clone(),
                details: RecyclingItemsSchema {
                    items: vec![
                        DropSchema {
                            code: reward_item.code.clone(),
                            quantity: recycling_schema.quantity.unwrap_or(1) as i32,
                        },
                    ],
                },
            }
        }))
    }

    /// Action Task Exchange
    async fn action_task_exchange_my_name_action_task_exchange_post(
        &self,
        name: String,
        context: &C,
    ) -> Result<ActionTaskExchangeMyNameActionTaskExchangePostResponse, ApiError> {
        info!(
            "action_task_exchange_my_name_action_task_exchange_post(\"{}\") - X-Span-ID: {:?}",
            name,
            context.get().0.clone()
        );
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Action Unequip Item
    async fn action_unequip_item_my_name_action_unequip_post(
        &self,
        name: String,
        unequip_schema: models::UnequipSchema,
        context: &C,
    ) -> Result<ActionUnequipItemMyNameActionUnequipPostResponse, ApiError> {
        info!(
            "action_unequip_item_my_name_action_unequip_post(\"{}\", {:?}) - X-Span-ID: {:?}",
            name,
            unequip_schema,
            context.get().0.clone()
        );

        let code = {
            let mut state = self.state.lock().unwrap();

            let character = state.get_character_mut(&name)?;

            let code = match unequip_schema.slot.as_str() {
                "weapon" => {
                    let code = character.weapon_slot.clone();
                    character.weapon_slot = String::new();
                    code
                }
                "shield" => {
                    let code = character.shield_slot.clone();
                    character.shield_slot = String::new(); 
                    code
                }
                "helmet" => {
                    let code = character.helmet_slot.clone();
                    character.helmet_slot = String::new(); 
                    code
                }
                "body_armor" => {
                    let code = character.body_armor_slot.clone();
                    character.body_armor_slot = String::new(); 
                    code
                }
                "leg_armor" => {
                    let code = character.leg_armor_slot.clone();
                    character.leg_armor_slot = String::new(); 
                    code
                }
                "boots" => {
                    let code = character.boots_slot.clone();
                    character.boots_slot = String::new(); 
                    code
                }
                "ring1" => {
                    let code = character.ring1_slot.clone();
                    character.ring1_slot = String::new(); 
                    code
                }
                "ring2" => {
                    let code = character.ring2_slot.clone();
                    character.ring2_slot = String::new(); 
                    code
                }
                "amulet" => {
                    let code = character.amulet_slot.clone();
                    character.amulet_slot = String::new(); 
                    code
                }
                "artifact1" => {
                    let code = character.artifact1_slot.clone();
                    character.artifact1_slot = String::new(); 
                    code
                }
                "artifact2" => {
                    let code = character.artifact2_slot.clone();
                    character.artifact2_slot = String::new(); 
                    code
                }
                "artifact3" => {
                    let code = character.artifact3_slot.clone();
                    character.artifact3_slot = String::new(); 
                    code
                }
                "consumable1" => {
                    let code = character.consumable1_slot.clone();
                    character.consumable1_slot = String::new(); 
                    code
                }
                "consumable2" => {
                    let code = character.consumable2_slot.clone();
                    character.consumable2_slot = String::new(); 
                    code
                }
                _ => return Err(ApiError("Api-Error: invalid slot".into())),
            };

            state.give_item(&name, &code)?;

            code
        };

        let state = self.state.lock().unwrap();
        let character = state.get_character(&name)?;
        
        Ok(ActionUnequipItemMyNameActionUnequipPostResponse::TheItemHasBeenSuccessfullyUnequippedAndAddedInHisInventory(EquipmentResponseSchema {
            data: EquipRequestSchema {
                cooldown: empty_cooldown(),
                slot: unequip_schema.slot,
                item: state.get_item(&code)?.unwrap().clone(),
                character: character.clone(),
            },
        }))
    }

    /// Action Withdraw Bank Gold
    async fn action_withdraw_bank_gold_my_name_action_bank_withdraw_gold_post(
        &self,
        name: String,
        deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
        context: &C,
    ) -> Result<ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse, ApiError> {
        info!("action_withdraw_bank_gold_my_name_action_bank_withdraw_gold_post(\"{}\", {:?}) - X-Span-ID: {:?}", name, deposit_withdraw_gold_schema, context.get().0.clone());

        let mut state = self.state.lock().unwrap();

        let character = state.get_character_mut(&name)?;
        character.gold += deposit_withdraw_gold_schema.quantity as i32;

        state.gold -= deposit_withdraw_gold_schema.quantity;

        Ok(ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::GoldsSuccessfullyWithdrawFromYourBank(GoldResponseSchema {
            data: GoldTransactionSchema {
                cooldown: empty_cooldown(),
                bank: GoldSchema {
                    quantity: state.gold as u32,
                },
                character: state.get_character(&name)?.clone(),
            },
        }))
    }

    /// Action Withdraw Bank
    async fn action_withdraw_bank_my_name_action_bank_withdraw_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
        context: &C,
    ) -> Result<ActionWithdrawBankMyNameActionBankWithdrawPostResponse, ApiError> {
        info!("action_withdraw_bank_my_name_action_bank_withdraw_post(\"{}\", {:?}) - X-Span-ID: {:?}", name, simple_item_schema, context.get().0.clone());

        {
            let mut state = self.state.lock().unwrap();

            for _ in 0..simple_item_schema.quantity {
                state.give_item(&name, &simple_item_schema.code)?;
            }

            let mut new_bank_items = vec![];

            for bank_item in &state.bank_items {
                if bank_item.code != simple_item_schema.code {
                    new_bank_items.push(bank_item.clone());
                    continue;
                }

                if bank_item.quantity > simple_item_schema.quantity {
                    new_bank_items.push(SimpleItemSchema {
                        code: simple_item_schema.code.clone(),
                        quantity: bank_item.quantity - simple_item_schema.quantity,
                    });
                    continue;
                }

                // remove item
            }

            state.bank_items = new_bank_items;
        }

        let state = self.state.lock().unwrap();

        Ok(ActionWithdrawBankMyNameActionBankWithdrawPostResponse::ItemSuccessfullyWithdrawFromYourBank(ActionItemBankResponseSchema {
            data: BankItemSchema {
                cooldown: empty_cooldown(),
                item: state.get_item(&simple_item_schema.code)?.unwrap().clone(),
                bank: state.bank_items.clone(),
                character: state.get_character(&name)?.clone(),
            },
        }))
    }

    /// Get All Characters Log,
    async fn get_all_characters_logs_my_logs_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllCharactersLogsMyLogsGetResponse, ApiError> {
        info!(
            "get_all_characters_logs_my_logs_get({:?}, {:?}) - X-Span-ID: {:?}",
            page,
            size,
            context.get().0.clone()
        );

        Ok(
            GetAllCharactersLogsMyLogsGetResponse::SuccessfullyFetchedLogs(DataPageLogSchema {
                data: vec![],
                total: swagger::Nullable::Present(0),
                page: swagger::Nullable::Present(0),
                size: swagger::Nullable::Present(0),
                pages: Some(Nullable::Present(1)),
            }),
        )
    }

    /// Get My Characters
    async fn get_my_characters_my_characters_get(
        &self,
        context: &C,
    ) -> Result<GetMyCharactersMyCharactersGetResponse, ApiError> {
        info!(
            "get_my_characters_my_characters_get() - X-Span-ID: {:?}",
            context.get().0.clone()
        );

        let characters = self
            .state
            .lock()
            .unwrap()
            .characters
            .values()
            .cloned()
            .collect();

        Ok(
            GetMyCharactersMyCharactersGetResponse::SuccessfullyFetchedCharacters(
                MyCharactersListSchema { data: characters },
            ),
        )
    }

    /// Get All Resources
    async fn get_all_resources_resources_get(
        &self,
        min_level: Option<i32>,
        max_level: Option<i32>,
        skill: Option<String>,
        drop: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllResourcesResourcesGetResponse, ApiError> {
        info!(
            "get_all_resources_resources_get({:?}, {:?}, {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}",
            min_level,
            max_level,
            skill,
            drop,
            page,
            size,
            context.get().0.clone()
        );

        let state = self.state.lock().unwrap();

        Ok(
            GetAllResourcesResourcesGetResponse::SuccessfullyFetchedResourcesDetails(
                DataPageResourceSchema {
                    data: state.resources.clone(),
                    pages: Some(Nullable::Present(1)),
                    ..Default::default()
                },
            ),
        )

        // Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Generate Token
    async fn generate_token_token_post(
        &self,
        context: &C,
    ) -> Result<GenerateTokenTokenPostResponse, ApiError> {
        info!(
            "generate_token_token_post() - X-Span-ID: {:?}",
            context.get().0.clone()
        );
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }
}
