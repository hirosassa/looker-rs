# ProjectFile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**String**> | An opaque token uniquely identifying a file within a project. Avoid parsing or decomposing the text of this token. This token is stable within a Looker release but may change between Looker releases | [optional][readonly]
**path** | Option<**String**> | Path, file name, and extension of the file relative to the project root directory | [optional][readonly]
**title** | Option<**String**> | Display name | [optional][readonly]
**_type** | Option<**String**> | File type: model, view, etc | [optional][readonly]
**extension** | Option<**String**> | The extension of the file: .view.lkml, .model.lkml, etc | [optional][readonly]
**mime_type** | Option<**String**> | File mime type | [optional][readonly]
**editable** | Option<**bool**> | State of editability for the file. | [optional][readonly]
**git_status** | Option<[**crate::models::GitStatus**](GitStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


