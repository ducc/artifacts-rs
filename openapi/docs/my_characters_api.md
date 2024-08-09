# my_characters_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**action_accept_new_task_my__name__action_task_new_post**](my_characters_api.md#action_accept_new_task_my__name__action_task_new_post) | **POST** /my/{name}/action/task/new | Action Accept New Task
**action_complete_task_my__name__action_task_complete_post**](my_characters_api.md#action_complete_task_my__name__action_task_complete_post) | **POST** /my/{name}/action/task/complete | Action Complete Task
**action_crafting_my__name__action_crafting_post**](my_characters_api.md#action_crafting_my__name__action_crafting_post) | **POST** /my/{name}/action/crafting | Action Crafting
**action_delete_item_my__name__action_delete_post**](my_characters_api.md#action_delete_item_my__name__action_delete_post) | **POST** /my/{name}/action/delete | Action Delete Item
**action_deposit_bank_gold_my__name__action_bank_deposit_gold_post**](my_characters_api.md#action_deposit_bank_gold_my__name__action_bank_deposit_gold_post) | **POST** /my/{name}/action/bank/deposit/gold | Action Deposit Bank Gold
**action_deposit_bank_my__name__action_bank_deposit_post**](my_characters_api.md#action_deposit_bank_my__name__action_bank_deposit_post) | **POST** /my/{name}/action/bank/deposit | Action Deposit Bank
**action_equip_item_my__name__action_equip_post**](my_characters_api.md#action_equip_item_my__name__action_equip_post) | **POST** /my/{name}/action/equip | Action Equip Item
**action_fight_my__name__action_fight_post**](my_characters_api.md#action_fight_my__name__action_fight_post) | **POST** /my/{name}/action/fight | Action Fight
**action_gathering_my__name__action_gathering_post**](my_characters_api.md#action_gathering_my__name__action_gathering_post) | **POST** /my/{name}/action/gathering | Action Gathering
**action_ge_buy_item_my__name__action_ge_buy_post**](my_characters_api.md#action_ge_buy_item_my__name__action_ge_buy_post) | **POST** /my/{name}/action/ge/buy | Action Ge Buy Item
**action_ge_sell_item_my__name__action_ge_sell_post**](my_characters_api.md#action_ge_sell_item_my__name__action_ge_sell_post) | **POST** /my/{name}/action/ge/sell | Action Ge Sell Item
**action_move_my__name__action_move_post**](my_characters_api.md#action_move_my__name__action_move_post) | **POST** /my/{name}/action/move | Action Move
**action_recycling_my__name__action_recycling_post**](my_characters_api.md#action_recycling_my__name__action_recycling_post) | **POST** /my/{name}/action/recycling | Action Recycling
**action_task_exchange_my__name__action_task_exchange_post**](my_characters_api.md#action_task_exchange_my__name__action_task_exchange_post) | **POST** /my/{name}/action/task/exchange | Action Task Exchange
**action_unequip_item_my__name__action_unequip_post**](my_characters_api.md#action_unequip_item_my__name__action_unequip_post) | **POST** /my/{name}/action/unequip | Action Unequip Item
**action_withdraw_bank_gold_my__name__action_bank_withdraw_gold_post**](my_characters_api.md#action_withdraw_bank_gold_my__name__action_bank_withdraw_gold_post) | **POST** /my/{name}/action/bank/withdraw/gold | Action Withdraw Bank Gold
**action_withdraw_bank_my__name__action_bank_withdraw_post**](my_characters_api.md#action_withdraw_bank_my__name__action_bank_withdraw_post) | **POST** /my/{name}/action/bank/withdraw | Action Withdraw Bank
**get_all_characters_logs_my_logs_get**](my_characters_api.md#get_all_characters_logs_my_logs_get) | **GET** /my/logs | Get All Characters Logs
**get_my_characters_my_characters_get**](my_characters_api.md#get_my_characters_my_characters_get) | **GET** /my/characters | Get My Characters


# **action_accept_new_task_my__name__action_task_new_post**
> models::TaskResponseSchema action_accept_new_task_my__name__action_task_new_post(ctx, name)
Action Accept New Task

Accepting a new task.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 

### Return type

[**models::TaskResponseSchema**](TaskResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_complete_task_my__name__action_task_complete_post**
> models::TaskRewardResponseSchema action_complete_task_my__name__action_task_complete_post(ctx, name)
Action Complete Task

Complete a task.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 

### Return type

[**models::TaskRewardResponseSchema**](TaskRewardResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_crafting_my__name__action_crafting_post**
> models::SkillResponseSchema action_crafting_my__name__action_crafting_post(ctx, name, crafting_schema)
Action Crafting

Crafting an item. The character must be on a map with a workshop.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **crafting_schema** | [**CraftingSchema**](CraftingSchema.md)|  | 

### Return type

[**models::SkillResponseSchema**](SkillResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_delete_item_my__name__action_delete_post**
> models::DeleteItemResponseSchema action_delete_item_my__name__action_delete_post(ctx, name, simple_item_schema)
Action Delete Item

Delete an item from your character's inventory.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **simple_item_schema** | [**SimpleItemSchema**](SimpleItemSchema.md)|  | 

### Return type

[**models::DeleteItemResponseSchema**](DeleteItemResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_deposit_bank_gold_my__name__action_bank_deposit_gold_post**
> models::GoldResponseSchema action_deposit_bank_gold_my__name__action_bank_deposit_gold_post(ctx, name, deposit_withdraw_gold_schema)
Action Deposit Bank Gold

Deposit golds in a bank on the character's map.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **deposit_withdraw_gold_schema** | [**DepositWithdrawGoldSchema**](DepositWithdrawGoldSchema.md)|  | 

### Return type

[**models::GoldResponseSchema**](GoldResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_deposit_bank_my__name__action_bank_deposit_post**
> models::ActionItemBankResponseSchema action_deposit_bank_my__name__action_bank_deposit_post(ctx, name, simple_item_schema)
Action Deposit Bank

Deposit an item in a bank on the character's map.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **simple_item_schema** | [**SimpleItemSchema**](SimpleItemSchema.md)|  | 

### Return type

[**models::ActionItemBankResponseSchema**](ActionItemBankResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_equip_item_my__name__action_equip_post**
> models::EquipmentResponseSchema action_equip_item_my__name__action_equip_post(ctx, name, equip_schema)
Action Equip Item

Equip an item on your character.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **equip_schema** | [**EquipSchema**](EquipSchema.md)|  | 

### Return type

[**models::EquipmentResponseSchema**](EquipmentResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_fight_my__name__action_fight_post**
> models::CharacterFightResponseSchema action_fight_my__name__action_fight_post(ctx, name)
Action Fight

Start a fight against a monster on the character's map.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 

### Return type

[**models::CharacterFightResponseSchema**](CharacterFightResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_gathering_my__name__action_gathering_post**
> models::SkillResponseSchema action_gathering_my__name__action_gathering_post(ctx, name)
Action Gathering

Harvest a resource on the character's map.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 

### Return type

[**models::SkillResponseSchema**](SkillResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_ge_buy_item_my__name__action_ge_buy_post**
> models::GeTransactionResponseSchema action_ge_buy_item_my__name__action_ge_buy_post(ctx, name, ge_transaction_item_schema)
Action Ge Buy Item

Buy an item at the Grand Exchange on the character's map.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **ge_transaction_item_schema** | [**GeTransactionItemSchema**](GeTransactionItemSchema.md)|  | 

### Return type

[**models::GeTransactionResponseSchema**](GETransactionResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_ge_sell_item_my__name__action_ge_sell_post**
> models::GeTransactionResponseSchema action_ge_sell_item_my__name__action_ge_sell_post(ctx, name, ge_transaction_item_schema)
Action Ge Sell Item

Sell an item at the Grand Exchange on the character's map.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **ge_transaction_item_schema** | [**GeTransactionItemSchema**](GeTransactionItemSchema.md)|  | 

### Return type

[**models::GeTransactionResponseSchema**](GETransactionResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_move_my__name__action_move_post**
> models::CharacterMovementResponseSchema action_move_my__name__action_move_post(ctx, name, destination_schema)
Action Move

Moves a character on the map using the map's X and Y position.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **destination_schema** | [**DestinationSchema**](DestinationSchema.md)|  | 

### Return type

[**models::CharacterMovementResponseSchema**](CharacterMovementResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_recycling_my__name__action_recycling_post**
> models::RecyclingResponseSchema action_recycling_my__name__action_recycling_post(ctx, name, recycling_schema)
Action Recycling

Recyling an item. The character must be on a map with a workshop (only for equipments and weapons).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **recycling_schema** | [**RecyclingSchema**](RecyclingSchema.md)|  | 

### Return type

[**models::RecyclingResponseSchema**](RecyclingResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_task_exchange_my__name__action_task_exchange_post**
> models::TaskRewardResponseSchema action_task_exchange_my__name__action_task_exchange_post(ctx, name)
Action Task Exchange

Exchange 3 tasks coins for a random reward. Rewards are exclusive resources for crafting  items.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 

### Return type

[**models::TaskRewardResponseSchema**](TaskRewardResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_unequip_item_my__name__action_unequip_post**
> models::EquipmentResponseSchema action_unequip_item_my__name__action_unequip_post(ctx, name, unequip_schema)
Action Unequip Item

Unequip an item on your character.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **unequip_schema** | [**UnequipSchema**](UnequipSchema.md)|  | 

### Return type

[**models::EquipmentResponseSchema**](EquipmentResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_withdraw_bank_gold_my__name__action_bank_withdraw_gold_post**
> models::GoldResponseSchema action_withdraw_bank_gold_my__name__action_bank_withdraw_gold_post(ctx, name, deposit_withdraw_gold_schema)
Action Withdraw Bank Gold

Withdraw gold from your bank.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **deposit_withdraw_gold_schema** | [**DepositWithdrawGoldSchema**](DepositWithdrawGoldSchema.md)|  | 

### Return type

[**models::GoldResponseSchema**](GoldResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **action_withdraw_bank_my__name__action_bank_withdraw_post**
> models::ActionItemBankResponseSchema action_withdraw_bank_my__name__action_bank_withdraw_post(ctx, name, simple_item_schema)
Action Withdraw Bank

Take an item from your bank and put it in the character's inventory.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Name of your character. | 
  **simple_item_schema** | [**SimpleItemSchema**](SimpleItemSchema.md)|  | 

### Return type

[**models::ActionItemBankResponseSchema**](ActionItemBankResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_all_characters_logs_my_logs_get**
> models::DataPageLogSchema get_all_characters_logs_my_logs_get(ctx, optional)
Get All Characters Logs

History of the last 100 actions of all your characters.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | **i32**| Page number | [default to 1]
 **size** | **i32**| Page size | [default to 50]

### Return type

[**models::DataPageLogSchema**](DataPage_LogSchema_.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_my_characters_my_characters_get**
> models::MyCharactersListSchema get_my_characters_my_characters_get(ctx, )
Get My Characters

List of your characters.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::MyCharactersListSchema**](MyCharactersListSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

