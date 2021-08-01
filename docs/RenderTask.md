# RenderTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**created_at** | Option<**String**> | Date/Time render task was created | [optional][readonly]
**dashboard_filters** | Option<**String**> | Filter values to apply to the dashboard queries, in URL query format | [optional][readonly]
**dashboard_id** | Option<**i64**> | Id of dashboard to render | [optional][readonly]
**dashboard_style** | Option<**String**> | Dashboard layout style: single_column or tiled | [optional][readonly]
**finalized_at** | Option<**String**> | Date/Time render task was completed | [optional][readonly]
**height** | Option<**i64**> | Output height in pixels. Flowed layouts may ignore this value. | [optional][readonly]
**id** | Option<**String**> | Id of this render task | [optional][readonly]
**look_id** | Option<**i64**> | Id of look to render | [optional][readonly]
**lookml_dashboard_id** | Option<**String**> | Id of lookml dashboard to render | [optional][readonly]
**query_id** | Option<**i64**> | Id of query to render | [optional][readonly]
**query_runtime** | Option<**f64**> | Number of seconds elapsed running queries | [optional][readonly]
**render_runtime** | Option<**f64**> | Number of seconds elapsed rendering data | [optional][readonly]
**result_format** | Option<**String**> | Output format: pdf, png, or jpg | [optional][readonly]
**runtime** | Option<**f64**> | Total seconds elapsed for render task | [optional][readonly]
**status** | Option<**String**> | Render task status: enqueued_for_query, querying, enqueued_for_render, rendering, success, failure | [optional][readonly]
**status_detail** | Option<**String**> | Additional information about the current status | [optional][readonly]
**user_id** | Option<**i64**> | The user account permissions in which the render task will execute | [optional][readonly]
**width** | Option<**i64**> | Output width in pixels | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


