# Session

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**ip_address** | Option<**String**> | IP address of user when this session was initiated | [optional][readonly]
**browser** | Option<**String**> | User's browser type | [optional][readonly]
**operating_system** | Option<**String**> | User's Operating System | [optional][readonly]
**city** | Option<**String**> | City component of user location (derived from IP address) | [optional][readonly]
**state** | Option<**String**> | State component of user location (derived from IP address) | [optional][readonly]
**country** | Option<**String**> | Country component of user location (derived from IP address) | [optional][readonly]
**credentials_type** | Option<**String**> | Type of credentials used for logging in this session | [optional][readonly]
**extended_at** | Option<**String**> | Time when this session was last extended by the user | [optional][readonly]
**extended_count** | Option<**i64**> | Number of times this session was extended | [optional][readonly]
**sudo_user_id** | Option<**i64**> | Actual user in the case when this session represents one user sudo'ing as another | [optional][readonly]
**created_at** | Option<**String**> | Time when this session was initiated | [optional][readonly]
**expires_at** | Option<**String**> | Time when this session will expire | [optional][readonly]
**url** | Option<**String**> | Link to get this item | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


