# OidcUserAttributeWrite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of User Attribute in OIDC | [optional]
**required** | Option<**bool**> | Required to be in OIDC assertion for login to be allowed to succeed | [optional]
**user_attribute_ids** | Option<**Vec<i64>**> | Looker User Attribute Ids | [optional]
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


