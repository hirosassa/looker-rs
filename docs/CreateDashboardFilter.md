# CreateDashboardFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique Id | [optional][readonly]
**dashboard_id** | **String** | Id of Dashboard | 
**name** | **String** | Name of filter | 
**title** | **String** | Title of filter | 
**_type** | **String** | Type of filter: one of date, number, string, or field | 
**default_value** | Option<**String**> | Default value of filter | [optional]
**model** | Option<**String**> | Model of filter (required if type = field) | [optional]
**explore** | Option<**String**> | Explore of filter (required if type = field) | [optional]
**dimension** | Option<**String**> | Dimension of filter (required if type = field) | [optional]
**field** | Option<**::std::collections::HashMap<String, String>**> | Field information | [optional][readonly]
**row** | Option<**i64**> | Display order of this filter relative to other filters | [optional]
**listens_to_filters** | Option<**Vec<String>**> | Array of listeners for faceted filters | [optional]
**allow_multiple_values** | Option<**bool**> | Whether the filter allows multiple filter values (deprecated in the latest version of dashboards) | [optional]
**required** | Option<**bool**> | Whether the filter requires a value to run the dashboard | [optional]
**ui_config** | Option<**::std::collections::HashMap<String, String>**> | The visual configuration for this filter. Used to set up how the UI for this filter should appear. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


