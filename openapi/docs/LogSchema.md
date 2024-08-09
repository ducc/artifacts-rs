# LogSchema

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**character** | **String** | Character name. | 
**account** | **String** | Account character. | 
**r#type** | **String** | Type of action. | 
**description** | **String** | Description of action. | 
**content** | [***serde_json::Value**](.md) |  | 
**cooldown** | **i32** | Cooldown in seconds. | 
**cooldown_expiration** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Datetime of cooldown expiration. | 
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Datetime of creation. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


