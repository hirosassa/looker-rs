# ProjectError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**String**> | A stable token that uniquely identifies this class of error, ignoring parameter values. Error message text may vary due to parameters or localization, but error codes do not. For example, a \"File not found\" error will have the same error code regardless of the filename in question or the user's display language | [optional][readonly]
**severity** | Option<**String**> | Severity: fatal, error, warning, info, success | [optional][readonly]
**kind** | Option<**String**> | Error classification: syntax, deprecation, model_configuration, etc | [optional][readonly]
**message** | Option<**String**> | Error message which may contain information such as dashboard or model names that may be considered sensitive in some use cases. Avoid storing or sending this message outside of Looker | [optional][readonly]
**field_name** | Option<**String**> | The field associated with this error | [optional][readonly]
**file_path** | Option<**String**> | Name of the file containing this error | [optional][readonly]
**line_number** | Option<**i64**> | Line number in the file of this error | [optional][readonly]
**model_id** | Option<**String**> | The model associated with this error | [optional][readonly]
**explore** | Option<**String**> | The explore associated with this error | [optional][readonly]
**help_url** | Option<**String**> | A link to Looker documentation about this error | [optional][readonly]
**params** | Option<**::std::collections::HashMap<String, String>**> | Error parameters | [optional][readonly]
**sanitized_message** | Option<**String**> | A version of the error message that does not contain potentially sensitive information. Suitable for situations in which messages are stored or sent to consumers outside of Looker, such as external logs. Sanitized messages will display \"(?)\" where sensitive information would appear in the corresponding non-sanitized message | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


