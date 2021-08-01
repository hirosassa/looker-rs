# LegacyFeature

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**String**> | Unique Id | [optional][readonly]
**name** | Option<**String**> | Name | [optional][readonly]
**description** | Option<**String**> | Description | [optional][readonly]
**enabled_locally** | Option<**bool**> | Whether this feature has been enabled by a user | [optional]
**enabled** | Option<**bool**> | Whether this feature is currently enabled | [optional][readonly]
**disallowed_as_of_version** | Option<**String**> | Looker version where this feature became a legacy feature | [optional][readonly]
**disable_on_upgrade_to_version** | Option<**String**> | Looker version where this feature will be automatically disabled | [optional][readonly]
**end_of_life_version** | Option<**String**> | Future Looker version where this feature will be removed | [optional][readonly]
**documentation_url** | Option<**String**> | URL for documentation about this feature | [optional][readonly]
**approximate_disable_date** | Option<**String**> | Approximate date that this feature will be automatically disabled. | [optional][readonly]
**approximate_end_of_life_date** | Option<**String**> | Approximate date that this feature will be removed. | [optional][readonly]
**has_disabled_on_upgrade** | Option<**bool**> | Whether this legacy feature may have been automatically disabled when upgrading to the current version. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


