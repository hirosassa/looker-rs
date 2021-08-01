# SessionConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**allow_persistent_sessions** | Option<**bool**> | Allow users to have persistent sessions when they login | [optional]
**session_minutes** | Option<**i64**> | Number of minutes for user sessions.  Must be between 5 and 43200 | [optional]
**unlimited_sessions_per_user** | Option<**bool**> | Allow users to have an unbounded number of concurrent sessions (otherwise, users will be limited to only one session at a time). | [optional]
**use_inactivity_based_logout** | Option<**bool**> | Enforce session logout for sessions that are inactive for 15 minutes. | [optional]
**track_session_location** | Option<**bool**> | Track location of session when user logs in. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


