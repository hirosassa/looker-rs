# ResultMakerWithIdVisConfigAndDynamicFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | Unique Id. | [optional][readonly]
**dynamic_fields** | Option<**String**> | JSON string of dynamic field information. | [optional][readonly]
**filterables** | Option<[**Vec<crate::models::ResultMakerFilterables>**](ResultMakerFilterables.md)> | array of items that can be filtered and information about them. | [optional][readonly]
**sorts** | Option<**Vec<String>**> | Sorts of the constituent Look, Query, or Merge Query | [optional][readonly]
**merge_result_id** | Option<**String**> | ID of merge result if this is a merge_result. | [optional][readonly]
**total** | Option<**bool**> | Total of the constituent Look, Query, or Merge Query | [optional][readonly]
**query_id** | Option<**i64**> | ID of query if this is a query. | [optional][readonly]
**sql_query_id** | Option<**String**> | ID of SQL Query if this is a SQL Runner Query | [optional][readonly]
**query** | Option<[**crate::models::Query**](Query.md)> |  | [optional]
**vis_config** | Option<**::std::collections::HashMap<String, String>**> | Vis config of the constituent Query, or Merge Query. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


