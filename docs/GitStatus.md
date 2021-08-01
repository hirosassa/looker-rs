# GitStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | Git action: add, delete, etc | [optional][readonly]
**conflict** | Option<**bool**> | When true, changes to the local file conflict with the remote repository | [optional][readonly]
**revertable** | Option<**bool**> | When true, the file can be reverted to an earlier state | [optional][readonly]
**text** | Option<**String**> | Git description of the action | [optional][readonly]
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


