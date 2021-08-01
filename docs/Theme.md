# Theme

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**begin_at** | Option<**String**> | Timestamp for when this theme becomes active. Null=always | [optional]
**end_at** | Option<**String**> | Timestamp for when this theme expires. Null=never | [optional]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**name** | Option<**String**> | Name of theme. Can only be alphanumeric and underscores. | [optional]
**settings** | Option<[**crate::models::ThemeSettings**](ThemeSettings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


