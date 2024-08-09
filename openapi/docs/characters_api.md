# characters_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**create_character_characters_create_post**](characters_api.md#create_character_characters_create_post) | **POST** /characters/create | Create Character
**delete_character_characters_delete_post**](characters_api.md#delete_character_characters_delete_post) | **POST** /characters/delete | Delete Character
**get_all_characters_characters__get**](characters_api.md#get_all_characters_characters__get) | **GET** /characters/ | Get All Characters
**get_character_characters__name__get**](characters_api.md#get_character_characters__name__get) | **GET** /characters/{name} | Get Character


# **create_character_characters_create_post**
> models::CharacterResponseSchema create_character_characters_create_post(ctx, add_character_schema)
Create Character

Create new character on your account. You can create up to 5 characters.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **add_character_schema** | [**AddCharacterSchema**](AddCharacterSchema.md)|  | 

### Return type

[**models::CharacterResponseSchema**](CharacterResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_character_characters_delete_post**
> models::CharacterResponseSchema delete_character_characters_delete_post(ctx, delete_character_schema)
Delete Character

Delete character on your account.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **delete_character_schema** | [**DeleteCharacterSchema**](DeleteCharacterSchema.md)|  | 

### Return type

[**models::CharacterResponseSchema**](CharacterResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_all_characters_characters__get**
> models::DataPageCharacterSchema get_all_characters_characters__get(optional)
Get All Characters

Fetch characters details.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| Default sort by combat total XP. | 
 **page** | **i32**| Page number | [default to 1]
 **size** | **i32**| Page size | [default to 50]

### Return type

[**models::DataPageCharacterSchema**](DataPage_CharacterSchema_.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_character_characters__name__get**
> models::CharacterResponseSchema get_character_characters__name__get(name)
Get Character

Retrieve the details of a character.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The character name. | 

### Return type

[**models::CharacterResponseSchema**](CharacterResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

