# DbConnectionBase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**name** | Option<**String**> | Name of the connection. Also used as the unique identifier | [optional][readonly]
**dialect** | Option<[**crate::models::Dialect**](Dialect.md)> |  | [optional]
**snippets** | Option<[**Vec<crate::models::Snippet>**](Snippet.md)> | SQL Runner snippets for this connection | [optional][readonly]
**pdts_enabled** | Option<**bool**> | True if PDTs are enabled on this connection | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


