# SqlQueryCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connection_name** | Option<**String**> | Name of the db connection on which to run this query | [optional]
**connection_id** | Option<**String**> | (DEPRECATED) Use `connection_name` instead | [optional]
**model_name** | Option<**String**> | Name of LookML Model (this or `connection_id` required) | [optional]
**sql** | Option<**String**> | SQL query | [optional]
**vis_config** | Option<**::std::collections::HashMap<String, String>**> | Visualization configuration properties. These properties are typically opaque and differ based on the type of visualization used. There is no specified set of allowed keys. The values can be any type supported by JSON. A \"type\" key with a string value is often present, and is used by Looker to determine which visualization to present. Visualizations ignore unknown vis_config properties. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


