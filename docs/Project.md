# Project

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**String**> | Project Id | [optional][readonly]
**name** | Option<**String**> | Project display name | [optional]
**uses_git** | Option<**bool**> | If true the project is configured with a git repository | [optional][readonly]
**git_remote_url** | Option<**String**> | Git remote repository url | [optional]
**git_username** | Option<**String**> | Git username for HTTPS authentication. (For production only, if using user attributes.) | [optional]
**git_password** | Option<**String**> | (Write-Only) Git password for HTTPS authentication. (For production only, if using user attributes.) | [optional]
**git_production_branch_name** | Option<**String**> | Git production branch name. Defaults to master. Supported only in Looker 21.0 and higher. | [optional]
**use_git_cookie_auth** | Option<**bool**> | If true, the project uses a git cookie for authentication. | [optional]
**git_username_user_attribute** | Option<**String**> | User attribute name for username in per-user HTTPS authentication. | [optional]
**git_password_user_attribute** | Option<**String**> | User attribute name for password in per-user HTTPS authentication. | [optional]
**git_service_name** | Option<**String**> | Name of the git service provider | [optional]
**git_application_server_http_port** | Option<**i64**> | Port that HTTP(S) application server is running on (for PRs, file browsing, etc.) | [optional]
**git_application_server_http_scheme** | Option<**String**> | Scheme that is running on application server (for PRs, file browsing, etc.) | [optional]
**deploy_secret** | Option<**String**> | (Write-Only) Optional secret token with which to authenticate requests to the webhook deploy endpoint. If not set, endpoint is unauthenticated. | [optional]
**unset_deploy_secret** | Option<**bool**> | (Write-Only) When true, unsets the deploy secret to allow unauthenticated access to the webhook deploy endpoint. | [optional]
**pull_request_mode** | Option<**String**> | The git pull request policy for this project. Valid values are: \"off\", \"links\", \"recommended\", \"required\". | [optional]
**validation_required** | Option<**bool**> | Validation policy: If true, the project must pass validation checks before project changes can be committed to the git repository | [optional]
**git_release_mgmt_enabled** | Option<**bool**> | If true, advanced git release management is enabled for this project | [optional]
**allow_warnings** | Option<**bool**> | Validation policy: If true, the project can be committed with warnings when `validation_required` is true. (`allow_warnings` does nothing if `validation_required` is false). | [optional]
**is_example** | Option<**bool**> | If true the project is an example project and cannot be modified | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


