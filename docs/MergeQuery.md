# MergeQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**column_limit** | Option<**String**> | Column Limit | [optional]
**dynamic_fields** | Option<**String**> | Dynamic Fields | [optional]
**id** | Option<**String**> | Unique Id | [optional][readonly]
**pivots** | Option<**Vec<String>**> | Pivots | [optional]
**result_maker_id** | Option<**i64**> | Unique to get results | [optional][readonly]
**sorts** | Option<**Vec<String>**> | Sorts | [optional]
**source_queries** | Option<[**Vec<crate::models::MergeQuerySourceQuery>**](MergeQuerySourceQuery.md)> | Source Queries defining the results to be merged. | [optional]
**total** | Option<**bool**> | Total | [optional]
**vis_config** | Option<**::std::collections::HashMap<String, String>**> | Visualization Config | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


