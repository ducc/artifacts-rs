# resources_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**get_all_resources_resources__get**](resources_api.md#get_all_resources_resources__get) | **GET** /resources/ | Get All Resources
**get_resource_resources__code__get**](resources_api.md#get_resource_resources__code__get) | **GET** /resources/{code} | Get Resource


# **get_all_resources_resources__get**
> models::DataPageResourceSchema get_all_resources_resources__get(optional)
Get All Resources

Fetch resources details.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **min_level** | **i32**| Skill minimum level. | 
 **max_level** | **i32**| Skill maximum level. | 
 **skill** | **String**| The code of the skill. | 
 **drop** | **String**| Item code of the drop. | 
 **page** | **i32**| Page number | [default to 1]
 **size** | **i32**| Page size | [default to 50]

### Return type

[**models::DataPageResourceSchema**](DataPage_ResourceSchema_.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_resource_resources__code__get**
> models::ResourceResponseSchema get_resource_resources__code__get(code)
Get Resource

Retrieve the details of a resource.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **code** | **String**| The code of the resource. | 

### Return type

[**models::ResourceResponseSchema**](ResourceResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

