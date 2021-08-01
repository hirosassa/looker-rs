# RunningQueries

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**user** | Option<[**crate::models::UserPublic**](UserPublic.md)> |  | [optional]
**query** | Option<[**crate::models::Query**](Query.md)> |  | [optional]
**sql_query** | Option<[**crate::models::SqlQuery**](SqlQuery.md)> |  | [optional]
**look** | Option<[**crate::models::LookBasic**](LookBasic.md)> |  | [optional]
**created_at** | Option<**String**> | Date/Time Query was initiated | [optional][readonly]
**completed_at** | Option<**String**> | Date/Time Query was completed | [optional][readonly]
**query_id** | Option<**String**> | Query Id | [optional][readonly]
**source** | Option<**String**> | Source (look, dashboard, queryrunner, explore, etc.) | [optional][readonly]
**node_id** | Option<**String**> | Node Id | [optional][readonly]
**slug** | Option<**String**> | Slug | [optional][readonly]
**query_task_id** | Option<**String**> | ID of a Query Task | [optional][readonly]
**cache_key** | Option<**String**> | Cache Key | [optional][readonly]
**connection_name** | Option<**String**> | Connection | [optional][readonly]
**dialect** | Option<**String**> | Dialect | [optional][readonly]
**connection_id** | Option<**String**> | Connection ID | [optional][readonly]
**message** | Option<**String**> | Additional Information(Error message or verbose status) | [optional][readonly]
**status** | Option<**String**> | Status description | [optional][readonly]
**runtime** | Option<**f64**> | Number of seconds elapsed running the Query | [optional][readonly]
**sql** | Option<**String**> | SQL text of the query as run | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


