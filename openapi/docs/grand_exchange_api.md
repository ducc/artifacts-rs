# grand_exchange_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**get_all_ge_items_ge__get**](grand_exchange_api.md#get_all_ge_items_ge__get) | **GET** /ge/ | Get All Ge Items
**get_ge_item_ge__code__get**](grand_exchange_api.md#get_ge_item_ge__code__get) | **GET** /ge/{code} | Get Ge Item


# **get_all_ge_items_ge__get**
> models::DataPageGeItemSchema get_all_ge_items_ge__get(optional)
Get All Ge Items

Fetch Grand Exchange items details.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | **i32**| Page number | [default to 1]
 **size** | **i32**| Page size | [default to 50]

### Return type

[**models::DataPageGeItemSchema**](DataPage_GEItemSchema_.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ge_item_ge__code__get**
> models::GeItemResponseSchema get_ge_item_ge__code__get(code)
Get Ge Item

Retrieve the details of a Grand Exchange item.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **code** | **String**| The code of the item. | 

### Return type

[**models::GeItemResponseSchema**](GEItemResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

