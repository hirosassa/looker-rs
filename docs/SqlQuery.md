# SqlQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**slug** | Option<**String**> | The identifier of the SQL query | [optional][readonly]
**last_runtime** | Option<**f32**> | Number of seconds this query took to run the most recent time it was run | [optional][readonly]
**run_count** | Option<**i64**> | Number of times this query has been run | [optional][readonly]
**browser_limit** | Option<**i64**> | Maximum number of rows this query will display on the SQL Runner page | [optional][readonly]
**sql** | Option<**String**> | SQL query text | [optional][readonly]
**last_run_at** | Option<**String**> | The most recent time this query was run | [optional][readonly]
**connection** | Option<[**crate::models::DbConnectionBase**](DBConnectionBase.md)> |  | [optional]
**model_name** | Option<**String**> | Model name this query uses | [optional][readonly]
**creator** | Option<[**crate::models::UserPublic**](UserPublic.md)> |  | [optional]
**explore_url** | Option<**String**> | Explore page URL for this SQL query | [optional][readonly]
**plaintext** | Option<**bool**> | Should this query be rendered as plain text | [optional][readonly]
**vis_config** | Option<**::std::collections::HashMap<String, String>**> | Visualization configuration properties. These properties are typically opaque and differ based on the type of visualization used. There is no specified set of allowed keys. The values can be any type supported by JSON. A \"type\" key with a string value is often present, and is used by Looker to determine which visualization to present. Visualizations ignore unknown vis_config properties. | [optional]
**result_maker_id** | Option<**i64**> | ID of the ResultMakerLookup entry. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


