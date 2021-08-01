# UserAttributeWithValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**name** | Option<**String**> | Name of user attribute | [optional][readonly]
**label** | Option<**String**> | Human-friendly label for user attribute | [optional][readonly]
**rank** | Option<**i64**> | Precedence for setting value on user (lowest wins) | [optional][readonly]
**value** | Option<**String**> | Value of attribute for user | [optional]
**user_id** | Option<**i64**> | Id of User | [optional][readonly]
**user_can_edit** | Option<**bool**> | Can the user set this value | [optional][readonly]
**value_is_hidden** | Option<**bool**> | If true, the \"value\" field will be null, because the attribute settings block access to this value | [optional][readonly]
**user_attribute_id** | Option<**i64**> | Id of User Attribute | [optional][readonly]
**source** | Option<**String**> | How user got this value for this attribute | [optional][readonly]
**hidden_value_domain_whitelist** | Option<**String**> | If this user attribute is hidden, whitelist of destinations to which it may be sent. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


