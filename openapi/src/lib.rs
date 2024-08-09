#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    unused_attributes,
    non_camel_case_types
)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use crate::server::Authorization;
use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::error::Error;
use std::task::{Context, Poll};
use swagger::{ApiError, ContextWrapper};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "1.6";

mod auth;
pub use auth::{AuthenticationApi, Claims};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateAccountAccountsCreatePostResponse {
    /// Account created successfully.
    AccountCreatedSuccessfully(models::ResponseSchema),
    /// Username already used.
    UsernameAlreadyUsed,
    /// Email already used.
    EmailAlreadyUsed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateCharacterCharactersCreatePostResponse {
    /// Successfully created character.
    SuccessfullyCreatedCharacter(models::CharacterResponseSchema),
    /// Name already used.
    NameAlreadyUsed,
    /// Maximum characters reached on your account.
    MaximumCharactersReachedOnYourAccount,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteCharacterCharactersDeletePostResponse {
    /// Successfully deleted character.
    SuccessfullyDeletedCharacter(models::CharacterResponseSchema),
    /// Character not found.
    CharacterNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetAllCharactersCharactersGetResponse {
    /// Successfully fetched characters details.
    SuccessfullyFetchedCharactersDetails(models::DataPageCharacterSchema),
    /// Characters not found.
    CharactersNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetCharacterCharactersNameGetResponse {
    /// Successfully fetched character.
    SuccessfullyFetchedCharacter(models::CharacterResponseSchema),
    /// Character not found.
    CharacterNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetStatusGetResponse {
    /// Successful Response
    SuccessfulResponse(models::StatusResponseSchema),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetAllEventsEventsGetResponse {
    /// Successfully fetched events details.
    SuccessfullyFetchedEventsDetails(models::DataPageActiveEventSchema),
    /// Events not found.
    EventsNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetAllGeItemsGeGetResponse {
    /// Fetch Grand Exchange items details.
    FetchGrandExchangeItemsDetails(models::DataPageGeItemSchema),
    /// Item not found.
    ItemNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetGeItemGeCodeGetResponse {
    /// Successfully fetched Grand Exchange item.
    SuccessfullyFetchedGrandExchangeItem(models::GeItemResponseSchema),
    /// Item not found.
    ItemNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetAllItemsItemsGetResponse {
    /// Fetch items details.
    FetchItemsDetails(models::DataPageItemSchema),
    /// Items not found.
    ItemsNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetItemItemsCodeGetResponse {
    /// Successfully fetched item.
    SuccessfullyFetchedItem(models::ItemResponseSchema),
    /// Item not found.
    ItemNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetAllMapsMapsGetResponse {
    /// Successfully fetched maps details.
    SuccessfullyFetchedMapsDetails(models::DataPageMapSchema),
    /// Maps not found.
    MapsNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetMapMapsXyGetResponse {
    /// Successfully fetched map.
    SuccessfullyFetchedMap(models::MapResponseSchema),
    /// Map not found.
    MapNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetAllMonstersMonstersGetResponse {
    /// Successfully fetched monsters details.
    SuccessfullyFetchedMonstersDetails(models::DataPageMonsterSchema),
    /// Monsters not found.
    MonstersNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetMonsterMonstersCodeGetResponse {
    /// Successfully fetched monster.
    SuccessfullyFetchedMonster(models::MonsterResponseSchema),
    /// Monster not found.
    MonsterNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ChangePasswordMyChangePasswordPostResponse {
    /// Password changed successfully.
    PasswordChangedSuccessfully(models::ResponseSchema),
    /// Use a different password.
    UseADifferentPassword,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetBankGoldsMyBankGoldGetResponse {
    /// Successfully fetched golds.
    SuccessfullyFetchedGolds(models::GoldBankResponseSchema),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetBankItemsMyBankItemsGetResponse {
    /// Successfully fetched items.
    SuccessfullyFetchedItems(models::DataPageSimpleItemSchema),
    /// Items not found.
    ItemsNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionAcceptNewTaskMyNameActionTaskNewPostResponse {
    /// New task successfully accepted.
    NewTaskSuccessfullyAccepted(models::TaskResponseSchema),
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Tasks Master not found on this map.
    TasksMasterNotFoundOnThisMap,
    /// Character already has a task.
    CharacterAlreadyHasATask,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionCompleteTaskMyNameActionTaskCompletePostResponse {
    /// The task has been successfully completed.
    TheTaskHasBeenSuccessfullyCompleted(models::TaskRewardResponseSchema),
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Tasks Master not found on this map.
    TasksMasterNotFoundOnThisMap,
    /// Character has not completed the task.
    CharacterHasNotCompletedTheTask,
    /// Character has no task.
    CharacterHasNoTask,
    /// Character inventory is full.
    CharacterInventoryIsFull,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionCraftingMyNameActionCraftingPostResponse {
    /// The item was successfully crafted.
    TheItemWasSuccessfullyCrafted(models::SkillResponseSchema),
    /// Craft not found.
    CraftNotFound,
    /// Workshop not found on this map.
    WorkshopNotFoundOnThisMap,
    /// Character not found.
    CharacterNotFound,
    /// Character inventory is full.
    CharacterInventoryIsFull,
    /// Character in cooldown.
    CharacterInCooldown,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Not skill level required.
    NotSkillLevelRequired,
    /// Missing item or insufficient quantity in your inventory.
    MissingItemOrInsufficientQuantityInYourInventory,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionDeleteItemMyNameActionDeletePostResponse {
    /// Item successfully deleted from your character.
    ItemSuccessfullyDeletedFromYourCharacter(models::DeleteItemResponseSchema),
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Missing item or insufficient quantity in your inventory.
    MissingItemOrInsufficientQuantityInYourInventory,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse {
    /// Golds successfully deposited in your bank.
    GoldsSuccessfullyDepositedInYourBank(models::GoldResponseSchema),
    /// Bank not found on this map.
    BankNotFoundOnThisMap,
    /// Insufficient golds on your character.
    InsufficientGoldsOnYourCharacter,
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// A transaction is already in progress with this item/your golds in your bank.
    ATransactionIsAlreadyInProgressWithThisItem,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionDepositBankMyNameActionBankDepositPostResponse {
    /// Item successfully deposited in your bank.
    ItemSuccessfullyDepositedInYourBank(models::ActionItemBankResponseSchema),
    /// Bank not found on this map.
    BankNotFoundOnThisMap,
    /// Item not found.
    ItemNotFound,
    /// A transaction is already in progress with this item/your golds in your bank.
    ATransactionIsAlreadyInProgressWithThisItem,
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Missing item or insufficient quantity in your inventory.
    MissingItemOrInsufficientQuantityInYourInventory,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionEquipItemMyNameActionEquipPostResponse {
    /// The item has been successfully equipped on your character.
    TheItemHasBeenSuccessfullyEquippedOnYourCharacter(models::EquipmentResponseSchema),
    /// Item not found.
    ItemNotFound,
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Missing item or insufficient quantity in your inventory.
    MissingItemOrInsufficientQuantityInYourInventory,
    /// Character level is insufficient.
    CharacterLevelIsInsufficient,
    /// Slot is not empty.
    SlotIsNotEmpty,
    /// This item is already equipped.
    ThisItemIsAlreadyEquipped,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionFightMyNameActionFightPostResponse {
    /// The fight ended successfully.
    TheFightEndedSuccessfully(models::CharacterFightResponseSchema),
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// Monster not found on this map.
    MonsterNotFoundOnThisMap,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Character inventory is full.
    CharacterInventoryIsFull,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionGatheringMyNameActionGatheringPostResponse {
    /// The resource has been successfully gathered.
    TheResourceHasBeenSuccessfullyGathered(models::SkillResponseSchema),
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// Resource not found on this map.
    ResourceNotFoundOnThisMap,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Not skill level required.
    NotSkillLevelRequired,
    /// Character inventory is full.
    CharacterInventoryIsFull,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionGeBuyItemMyNameActionGeBuyPostResponse {
    /// Item successfully buy from the Grand Exchange.
    ItemSuccessfullyBuyFromTheGrandExchange(models::GeTransactionResponseSchema),
    /// Grand Exchange not found on this map.
    GrandExchangeNotFoundOnThisMap,
    /// Character not found.
    CharacterNotFound,
    /// Character inventory is full.
    CharacterInventoryIsFull,
    /// Character in cooldown.
    CharacterInCooldown,
    /// A transaction is already in progress on this item by a another character.
    ATransactionIsAlreadyInProgressOnThisItemByAAnotherCharacter,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Insufficient golds on your character.
    InsufficientGoldsOnYourCharacter,
    /// No stock for this item.
    NoStockForThisItem,
    /// No item at this price.
    NoItemAtThisPrice,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionGeSellItemMyNameActionGeSellPostResponse {
    /// Item successfully sell at the Grand Exchange.
    ItemSuccessfullySellAtTheGrandExchange(models::GeTransactionResponseSchema),
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// Item not found.
    ItemNotFound,
    /// A transaction is already in progress on this item by a another character.
    ATransactionIsAlreadyInProgressOnThisItemByAAnotherCharacter,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Missing item or insufficient quantity in your inventory.
    MissingItemOrInsufficientQuantityInYourInventory,
    /// No item at this price.
    NoItemAtThisPrice,
    /// Grand Exchange not found on this map.
    GrandExchangeNotFoundOnThisMap,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionMoveMyNameActionMovePostResponse {
    /// The character has moved successfully.
    TheCharacterHasMovedSuccessfully(models::CharacterMovementResponseSchema),
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// Character already at destination.
    CharacterAlreadyAtDestination,
    /// Map not found.
    MapNotFound,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionRecyclingMyNameActionRecyclingPostResponse {
    /// The items were successfully recycled.
    TheItemsWereSuccessfullyRecycled(models::RecyclingResponseSchema),
    /// Item not found.
    ItemNotFound,
    /// Workshop not found on this map.
    WorkshopNotFoundOnThisMap,
    /// Character not found.
    CharacterNotFound,
    /// Character inventory is full.
    CharacterInventoryIsFull,
    /// Character in cooldown.
    CharacterInCooldown,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Not skill level required.
    NotSkillLevelRequired,
    /// Missing item or insufficient quantity in your inventory.
    MissingItemOrInsufficientQuantityInYourInventory,
    /// This item cannot be recycled.
    ThisItemCannotBeRecycled,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionTaskExchangeMyNameActionTaskExchangePostResponse {
    /// The tasks coins have been successfully exchanged.
    TheTasksCoinsHaveBeenSuccessfullyExchanged(models::TaskRewardResponseSchema),
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Tasks Master not found on this map.
    TasksMasterNotFoundOnThisMap,
    /// Missing item or insufficient quantity in your inventory.
    MissingItemOrInsufficientQuantityInYourInventory,
    /// Character inventory is full.
    CharacterInventoryIsFull,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionUnequipItemMyNameActionUnequipPostResponse {
    /// The item has been successfully unequipped and added in his inventory.
    TheItemHasBeenSuccessfullyUnequippedAndAddedInHisInventory(models::EquipmentResponseSchema),
    /// Item not found.
    ItemNotFound,
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Slot is empty.
    SlotIsEmpty,
    /// Character inventory is full.
    CharacterInventoryIsFull,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse {
    /// Golds successfully withdraw from your bank.
    GoldsSuccessfullyWithdrawFromYourBank(models::GoldResponseSchema),
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// A transaction is already in progress with this item/your golds in your bank.
    ATransactionIsAlreadyInProgressWithThisItem,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Bank not found on this map.
    BankNotFoundOnThisMap,
    /// Insufficient golds in your bank.
    InsufficientGoldsInYourBank,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ActionWithdrawBankMyNameActionBankWithdrawPostResponse {
    /// Item successfully withdraw from your bank.
    ItemSuccessfullyWithdrawFromYourBank(models::ActionItemBankResponseSchema),
    /// Item not found.
    ItemNotFound,
    /// Character not found.
    CharacterNotFound,
    /// Character in cooldown.
    CharacterInCooldown,
    /// A transaction is already in progress with this item/your golds in your bank.
    ATransactionIsAlreadyInProgressWithThisItem,
    /// An action is already in progress by your character.
    AnActionIsAlreadyInProgressByYourCharacter,
    /// Character inventory is full.
    CharacterInventoryIsFull,
    /// Bank not found on this map.
    BankNotFoundOnThisMap,
    /// Missing item or insufficient quantity in your inventory.
    MissingItemOrInsufficientQuantityInYourInventory,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetAllCharactersLogsMyLogsGetResponse {
    /// Successfully fetched logs.
    SuccessfullyFetchedLogs(models::DataPageLogSchema),
    /// Logs not found.
    LogsNotFound,
    /// Character not found.
    CharacterNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetMyCharactersMyCharactersGetResponse {
    /// Successfully fetched characters.
    SuccessfullyFetchedCharacters(models::MyCharactersListSchema),
    /// Characters not found.
    CharactersNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetAllResourcesResourcesGetResponse {
    /// Successfully fetched resources details.
    SuccessfullyFetchedResourcesDetails(models::DataPageResourceSchema),
    /// Resources not found.
    ResourcesNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetResourceResourcesCodeGetResponse {
    /// Successfully fetched resource.
    SuccessfullyFetchedResource(models::ResourceResponseSchema),
    /// Ressource not found.
    RessourceNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GenerateTokenTokenPostResponse {
    /// Token generated successfully
    TokenGeneratedSuccessfully(models::TokenResponseSchema),
    /// Token generation failed.
    TokenGenerationFailed,
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Create Account
    async fn create_account_accounts_create_post(
        &self,
        add_account_schema: models::AddAccountSchema,
        context: &C,
    ) -> Result<CreateAccountAccountsCreatePostResponse, ApiError>;

    /// Create Character
    async fn create_character_characters_create_post(
        &self,
        add_character_schema: models::AddCharacterSchema,
        context: &C,
    ) -> Result<CreateCharacterCharactersCreatePostResponse, ApiError>;

    /// Delete Character
    async fn delete_character_characters_delete_post(
        &self,
        delete_character_schema: models::DeleteCharacterSchema,
        context: &C,
    ) -> Result<DeleteCharacterCharactersDeletePostResponse, ApiError>;

    /// Get All Characters
    async fn get_all_characters_characters_get(
        &self,
        sort: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllCharactersCharactersGetResponse, ApiError>;

    /// Get Character
    async fn get_character_characters_name_get(
        &self,
        name: String,
        context: &C,
    ) -> Result<GetCharacterCharactersNameGetResponse, ApiError>;

    /// Get Status
    async fn get_status_get(&self, context: &C) -> Result<GetStatusGetResponse, ApiError>;

    /// Get All Events
    async fn get_all_events_events_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllEventsEventsGetResponse, ApiError>;

    /// Get All Ge Items
    async fn get_all_ge_items_ge_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllGeItemsGeGetResponse, ApiError>;

    /// Get Ge Item
    async fn get_ge_item_ge_code_get(
        &self,
        code: String,
        context: &C,
    ) -> Result<GetGeItemGeCodeGetResponse, ApiError>;

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
    ) -> Result<GetAllItemsItemsGetResponse, ApiError>;

    /// Get Item
    async fn get_item_items_code_get(
        &self,
        code: String,
        context: &C,
    ) -> Result<GetItemItemsCodeGetResponse, ApiError>;

    /// Get All Maps
    async fn get_all_maps_maps_get(
        &self,
        content_type: Option<String>,
        content_code: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllMapsMapsGetResponse, ApiError>;

    /// Get Map
    async fn get_map_maps_xy_get(
        &self,
        x: i32,
        y: i32,
        context: &C,
    ) -> Result<GetMapMapsXyGetResponse, ApiError>;

    /// Get All Monsters
    async fn get_all_monsters_monsters_get(
        &self,
        min_level: Option<i32>,
        max_level: Option<i32>,
        drop: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllMonstersMonstersGetResponse, ApiError>;

    /// Get Monster
    async fn get_monster_monsters_code_get(
        &self,
        code: String,
        context: &C,
    ) -> Result<GetMonsterMonstersCodeGetResponse, ApiError>;

    /// Change Password
    async fn change_password_my_change_password_post(
        &self,
        change_password: models::ChangePassword,
        context: &C,
    ) -> Result<ChangePasswordMyChangePasswordPostResponse, ApiError>;

    /// Get Bank Golds
    async fn get_bank_golds_my_bank_gold_get(
        &self,
        context: &C,
    ) -> Result<GetBankGoldsMyBankGoldGetResponse, ApiError>;

    /// Get Bank Items
    async fn get_bank_items_my_bank_items_get(
        &self,
        item_code: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetBankItemsMyBankItemsGetResponse, ApiError>;

    /// Action Accept New Task
    async fn action_accept_new_task_my_name_action_task_new_post(
        &self,
        name: String,
        context: &C,
    ) -> Result<ActionAcceptNewTaskMyNameActionTaskNewPostResponse, ApiError>;

    /// Action Complete Task
    async fn action_complete_task_my_name_action_task_complete_post(
        &self,
        name: String,
        context: &C,
    ) -> Result<ActionCompleteTaskMyNameActionTaskCompletePostResponse, ApiError>;

    /// Action Crafting
    async fn action_crafting_my_name_action_crafting_post(
        &self,
        name: String,
        crafting_schema: models::CraftingSchema,
        context: &C,
    ) -> Result<ActionCraftingMyNameActionCraftingPostResponse, ApiError>;

    /// Action Delete Item
    async fn action_delete_item_my_name_action_delete_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
        context: &C,
    ) -> Result<ActionDeleteItemMyNameActionDeletePostResponse, ApiError>;

    /// Action Deposit Bank Gold
    async fn action_deposit_bank_gold_my_name_action_bank_deposit_gold_post(
        &self,
        name: String,
        deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
        context: &C,
    ) -> Result<ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse, ApiError>;

    /// Action Deposit Bank
    async fn action_deposit_bank_my_name_action_bank_deposit_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
        context: &C,
    ) -> Result<ActionDepositBankMyNameActionBankDepositPostResponse, ApiError>;

    /// Action Equip Item
    async fn action_equip_item_my_name_action_equip_post(
        &self,
        name: String,
        equip_schema: models::EquipSchema,
        context: &C,
    ) -> Result<ActionEquipItemMyNameActionEquipPostResponse, ApiError>;

    /// Action Fight
    async fn action_fight_my_name_action_fight_post(
        &self,
        name: String,
        context: &C,
    ) -> Result<ActionFightMyNameActionFightPostResponse, ApiError>;

    /// Action Gathering
    async fn action_gathering_my_name_action_gathering_post(
        &self,
        name: String,
        context: &C,
    ) -> Result<ActionGatheringMyNameActionGatheringPostResponse, ApiError>;

    /// Action Ge Buy Item
    async fn action_ge_buy_item_my_name_action_ge_buy_post(
        &self,
        name: String,
        ge_transaction_item_schema: models::GeTransactionItemSchema,
        context: &C,
    ) -> Result<ActionGeBuyItemMyNameActionGeBuyPostResponse, ApiError>;

    /// Action Ge Sell Item
    async fn action_ge_sell_item_my_name_action_ge_sell_post(
        &self,
        name: String,
        ge_transaction_item_schema: models::GeTransactionItemSchema,
        context: &C,
    ) -> Result<ActionGeSellItemMyNameActionGeSellPostResponse, ApiError>;

    /// Action Move
    async fn action_move_my_name_action_move_post(
        &self,
        name: String,
        destination_schema: models::DestinationSchema,
        context: &C,
    ) -> Result<ActionMoveMyNameActionMovePostResponse, ApiError>;

    /// Action Recycling
    async fn action_recycling_my_name_action_recycling_post(
        &self,
        name: String,
        recycling_schema: models::RecyclingSchema,
        context: &C,
    ) -> Result<ActionRecyclingMyNameActionRecyclingPostResponse, ApiError>;

    /// Action Task Exchange
    async fn action_task_exchange_my_name_action_task_exchange_post(
        &self,
        name: String,
        context: &C,
    ) -> Result<ActionTaskExchangeMyNameActionTaskExchangePostResponse, ApiError>;

    /// Action Unequip Item
    async fn action_unequip_item_my_name_action_unequip_post(
        &self,
        name: String,
        unequip_schema: models::UnequipSchema,
        context: &C,
    ) -> Result<ActionUnequipItemMyNameActionUnequipPostResponse, ApiError>;

    /// Action Withdraw Bank Gold
    async fn action_withdraw_bank_gold_my_name_action_bank_withdraw_gold_post(
        &self,
        name: String,
        deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
        context: &C,
    ) -> Result<ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse, ApiError>;

    /// Action Withdraw Bank
    async fn action_withdraw_bank_my_name_action_bank_withdraw_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
        context: &C,
    ) -> Result<ActionWithdrawBankMyNameActionBankWithdrawPostResponse, ApiError>;

    /// Get All Characters Logs
    async fn get_all_characters_logs_my_logs_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
        context: &C,
    ) -> Result<GetAllCharactersLogsMyLogsGetResponse, ApiError>;

    /// Get My Characters
    async fn get_my_characters_my_characters_get(
        &self,
        context: &C,
    ) -> Result<GetMyCharactersMyCharactersGetResponse, ApiError>;

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
    ) -> Result<GetAllResourcesResourcesGetResponse, ApiError>;

    /// Get Resource
    async fn get_resource_resources_code_get(
        &self,
        code: String,
        context: &C,
    ) -> Result<GetResourceResourcesCodeGetResponse, ApiError>;

    /// Generate Token
    async fn generate_token_token_post(
        &self,
        context: &C,
    ) -> Result<GenerateTokenTokenPostResponse, ApiError>;
}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Create Account
    async fn create_account_accounts_create_post(
        &self,
        add_account_schema: models::AddAccountSchema,
    ) -> Result<CreateAccountAccountsCreatePostResponse, ApiError>;

    /// Create Character
    async fn create_character_characters_create_post(
        &self,
        add_character_schema: models::AddCharacterSchema,
    ) -> Result<CreateCharacterCharactersCreatePostResponse, ApiError>;

    /// Delete Character
    async fn delete_character_characters_delete_post(
        &self,
        delete_character_schema: models::DeleteCharacterSchema,
    ) -> Result<DeleteCharacterCharactersDeletePostResponse, ApiError>;

    /// Get All Characters
    async fn get_all_characters_characters_get(
        &self,
        sort: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllCharactersCharactersGetResponse, ApiError>;

    /// Get Character
    async fn get_character_characters_name_get(
        &self,
        name: String,
    ) -> Result<GetCharacterCharactersNameGetResponse, ApiError>;

    /// Get Status
    async fn get_status_get(&self) -> Result<GetStatusGetResponse, ApiError>;

    /// Get All Events
    async fn get_all_events_events_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllEventsEventsGetResponse, ApiError>;

    /// Get All Ge Items
    async fn get_all_ge_items_ge_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllGeItemsGeGetResponse, ApiError>;

    /// Get Ge Item
    async fn get_ge_item_ge_code_get(
        &self,
        code: String,
    ) -> Result<GetGeItemGeCodeGetResponse, ApiError>;

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
    ) -> Result<GetAllItemsItemsGetResponse, ApiError>;

    /// Get Item
    async fn get_item_items_code_get(
        &self,
        code: String,
    ) -> Result<GetItemItemsCodeGetResponse, ApiError>;

    /// Get All Maps
    async fn get_all_maps_maps_get(
        &self,
        content_type: Option<String>,
        content_code: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllMapsMapsGetResponse, ApiError>;

    /// Get Map
    async fn get_map_maps_xy_get(
        &self,
        x: i32,
        y: i32,
    ) -> Result<GetMapMapsXyGetResponse, ApiError>;

    /// Get All Monsters
    async fn get_all_monsters_monsters_get(
        &self,
        min_level: Option<i32>,
        max_level: Option<i32>,
        drop: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllMonstersMonstersGetResponse, ApiError>;

    /// Get Monster
    async fn get_monster_monsters_code_get(
        &self,
        code: String,
    ) -> Result<GetMonsterMonstersCodeGetResponse, ApiError>;

    /// Change Password
    async fn change_password_my_change_password_post(
        &self,
        change_password: models::ChangePassword,
    ) -> Result<ChangePasswordMyChangePasswordPostResponse, ApiError>;

    /// Get Bank Golds
    async fn get_bank_golds_my_bank_gold_get(
        &self,
    ) -> Result<GetBankGoldsMyBankGoldGetResponse, ApiError>;

    /// Get Bank Items
    async fn get_bank_items_my_bank_items_get(
        &self,
        item_code: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetBankItemsMyBankItemsGetResponse, ApiError>;

    /// Action Accept New Task
    async fn action_accept_new_task_my_name_action_task_new_post(
        &self,
        name: String,
    ) -> Result<ActionAcceptNewTaskMyNameActionTaskNewPostResponse, ApiError>;

    /// Action Complete Task
    async fn action_complete_task_my_name_action_task_complete_post(
        &self,
        name: String,
    ) -> Result<ActionCompleteTaskMyNameActionTaskCompletePostResponse, ApiError>;

    /// Action Crafting
    async fn action_crafting_my_name_action_crafting_post(
        &self,
        name: String,
        crafting_schema: models::CraftingSchema,
    ) -> Result<ActionCraftingMyNameActionCraftingPostResponse, ApiError>;

    /// Action Delete Item
    async fn action_delete_item_my_name_action_delete_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
    ) -> Result<ActionDeleteItemMyNameActionDeletePostResponse, ApiError>;

    /// Action Deposit Bank Gold
    async fn action_deposit_bank_gold_my_name_action_bank_deposit_gold_post(
        &self,
        name: String,
        deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
    ) -> Result<ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse, ApiError>;

    /// Action Deposit Bank
    async fn action_deposit_bank_my_name_action_bank_deposit_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
    ) -> Result<ActionDepositBankMyNameActionBankDepositPostResponse, ApiError>;

    /// Action Equip Item
    async fn action_equip_item_my_name_action_equip_post(
        &self,
        name: String,
        equip_schema: models::EquipSchema,
    ) -> Result<ActionEquipItemMyNameActionEquipPostResponse, ApiError>;

    /// Action Fight
    async fn action_fight_my_name_action_fight_post(
        &self,
        name: String,
    ) -> Result<ActionFightMyNameActionFightPostResponse, ApiError>;

    /// Action Gathering
    async fn action_gathering_my_name_action_gathering_post(
        &self,
        name: String,
    ) -> Result<ActionGatheringMyNameActionGatheringPostResponse, ApiError>;

    /// Action Ge Buy Item
    async fn action_ge_buy_item_my_name_action_ge_buy_post(
        &self,
        name: String,
        ge_transaction_item_schema: models::GeTransactionItemSchema,
    ) -> Result<ActionGeBuyItemMyNameActionGeBuyPostResponse, ApiError>;

    /// Action Ge Sell Item
    async fn action_ge_sell_item_my_name_action_ge_sell_post(
        &self,
        name: String,
        ge_transaction_item_schema: models::GeTransactionItemSchema,
    ) -> Result<ActionGeSellItemMyNameActionGeSellPostResponse, ApiError>;

    /// Action Move
    async fn action_move_my_name_action_move_post(
        &self,
        name: String,
        destination_schema: models::DestinationSchema,
    ) -> Result<ActionMoveMyNameActionMovePostResponse, ApiError>;

    /// Action Recycling
    async fn action_recycling_my_name_action_recycling_post(
        &self,
        name: String,
        recycling_schema: models::RecyclingSchema,
    ) -> Result<ActionRecyclingMyNameActionRecyclingPostResponse, ApiError>;

    /// Action Task Exchange
    async fn action_task_exchange_my_name_action_task_exchange_post(
        &self,
        name: String,
    ) -> Result<ActionTaskExchangeMyNameActionTaskExchangePostResponse, ApiError>;

    /// Action Unequip Item
    async fn action_unequip_item_my_name_action_unequip_post(
        &self,
        name: String,
        unequip_schema: models::UnequipSchema,
    ) -> Result<ActionUnequipItemMyNameActionUnequipPostResponse, ApiError>;

    /// Action Withdraw Bank Gold
    async fn action_withdraw_bank_gold_my_name_action_bank_withdraw_gold_post(
        &self,
        name: String,
        deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
    ) -> Result<ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse, ApiError>;

    /// Action Withdraw Bank
    async fn action_withdraw_bank_my_name_action_bank_withdraw_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
    ) -> Result<ActionWithdrawBankMyNameActionBankWithdrawPostResponse, ApiError>;

    /// Get All Characters Logs
    async fn get_all_characters_logs_my_logs_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllCharactersLogsMyLogsGetResponse, ApiError>;

    /// Get My Characters
    async fn get_my_characters_my_characters_get(
        &self,
    ) -> Result<GetMyCharactersMyCharactersGetResponse, ApiError>;

    /// Get All Resources
    async fn get_all_resources_resources_get(
        &self,
        min_level: Option<i32>,
        max_level: Option<i32>,
        skill: Option<String>,
        drop: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllResourcesResourcesGetResponse, ApiError>;

    /// Get Resource
    async fn get_resource_resources_code_get(
        &self,
        code: String,
    ) -> Result<GetResourceResourcesCodeGetResponse, ApiError>;

    /// Generate Token
    async fn generate_token_token_post(&self) -> Result<GenerateTokenTokenPostResponse, ApiError>;
}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync>
where
    Self: Sized,
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
        ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Create Account
    async fn create_account_accounts_create_post(
        &self,
        add_account_schema: models::AddAccountSchema,
    ) -> Result<CreateAccountAccountsCreatePostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .create_account_accounts_create_post(add_account_schema, &context)
            .await
    }

    /// Create Character
    async fn create_character_characters_create_post(
        &self,
        add_character_schema: models::AddCharacterSchema,
    ) -> Result<CreateCharacterCharactersCreatePostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .create_character_characters_create_post(add_character_schema, &context)
            .await
    }

    /// Delete Character
    async fn delete_character_characters_delete_post(
        &self,
        delete_character_schema: models::DeleteCharacterSchema,
    ) -> Result<DeleteCharacterCharactersDeletePostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_character_characters_delete_post(delete_character_schema, &context)
            .await
    }

    /// Get All Characters
    async fn get_all_characters_characters_get(
        &self,
        sort: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllCharactersCharactersGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_characters_characters_get(sort, page, size, &context)
            .await
    }

    /// Get Character
    async fn get_character_characters_name_get(
        &self,
        name: String,
    ) -> Result<GetCharacterCharactersNameGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_character_characters_name_get(name, &context)
            .await
    }

    /// Get Status
    async fn get_status_get(&self) -> Result<GetStatusGetResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_status_get(&context).await
    }

    /// Get All Events
    async fn get_all_events_events_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllEventsEventsGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_events_events_get(page, size, &context)
            .await
    }

    /// Get All Ge Items
    async fn get_all_ge_items_ge_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllGeItemsGeGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_ge_items_ge_get(page, size, &context)
            .await
    }

    /// Get Ge Item
    async fn get_ge_item_ge_code_get(
        &self,
        code: String,
    ) -> Result<GetGeItemGeCodeGetResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_ge_item_ge_code_get(code, &context).await
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
    ) -> Result<GetAllItemsItemsGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_items_items_get(
                min_level,
                max_level,
                name,
                r#type,
                craft_skill,
                craft_material,
                page,
                size,
                &context,
            )
            .await
    }

    /// Get Item
    async fn get_item_items_code_get(
        &self,
        code: String,
    ) -> Result<GetItemItemsCodeGetResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_item_items_code_get(code, &context).await
    }

    /// Get All Maps
    async fn get_all_maps_maps_get(
        &self,
        content_type: Option<String>,
        content_code: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllMapsMapsGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_maps_maps_get(content_type, content_code, page, size, &context)
            .await
    }

    /// Get Map
    async fn get_map_maps_xy_get(
        &self,
        x: i32,
        y: i32,
    ) -> Result<GetMapMapsXyGetResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_map_maps_xy_get(x, y, &context).await
    }

    /// Get All Monsters
    async fn get_all_monsters_monsters_get(
        &self,
        min_level: Option<i32>,
        max_level: Option<i32>,
        drop: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllMonstersMonstersGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_monsters_monsters_get(min_level, max_level, drop, page, size, &context)
            .await
    }

    /// Get Monster
    async fn get_monster_monsters_code_get(
        &self,
        code: String,
    ) -> Result<GetMonsterMonstersCodeGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_monster_monsters_code_get(code, &context)
            .await
    }

    /// Change Password
    async fn change_password_my_change_password_post(
        &self,
        change_password: models::ChangePassword,
    ) -> Result<ChangePasswordMyChangePasswordPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .change_password_my_change_password_post(change_password, &context)
            .await
    }

    /// Get Bank Golds
    async fn get_bank_golds_my_bank_gold_get(
        &self,
    ) -> Result<GetBankGoldsMyBankGoldGetResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_bank_golds_my_bank_gold_get(&context).await
    }

    /// Get Bank Items
    async fn get_bank_items_my_bank_items_get(
        &self,
        item_code: Option<String>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetBankItemsMyBankItemsGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_bank_items_my_bank_items_get(item_code, page, size, &context)
            .await
    }

    /// Action Accept New Task
    async fn action_accept_new_task_my_name_action_task_new_post(
        &self,
        name: String,
    ) -> Result<ActionAcceptNewTaskMyNameActionTaskNewPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_accept_new_task_my_name_action_task_new_post(name, &context)
            .await
    }

    /// Action Complete Task
    async fn action_complete_task_my_name_action_task_complete_post(
        &self,
        name: String,
    ) -> Result<ActionCompleteTaskMyNameActionTaskCompletePostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_complete_task_my_name_action_task_complete_post(name, &context)
            .await
    }

    /// Action Crafting
    async fn action_crafting_my_name_action_crafting_post(
        &self,
        name: String,
        crafting_schema: models::CraftingSchema,
    ) -> Result<ActionCraftingMyNameActionCraftingPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_crafting_my_name_action_crafting_post(name, crafting_schema, &context)
            .await
    }

    /// Action Delete Item
    async fn action_delete_item_my_name_action_delete_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
    ) -> Result<ActionDeleteItemMyNameActionDeletePostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_delete_item_my_name_action_delete_post(name, simple_item_schema, &context)
            .await
    }

    /// Action Deposit Bank Gold
    async fn action_deposit_bank_gold_my_name_action_bank_deposit_gold_post(
        &self,
        name: String,
        deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
    ) -> Result<ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_deposit_bank_gold_my_name_action_bank_deposit_gold_post(
                name,
                deposit_withdraw_gold_schema,
                &context,
            )
            .await
    }

    /// Action Deposit Bank
    async fn action_deposit_bank_my_name_action_bank_deposit_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
    ) -> Result<ActionDepositBankMyNameActionBankDepositPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_deposit_bank_my_name_action_bank_deposit_post(
                name,
                simple_item_schema,
                &context,
            )
            .await
    }

    /// Action Equip Item
    async fn action_equip_item_my_name_action_equip_post(
        &self,
        name: String,
        equip_schema: models::EquipSchema,
    ) -> Result<ActionEquipItemMyNameActionEquipPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_equip_item_my_name_action_equip_post(name, equip_schema, &context)
            .await
    }

    /// Action Fight
    async fn action_fight_my_name_action_fight_post(
        &self,
        name: String,
    ) -> Result<ActionFightMyNameActionFightPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_fight_my_name_action_fight_post(name, &context)
            .await
    }

    /// Action Gathering
    async fn action_gathering_my_name_action_gathering_post(
        &self,
        name: String,
    ) -> Result<ActionGatheringMyNameActionGatheringPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_gathering_my_name_action_gathering_post(name, &context)
            .await
    }

    /// Action Ge Buy Item
    async fn action_ge_buy_item_my_name_action_ge_buy_post(
        &self,
        name: String,
        ge_transaction_item_schema: models::GeTransactionItemSchema,
    ) -> Result<ActionGeBuyItemMyNameActionGeBuyPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_ge_buy_item_my_name_action_ge_buy_post(
                name,
                ge_transaction_item_schema,
                &context,
            )
            .await
    }

    /// Action Ge Sell Item
    async fn action_ge_sell_item_my_name_action_ge_sell_post(
        &self,
        name: String,
        ge_transaction_item_schema: models::GeTransactionItemSchema,
    ) -> Result<ActionGeSellItemMyNameActionGeSellPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_ge_sell_item_my_name_action_ge_sell_post(
                name,
                ge_transaction_item_schema,
                &context,
            )
            .await
    }

    /// Action Move
    async fn action_move_my_name_action_move_post(
        &self,
        name: String,
        destination_schema: models::DestinationSchema,
    ) -> Result<ActionMoveMyNameActionMovePostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_move_my_name_action_move_post(name, destination_schema, &context)
            .await
    }

    /// Action Recycling
    async fn action_recycling_my_name_action_recycling_post(
        &self,
        name: String,
        recycling_schema: models::RecyclingSchema,
    ) -> Result<ActionRecyclingMyNameActionRecyclingPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_recycling_my_name_action_recycling_post(name, recycling_schema, &context)
            .await
    }

    /// Action Task Exchange
    async fn action_task_exchange_my_name_action_task_exchange_post(
        &self,
        name: String,
    ) -> Result<ActionTaskExchangeMyNameActionTaskExchangePostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_task_exchange_my_name_action_task_exchange_post(name, &context)
            .await
    }

    /// Action Unequip Item
    async fn action_unequip_item_my_name_action_unequip_post(
        &self,
        name: String,
        unequip_schema: models::UnequipSchema,
    ) -> Result<ActionUnequipItemMyNameActionUnequipPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_unequip_item_my_name_action_unequip_post(name, unequip_schema, &context)
            .await
    }

    /// Action Withdraw Bank Gold
    async fn action_withdraw_bank_gold_my_name_action_bank_withdraw_gold_post(
        &self,
        name: String,
        deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
    ) -> Result<ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_withdraw_bank_gold_my_name_action_bank_withdraw_gold_post(
                name,
                deposit_withdraw_gold_schema,
                &context,
            )
            .await
    }

    /// Action Withdraw Bank
    async fn action_withdraw_bank_my_name_action_bank_withdraw_post(
        &self,
        name: String,
        simple_item_schema: models::SimpleItemSchema,
    ) -> Result<ActionWithdrawBankMyNameActionBankWithdrawPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .action_withdraw_bank_my_name_action_bank_withdraw_post(
                name,
                simple_item_schema,
                &context,
            )
            .await
    }

    /// Get All Characters Logs
    async fn get_all_characters_logs_my_logs_get(
        &self,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<GetAllCharactersLogsMyLogsGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_characters_logs_my_logs_get(page, size, &context)
            .await
    }

    /// Get My Characters
    async fn get_my_characters_my_characters_get(
        &self,
    ) -> Result<GetMyCharactersMyCharactersGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_my_characters_my_characters_get(&context)
            .await
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
    ) -> Result<GetAllResourcesResourcesGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_resources_resources_get(
                min_level, max_level, skill, drop, page, size, &context,
            )
            .await
    }

    /// Get Resource
    async fn get_resource_resources_code_get(
        &self,
        code: String,
    ) -> Result<GetResourceResourcesCodeGetResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_resource_resources_code_get(code, &context)
            .await
    }

    /// Generate Token
    async fn generate_token_token_post(&self) -> Result<GenerateTokenTokenPostResponse, ApiError> {
        let context = self.context().clone();
        self.api().generate_token_token_post(&context).await
    }
}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
