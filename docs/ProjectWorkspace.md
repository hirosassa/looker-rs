# ProjectWorkspace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**project_id** | Option<**String**> | The id of the project | [optional][readonly]
**workspace_id** | Option<**String**> | The id of the local workspace containing the project files | [optional][readonly]
**git_status** | Option<**String**> | The status of the local git directory | [optional][readonly]
**git_head** | Option<**String**> | Git head revision name | [optional][readonly]
**dependency_status** | Option<**String**> | Status of the dependencies in your project. Valid values are: \"lock_optional\", \"lock_required\", \"lock_error\", \"install_none\". | [optional][readonly]
**git_branch** | Option<[**crate::models::GitBranch**](GitBranch.md)> |  | [optional]
**lookml_type** | Option<**String**> | The lookml syntax used by all files in this project | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


