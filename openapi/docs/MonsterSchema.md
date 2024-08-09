# MonsterSchema

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the monster. | 
**code** | **String** | The code of the monster. This is the monster's unique identifier (ID). | 
**level** | **i32** | Monster level. | 
**hp** | **i32** | Monster hit points. | 
**attack_fire** | **i32** | Monster fire attack. | 
**attack_earth** | **i32** | Monster earth attack. | 
**attack_water** | **i32** | Monster water attack. | 
**attack_air** | **i32** | Monster air attack. | 
**res_fire** | **i32** | Monster % fire resistance. | 
**res_earth** | **i32** | Monster % earth resistance. | 
**res_water** | **i32** | Monster % water resistance. | 
**res_air** | **i32** | Monster % air resistance. | 
**min_gold** | **i32** | Monster minimum gold drop.  | 
**max_gold** | **i32** | Monster maximum gold drop.  | 
**drops** | [**Vec<models::DropRateSchema>**](DropRateSchema.md) | Monster drops. This is a list of items that the monster drops after killing the monster.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


