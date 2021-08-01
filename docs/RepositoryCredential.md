# RepositoryCredential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**String**> | Unique Id | [optional][readonly]
**root_project_id** | Option<**String**> | Root project Id | [optional][readonly]
**remote_url** | Option<**String**> | Git remote repository url | [optional][readonly]
**git_username** | Option<**String**> | Git username for HTTPS authentication. | [optional]
**git_password** | Option<**String**> | (Write-Only) Git password for HTTPS authentication. | [optional]
**ssh_public_key** | Option<**String**> | Public deploy key for SSH authentication. | [optional]
**is_configured** | Option<**bool**> | Whether the credentials have been configured for the Git Repository. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


