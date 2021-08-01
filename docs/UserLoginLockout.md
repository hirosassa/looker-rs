# UserLoginLockout

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**key** | Option<**String**> | Hash of user's client id | [optional][readonly]
**auth_type** | Option<**String**> | Authentication method for login failures | [optional][readonly]
**ip** | Option<**String**> | IP address of most recent failed attempt | [optional][readonly]
**user_id** | Option<**i64**> | User ID | [optional][readonly]
**remote_id** | Option<**String**> | Remote ID of user if using LDAP | [optional][readonly]
**full_name** | Option<**String**> | User's name | [optional][readonly]
**email** | Option<**String**> | Email address associated with the user's account | [optional][readonly]
**fail_count** | Option<**i64**> | Number of failures that triggered the lockout | [optional][readonly]
**lockout_at** | Option<**String**> | Time when lockout was triggered | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


