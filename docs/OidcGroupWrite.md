# OidcGroupWrite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | Unique Id | [optional]
**looker_group_id** | Option<**i64**> | Unique Id of group in Looker | [optional][readonly]
**looker_group_name** | Option<**String**> | Name of group in Looker | [optional]
**name** | Option<**String**> | Name of group in OIDC | [optional]
**role_ids** | Option<**Vec<i64>**> | Looker Role Ids | [optional]
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


