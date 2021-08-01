# ContentMeta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**name** | Option<**String**> | Name or title of underlying content | [optional][readonly]
**parent_id** | Option<**i64**> | Id of Parent Content | [optional][readonly]
**dashboard_id** | Option<**String**> | Id of associated dashboard when content_type is \"dashboard\" | [optional][readonly]
**look_id** | Option<**i64**> | Id of associated look when content_type is \"look\" | [optional][readonly]
**folder_id** | Option<**String**> | Id of associated folder when content_type is \"space\" | [optional][readonly]
**content_type** | Option<**String**> | Content Type (\"dashboard\", \"look\", or \"space\") | [optional][readonly]
**inherits** | Option<**bool**> | Whether content inherits its access levels from parent | [optional]
**inheriting_id** | Option<**i64**> | Id of Inherited Content | [optional][readonly]
**slug** | Option<**String**> | Content Slug | [optional][readonly]
**space_id** | Option<**String**> | Id of associated space when content_type is \"space\" | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


