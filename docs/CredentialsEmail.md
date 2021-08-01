# CredentialsEmail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**created_at** | Option<**String**> | Timestamp for the creation of this credential | [optional][readonly]
**email** | Option<**String**> | EMail address used for user login | [optional]
**forced_password_reset_at_next_login** | Option<**bool**> | Force the user to change their password upon their next login | [optional]
**is_disabled** | Option<**bool**> | Has this credential been disabled? | [optional][readonly]
**logged_in_at** | Option<**String**> | Timestamp for most recent login using credential | [optional][readonly]
**password_reset_url** | Option<**String**> | Url with one-time use secret token that the user can use to reset password | [optional][readonly]
**_type** | Option<**String**> | Short name for the type of this kind of credential | [optional][readonly]
**url** | Option<**String**> | Link to get this item | [optional][readonly]
**user_url** | Option<**String**> | Link to get this user | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


