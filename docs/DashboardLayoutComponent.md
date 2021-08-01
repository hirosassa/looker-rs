# DashboardLayoutComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**String**> | Unique Id | [optional][readonly]
**dashboard_layout_id** | Option<**String**> | Id of Dashboard Layout | [optional]
**dashboard_element_id** | Option<**String**> | Id Of Dashboard Element | [optional]
**row** | Option<**i64**> | Row | [optional]
**column** | Option<**i64**> | Column | [optional]
**width** | Option<**i64**> | Width | [optional]
**height** | Option<**i64**> | Height | [optional]
**deleted** | Option<**bool**> | Whether or not the dashboard layout component is deleted | [optional][readonly]
**element_title** | Option<**String**> | Dashboard element title, extracted from the Dashboard Element. | [optional][readonly]
**element_title_hidden** | Option<**bool**> | Whether or not the dashboard element title is displayed. | [optional][readonly]
**vis_type** | Option<**String**> | Visualization type, extracted from a query's vis_config | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


