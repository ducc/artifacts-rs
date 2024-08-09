# maps_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**get_all_maps_maps__get**](maps_api.md#get_all_maps_maps__get) | **GET** /maps/ | Get All Maps
**get_map_maps__x___y__get**](maps_api.md#get_map_maps__x___y__get) | **GET** /maps/{x}/{y} | Get Map


# **get_all_maps_maps__get**
> models::DataPageMapSchema get_all_maps_maps__get(optional)
Get All Maps

Fetch maps details.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **content_type** | **String**| Type of content on the map. | 
 **content_code** | **String**| Content code on the map. | 
 **page** | **i32**| Page number | [default to 1]
 **size** | **i32**| Page size | [default to 50]

### Return type

[**models::DataPageMapSchema**](DataPage_MapSchema_.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_map_maps__x___y__get**
> models::MapResponseSchema get_map_maps__x___y__get(x, y)
Get Map

Retrieve the details of a map.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x** | **i32**| The position x of the map. | 
  **y** | **i32**| The position X of the map. | 

### Return type

[**models::MapResponseSchema**](MapResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

