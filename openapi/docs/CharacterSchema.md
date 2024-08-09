# CharacterSchema

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the character. | 
**skin** | **String** | Character skin code. | 
**level** | **i32** | Combat level. | 
**xp** | **i32** | The current xp level of the combat level. | 
**max_xp** | **i32** | XP required to level up the character. | 
**total_xp** | **i32** | Total XP of your character. | 
**gold** | **i32** | The numbers of golds on this character. | 
**speed** | **i32** | *Not available, on the roadmap. Character movement speed. | 
**mining_level** | **i32** | Mining level. | 
**mining_xp** | **i32** | The current xp level of the Mining skill. | 
**mining_max_xp** | **i32** | Mining XP required to level up the skill. | 
**woodcutting_level** | **i32** | Woodcutting level. | 
**woodcutting_xp** | **i32** | The current xp level of the Woodcutting skill. | 
**woodcutting_max_xp** | **i32** | Woodcutting XP required to level up the skill. | 
**fishing_level** | **i32** | Fishing level. | 
**fishing_xp** | **i32** | The current xp level of the Fishing skill. | 
**fishing_max_xp** | **i32** | Fishing XP required to level up the skill. | 
**weaponcrafting_level** | **i32** | Weaponcrafting level. | 
**weaponcrafting_xp** | **i32** | The current xp level of the Weaponcrafting skill. | 
**weaponcrafting_max_xp** | **i32** | Weaponcrafting XP required to level up the skill. | 
**gearcrafting_level** | **i32** | Gearcrafting level. | 
**gearcrafting_xp** | **i32** | The current xp level of the Gearcrafting skill. | 
**gearcrafting_max_xp** | **i32** | Gearcrafting XP required to level up the skill. | 
**jewelrycrafting_level** | **i32** | Jewelrycrafting level. | 
**jewelrycrafting_xp** | **i32** | The current xp level of the Jewelrycrafting skill. | 
**jewelrycrafting_max_xp** | **i32** | Jewelrycrafting XP required to level up the skill. | 
**cooking_level** | **i32** | The current xp level of the Cooking skill. | 
**cooking_xp** | **i32** | Cooking XP. | 
**cooking_max_xp** | **i32** | Cooking XP required to level up the skill. | 
**hp** | **i32** | Character HP. | 
**haste** | **i32** | *Character Haste. Increase speed attack (reduce fight cooldown) | 
**critical_strike** | **i32** | *Not available, on the roadmap. Character Critical   Strike. Critical strikes increase the attack's damage. | 
**stamina** | **i32** | *Not available, on the roadmap. Regenerates life at the start of each turn. | 
**attack_fire** | **i32** | Fire attack. | 
**attack_earth** | **i32** | Earth attack. | 
**attack_water** | **i32** | Water attack. | 
**attack_air** | **i32** | Air attack. | 
**dmg_fire** | **i32** | % Fire damage. | 
**dmg_earth** | **i32** | % Earth damage. | 
**dmg_water** | **i32** | % Water damage. | 
**dmg_air** | **i32** | % Air damage. | 
**res_fire** | **i32** | % Fire resistance. | 
**res_earth** | **i32** | % Earth resistance. | 
**res_water** | **i32** | % Water resistance. | 
**res_air** | **i32** | % Air resistance. | 
**x** | **i32** | Character x coordinate. | 
**y** | **i32** | Character y coordinate. | 
**cooldown** | **i32** | Cooldown in seconds. | 
**cooldown_expiration** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Datetime Cooldown expiration. | [optional] [default to None]
**weapon_slot** | **String** | Weapon slot. | 
**shield_slot** | **String** | Shield slot. | 
**helmet_slot** | **String** | Helmet slot. | 
**body_armor_slot** | **String** | Body armor slot. | 
**leg_armor_slot** | **String** | Leg armor slot. | 
**boots_slot** | **String** | Boots slot. | 
**ring1_slot** | **String** | Ring 1 slot. | 
**ring2_slot** | **String** | Ring 2 slot. | 
**amulet_slot** | **String** | Amulet slot. | 
**artifact1_slot** | **String** | Artifact 1 slot. | 
**artifact2_slot** | **String** | Artifact 2 slot. | 
**artifact3_slot** | **String** | Artifact 3 slot. | 
**consumable1_slot** | **String** | Consumable 1 slot. | 
**consumable1_slot_quantity** | **i32** | Consumable 1 quantity. | 
**consumable2_slot** | **String** | Consumable 2 slot. | 
**consumable2_slot_quantity** | **i32** | Consumable 2 quantity. | 
**task** | **String** | Task in progress. | 
**task_type** | **String** | Task type. | 
**task_progress** | **i32** | Task progression. | 
**task_total** | **i32** | Task total objective. | 
**inventory_max_items** | **i32** | Inventory max items. | 
**inventory** | [**Vec<models::InventorySlot>**](InventorySlot.md) | List of inventory slots. | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


