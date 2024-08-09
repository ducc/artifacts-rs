# monsters_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**get_all_monsters_monsters__get**](monsters_api.md#get_all_monsters_monsters__get) | **GET** /monsters/ | Get All Monsters
**get_monster_monsters__code__get**](monsters_api.md#get_monster_monsters__code__get) | **GET** /monsters/{code} | Get Monster


# **get_all_monsters_monsters__get**
> models::DataPageMonsterSchema get_all_monsters_monsters__get(optional)
Get All Monsters

Fetch monsters details.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **min_level** | **i32**| Monster minimum level. | 
 **max_level** | **i32**| Monster maximum level. | 
 **drop** | **String**| Item code of the drop. | 
 **page** | **i32**| Page number | [default to 1]
 **size** | **i32**| Page size | [default to 50]

### Return type

[**models::DataPageMonsterSchema**](DataPage_MonsterSchema_.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_monster_monsters__code__get**
> models::MonsterResponseSchema get_monster_monsters__code__get(code)
Get Monster

Retrieve the details of a monster.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **code** | **String**| The code of the monster. | 

### Return type

[**models::MonsterResponseSchema**](MonsterResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

