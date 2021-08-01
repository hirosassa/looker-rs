# LookmlModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**allowed_db_connection_names** | Option<**Vec<String>**> | Array of names of connections this model is allowed to use | [optional]
**explores** | Option<[**Vec<crate::models::LookmlModelNavExplore>**](LookmlModelNavExplore.md)> | Array of explores (if has_content) | [optional][readonly]
**has_content** | Option<**bool**> | Does this model declaration have have lookml content? | [optional][readonly]
**label** | Option<**String**> | UI-friendly name for this model | [optional][readonly]
**name** | Option<**String**> | Name of the model. Also used as the unique identifier | [optional]
**project_name** | Option<**String**> | Name of project containing the model | [optional]
**unlimited_db_connections** | Option<**bool**> | Is this model allowed to use all current and future connections | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


