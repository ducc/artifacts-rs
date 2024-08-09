# ItemSchema

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Item name. | 
**code** | **String** | Item code. This is the item's unique identifier (ID). | 
**level** | **u32** | Item level. | 
**r#type** | **String** | Item type. | 
**subtype** | **String** | Item subtype. | 
**description** | **String** | Item description. | 
**effects** | [**Vec<models::ItemEffectSchema>**](ItemEffectSchema.md) | List of object effects. For equipment, it will include item stats. | [optional] [default to None]
**craft** | [***models::CraftSchema**](CraftSchema.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


