# Query

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**model** | **String** | Model | 
**view** | **String** | Explore Name | 
**fields** | Option<**Vec<String>**> | Fields | [optional]
**pivots** | Option<**Vec<String>**> | Pivots | [optional]
**fill_fields** | Option<**Vec<String>**> | Fill Fields | [optional]
**filters** | Option<**::std::collections::HashMap<String, String>**> | Filters | [optional]
**filter_expression** | Option<**String**> | Filter Expression | [optional]
**sorts** | Option<**Vec<String>**> | Sorting for the query results. Use the format `[\"view.field\", ...]` to sort on fields in ascending order. Use the format `[\"view.field desc\", ...]` to sort on fields in descending order. Use `[\"__UNSORTED__\"]` (2 underscores before and after) to disable sorting entirely. Empty sorts `[]` will trigger a default sort. | [optional]
**limit** | Option<**String**> | Limit | [optional]
**column_limit** | Option<**String**> | Column Limit | [optional]
**total** | Option<**bool**> | Total | [optional]
**row_total** | Option<**String**> | Raw Total | [optional]
**subtotals** | Option<**Vec<String>**> | Fields on which to run subtotals | [optional]
**vis_config** | Option<**::std::collections::HashMap<String, String>**> | Visualization configuration properties. These properties are typically opaque and differ based on the type of visualization used. There is no specified set of allowed keys. The values can be any type supported by JSON. A \"type\" key with a string value is often present, and is used by Looker to determine which visualization to present. Visualizations ignore unknown vis_config properties. | [optional]
**filter_config** | Option<**::std::collections::HashMap<String, String>**> | The filter_config represents the state of the filter UI on the explore page for a given query. When running a query via the Looker UI, this parameter takes precedence over \"filters\". When creating a query or modifying an existing query, \"filter_config\" should be set to null. Setting it to any other value could cause unexpected filtering behavior. The format should be considered opaque. | [optional]
**visible_ui_sections** | Option<**String**> | Visible UI Sections | [optional]
**slug** | Option<**String**> | Slug | [optional][readonly]
**dynamic_fields** | Option<**String**> | Dynamic Fields | [optional]
**client_id** | Option<**String**> | Client Id: used to generate shortened explore URLs. If set by client, must be a unique 22 character alphanumeric string. Otherwise one will be generated. | [optional]
**share_url** | Option<**String**> | Share Url | [optional][readonly]
**expanded_share_url** | Option<**String**> | Expanded Share Url | [optional][readonly]
**url** | Option<**String**> | Expanded Url | [optional][readonly]
**query_timezone** | Option<**String**> | Query Timezone | [optional]
**has_table_calculations** | Option<**bool**> | Has Table Calculations | [optional][readonly]
**runtime** | Option<**f64**> | (DEPRECATED) Runtime (Deprecated) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


