# ProjectValidationCache

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<crate::models::ProjectError>**](ProjectError.md)> | A list of project errors | [optional][readonly]
**project_digest** | Option<**String**> | A hash value computed from the project's current state | [optional][readonly]
**models_not_validated** | Option<[**Vec<crate::models::ModelsNotValidated>**](ModelsNotValidated.md)> | A list of models which were not fully validated | [optional][readonly]
**computation_time** | Option<**f32**> | Duration of project validation in seconds | [optional][readonly]
**stale** | Option<**bool**> | If true, the cached project validation results are no longer accurate because the project has changed since the cached results were calculated | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


