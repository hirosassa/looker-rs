# BackupConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**_type** | Option<**String**> | Type of backup: looker-s3 or custom-s3 | [optional]
**custom_s3_bucket** | Option<**String**> | Name of bucket for custom-s3 backups | [optional]
**custom_s3_bucket_region** | Option<**String**> | Name of region where the bucket is located | [optional]
**custom_s3_key** | Option<**String**> | (Write-Only) AWS S3 key used for custom-s3 backups | [optional]
**custom_s3_secret** | Option<**String**> | (Write-Only) AWS S3 secret used for custom-s3 backups | [optional]
**url** | Option<**String**> | Link to get this item | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


