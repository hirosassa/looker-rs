# Role

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**name** | Option<**String**> | Name of Role | [optional]
**permission_set** | Option<[**crate::models::PermissionSet**](PermissionSet.md)> |  | [optional]
**permission_set_id** | Option<**i64**> | (Write-Only) Id of permission set | [optional]
**model_set** | Option<[**crate::models::ModelSet**](ModelSet.md)> |  | [optional]
**model_set_id** | Option<**i64**> | (Write-Only) Id of model set | [optional]
**url** | Option<**String**> | Link to get this item | [optional][readonly]
**users_url** | Option<**String**> | Link to get list of users with this role | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


