# LdapUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**all_emails** | Option<**Vec<String>**> | Array of user's email addresses and aliases for use in migration | [optional][readonly]
**attributes** | Option<**::std::collections::HashMap<String, String>**> | Dictionary of user's attributes (name/value) | [optional][readonly]
**email** | Option<**String**> | Primary email address | [optional][readonly]
**first_name** | Option<**String**> | First name | [optional][readonly]
**groups** | Option<**Vec<String>**> | Array of user's groups (group names only) | [optional][readonly]
**last_name** | Option<**String**> | Last Name | [optional][readonly]
**ldap_dn** | Option<**String**> | LDAP's distinguished name for the user record | [optional][readonly]
**ldap_id** | Option<**String**> | LDAP's Unique ID for the user | [optional][readonly]
**roles** | Option<**Vec<String>**> | Array of user's roles (role names only) | [optional][readonly]
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**url** | Option<**String**> | Link to ldap config | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


