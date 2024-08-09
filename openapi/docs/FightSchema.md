# FightSchema

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**xp** | **i32** | The amount of xp gained by the fight. | 
**gold** | **i32** | The amount of gold gained by the fight. | 
**drops** | [**Vec<models::DropSchema>**](DropSchema.md) | The items dropped by the fight. | 
**turns** | **i32** | Numbers of the turns of the combat. | 
**monster_blocked_hits** | [***models::BlockedHitsSchema**](BlockedHitsSchema.md) | The amount of blocked hits by the monster. | 
**player_blocked_hits** | [***models::BlockedHitsSchema**](BlockedHitsSchema.md) | The amount of blocked hits by the player. | 
**logs** | **Vec<String>** | The fight logs. | 
**result** | **String** | The result of the fight. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


