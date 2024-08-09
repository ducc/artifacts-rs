# items_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**get_all_items_items__get**](items_api.md#get_all_items_items__get) | **GET** /items/ | Get All Items
**get_item_items__code__get**](items_api.md#get_item_items__code__get) | **GET** /items/{code} | Get Item


# **get_all_items_items__get**
> models::DataPageItemSchema get_all_items_items__get(optional)
Get All Items

Fetch items details.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **min_level** | **i32**| Minimum level items. | 
 **max_level** | **i32**| Maximum level items. | 
 **name** | **String**| Name of the item. | 
 **r#type** | **String**| Type of items. | 
 **craft_skill** | **String**| Skill to craft items. | 
 **craft_material** | **String**| Item code of items used as material for crafting. | 
 **page** | **i32**| Page number | [default to 1]
 **size** | **i32**| Page size | [default to 50]

### Return type

[**models::DataPageItemSchema**](DataPage_ItemSchema_.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_item_items__code__get**
> models::ItemResponseSchema get_item_items__code__get(code)
Get Item

Retrieve the details of a item.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **code** | **String**| The code of the item. | 

### Return type

[**models::ItemResponseSchema**](ItemResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

