# my_account_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**change_password_my_change_password_post**](my_account_api.md#change_password_my_change_password_post) | **POST** /my/change_password | Change Password
**get_bank_golds_my_bank_gold_get**](my_account_api.md#get_bank_golds_my_bank_gold_get) | **GET** /my/bank/gold | Get Bank Golds
**get_bank_items_my_bank_items_get**](my_account_api.md#get_bank_items_my_bank_items_get) | **GET** /my/bank/items | Get Bank Items


# **change_password_my_change_password_post**
> models::ResponseSchema change_password_my_change_password_post(ctx, change_password)
Change Password

Change your account password. Changing the password reset the account token.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **change_password** | [**ChangePassword**](ChangePassword.md)|  | 

### Return type

[**models::ResponseSchema**](ResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_bank_golds_my_bank_gold_get**
> models::GoldBankResponseSchema get_bank_golds_my_bank_gold_get(ctx, )
Get Bank Golds

Fetch golds in your bank.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::GoldBankResponseSchema**](GoldBankResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_bank_items_my_bank_items_get**
> models::DataPageSimpleItemSchema get_bank_items_my_bank_items_get(ctx, optional)
Get Bank Items

Fetch all items in your bank.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **item_code** | **String**| Item to search in your bank. | 
 **page** | **i32**| Page number | [default to 1]
 **size** | **i32**| Page size | [default to 50]

### Return type

[**models::DataPageSimpleItemSchema**](DataPage_SimpleItemSchema_.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

