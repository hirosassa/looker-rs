# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**can_add_to_content_metadata** | Option<**bool**> | Group can be used in content access controls | [optional]
**contains_current_user** | Option<**bool**> | Currently logged in user is group member | [optional][readonly]
**external_group_id** | Option<**String**> | External Id group if embed group | [optional][readonly]
**externally_managed** | Option<**bool**> | Group membership controlled outside of Looker | [optional][readonly]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**include_by_default** | Option<**bool**> | New users are added to this group by default | [optional][readonly]
**name** | Option<**String**> | Name of group | [optional]
**user_count** | Option<**i64**> | Number of users included in this group | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


