# DbConnectionOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context** | Option<**String**> | Context in which to override (`pdt` is the only allowed value) | [optional]
**host** | Option<**String**> | Host name/address of server | [optional]
**port** | Option<**String**> | Port number on server | [optional]
**username** | Option<**String**> | Username for server authentication | [optional]
**password** | Option<**String**> | (Write-Only) Password for server authentication | [optional]
**has_password** | Option<**bool**> | Whether or not the password is overridden in this context | [optional][readonly]
**certificate** | Option<**String**> | (Write-Only) Base64 encoded Certificate body for server authentication (when appropriate for dialect). | [optional]
**file_type** | Option<**String**> | (Write-Only) Certificate keyfile type - .json or .p12 | [optional]
**database** | Option<**String**> | Database name | [optional]
**schema** | Option<**String**> | Scheme name | [optional]
**jdbc_additional_params** | Option<**String**> | Additional params to add to JDBC connection string | [optional]
**after_connect_statements** | Option<**String**> | SQL statements (semicolon separated) to issue after connecting to the database. Requires `custom_after_connect_statements` license feature | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


