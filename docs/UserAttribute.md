# UserAttribute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**name** | Option<**String**> | Name of user attribute | [optional]
**label** | Option<**String**> | Human-friendly label for user attribute | [optional]
**_type** | Option<**String**> | Type of user attribute (\"string\", \"number\", \"datetime\", \"yesno\", \"zipcode\") | [optional]
**default_value** | Option<**String**> | Default value for when no value is set on the user | [optional]
**is_system** | Option<**bool**> | Attribute is a system default | [optional][readonly]
**is_permanent** | Option<**bool**> | Attribute is permanent and cannot be deleted | [optional][readonly]
**value_is_hidden** | Option<**bool**> | If true, users will not be able to view values of this attribute | [optional]
**user_can_view** | Option<**bool**> | Non-admin users can see the values of their attributes and use them in filters | [optional]
**user_can_edit** | Option<**bool**> | Users can change the value of this attribute for themselves | [optional]
**hidden_value_domain_whitelist** | Option<**String**> | Destinations to which a hidden attribute may be sent. Once set, cannot be edited. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


