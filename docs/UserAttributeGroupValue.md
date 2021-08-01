# UserAttributeGroupValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**i64**> | Unique Id of this group-attribute relation | [optional][readonly]
**group_id** | Option<**i64**> | Id of group | [optional][readonly]
**user_attribute_id** | Option<**i64**> | Id of user attribute | [optional][readonly]
**value_is_hidden** | Option<**bool**> | If true, the \"value\" field will be null, because the attribute settings block access to this value | [optional][readonly]
**rank** | Option<**i64**> | Precedence for resolving value for user | [optional][readonly]
**value** | Option<**String**> | Value of user attribute for group | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


