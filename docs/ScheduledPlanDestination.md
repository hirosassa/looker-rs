# ScheduledPlanDestination

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**scheduled_plan_id** | Option<**i64**> | Id of a scheduled plan you own | [optional]
**format** | Option<**String**> | The data format to send to the given destination. Supported formats vary by destination, but include: \"txt\", \"csv\", \"inline_json\", \"json\", \"json_detail\", \"xlsx\", \"html\", \"wysiwyg_pdf\", \"assembled_pdf\", \"wysiwyg_png\" | [optional]
**apply_formatting** | Option<**bool**> | Are values formatted? (containing currency symbols, digit separators, etc. | [optional]
**apply_vis** | Option<**bool**> | Whether visualization options are applied to the results. | [optional]
**address** | Option<**String**> | Address for recipient. For email e.g. 'user@example.com'. For webhooks e.g. 'https://domain/path'. For Amazon S3 e.g. 's3://bucket-name/path/'. For SFTP e.g. 'sftp://host-name/path/'.  | [optional]
**looker_recipient** | Option<**bool**> | Whether the recipient is a Looker user on the current instance (only applicable for email recipients) | [optional][readonly]
**_type** | Option<**String**> | Type of the address ('email', 'webhook', 's3', or 'sftp') | [optional]
**parameters** | Option<**String**> | JSON object containing parameters for external scheduling. For Amazon S3, this requires keys and values for access_key_id and region. For SFTP, this requires a key and value for username. | [optional]
**secret_parameters** | Option<**String**> | (Write-Only) JSON object containing secret parameters for external scheduling. For Amazon S3, this requires a key and value for secret_access_key. For SFTP, this requires a key and value for password. | [optional]
**message** | Option<**String**> | Optional message to be included in scheduled emails | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


