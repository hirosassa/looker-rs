# CredentialsLdap

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**created_at** | Option<**String**> | Timestamp for the creation of this credential | [optional][readonly]
**email** | Option<**String**> | EMail address | [optional][readonly]
**is_disabled** | Option<**bool**> | Has this credential been disabled? | [optional][readonly]
**ldap_dn** | Option<**String**> | LDAP Distinguished name for this user (as-of the last login) | [optional][readonly]
**ldap_id** | Option<**String**> | LDAP Unique ID for this user | [optional][readonly]
**logged_in_at** | Option<**String**> | Timestamp for most recent login using credential | [optional][readonly]
**_type** | Option<**String**> | Short name for the type of this kind of credential | [optional][readonly]
**url** | Option<**String**> | Link to get this item | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


