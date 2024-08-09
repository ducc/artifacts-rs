# StatusSchema

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **String** | Server status | 
**version** | **String** |  | [optional] [default to None]
**characters_online** | **i32** |  | [optional] [default to None]
**server_time** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**announcements** | [**Vec<models::AnnouncementSchema>**](AnnouncementSchema.md) |  | [optional] [default to None]
**last_wipe** | **String** | Last server wipe. | 
**next_wipe** | **String** | Next server wipe. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


