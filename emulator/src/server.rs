//! Main library entry point for artifacts_openapi implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use chrono::Utc;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::header::GetAll;
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use predicates::prelude::*;
use serde_json::json;
use std::collections::HashMap;
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
    self, CharacterFightDataSchema, CharacterFightResponseSchema, CharacterMovementDataSchema, CharacterMovementResponseSchema, CharacterSchema, CooldownSchema, CraftSchema, CraftingSchema, DataPageActiveEventSchema, DataPageItemSchema, DataPageLogSchema, DataPageMapSchema, DataPageResourceSchema, DataPageSimpleItemSchema, DropSchema, FightSchema, GoldBankResponseSchema, GoldSchema, InventorySlot, ItemResponseSchema, ItemSchema, LogSchema, MapSchema, MonsterResponseSchema, MonsterSchema, MyCharactersListSchema, ResourceResponseSchema, ResourceSchema, SingleItemSchema, SkillDataSchema, SkillInfoSchema, SkillResponseSchema
};

pub async fn create(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse().expect("Failed to parse bind address");

    let state = Arc::new(Mutex::new(State::from_seed().await?));
    state
        .lock()
        .unwrap()
        .create_character("emulated123", "men1")
        .unwrap();

    let server = Server::new(state.clone());

    let service = MakeService::new(server);

    let service =
        artifacts_openapi::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    info!("Starting a server (over http, so no TLS)");
    // Using HTTP
    hyper::server::Server::bind(&addr).serve(service).await?;

    Ok(())
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get Status
    async fn get_status_get(&self, context: &C) -> Result<GetStatusGetResponse, ApiError> {
        info!(
            "get_status_get() - X-Span-ID: {:?}",
            context.get().0.clone()
        );
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
                    ..Default::default()
                },
            ),
        )

        // Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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

        Ok(GetItemItemsCodeGetResponse::SuccessfullyFetchedItem(
            ItemResponseSchema {
                data: SingleItemSchema {
                    item: item.clone(),
                    ge: None,
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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

        Ok(
            GetMonsterMonstersCodeGetResponse::SuccessfullyFetchedMonster(MonsterResponseSchema {
                data: MonsterSchema {
                    name: "Yellow Slime".into(),
                    code: "yellow_slime".into(),
                    level: 2,
                    hp: 100,
                    ..Default::default()
                },
            }),
        )

        // Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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

        Ok(GetBankGoldsMyBankGoldGetResponse::SuccessfullyFetchedGolds(
            GoldBankResponseSchema {
                data: GoldSchema {
                    quantity: 999999999,
                },
            },
        ))

        // Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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

        Ok(
            GetBankItemsMyBankItemsGetResponse::SuccessfullyFetchedItems(
                DataPageSimpleItemSchema {
                    data: vec![],
                    ..Default::default()
                },
            ),
        )

        // Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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

        if let Some(skill) = skill {
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
        }

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

        let character = state.get_character(&name)?;

        Ok(
            ActionCraftingMyNameActionCraftingPostResponse::TheItemWasSuccessfullyCrafted(
                SkillResponseSchema {
                    data: SkillDataSchema {
                        cooldown: Default::default(),
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
                        cooldown: CooldownSchema {
                            total_seconds: 0,
                            remaining_seconds: 0,
                            expiration: Utc::now(),
                            reason: "k".into(),
                            ..Default::default()
                        },
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
                cooldown: CooldownSchema{ total_seconds: 0, remaining_seconds: 0, expiration: Utc::now(), reason: "k".into(), ..Default::default() },
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
                        cooldown: CooldownSchema {
                            total_seconds: 0,
                            remaining_seconds: 0,
                            expiration: Utc::now(),
                            reason: "k".into(),
                            ..Default::default()
                        },
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Action Withdraw Bank Gold
    async fn action_withdraw_bank_gold_my_name_action_bank_withdraw_gold_post(
        &self,
        name: String,
        deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
        context: &C,
    ) -> Result<ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse, ApiError> {
        info!("action_withdraw_bank_gold_my_name_action_bank_withdraw_gold_post(\"{}\", {:?}) - X-Span-ID: {:?}", name, deposit_withdraw_gold_schema, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Action Withdraw Bank
    async fn action_withdraw_bank_my_name_action_bank_withdraw_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
        context: &C,
    ) -> Result<ActionWithdrawBankMyNameActionBankWithdrawPostResponse, ApiError> {
        info!("action_withdraw_bank_my_name_action_bank_withdraw_post(\"{}\", {:?}) - X-Span-ID: {:?}", name, simple_item_schema, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
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
                pages: None,
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

        Ok(
            GetAllResourcesResourcesGetResponse::SuccessfullyFetchedResourcesDetails(
                DataPageResourceSchema::default(),
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
