# CreateQueryTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**query_id** | **i64** | Id of query to run | 
**result_format** | **String** | Desired async query result format. Valid values are: \"inline_json\", \"json\", \"json_detail\", \"json_fe\", \"csv\", \"html\", \"md\", \"txt\", \"xlsx\", \"gsxml\". | 
**source** | Option<**String**> | Source of query task | [optional]
**deferred** | Option<**bool**> | Create the task but defer execution | [optional]
**look_id** | Option<**i64**> | Id of look associated with query. | [optional]
**dashboard_id** | Option<**String**> | Id of dashboard associated with query. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


