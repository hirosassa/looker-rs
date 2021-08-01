# HomepageItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**content_created_by** | Option<**String**> | Name of user who created the content this item is based on | [optional][readonly]
**content_favorite_id** | Option<**i64**> | Content favorite id associated with the item this content is based on | [optional][readonly]
**content_metadata_id** | Option<**i64**> | Content metadata id associated with the item this content is based on | [optional][readonly]
**content_updated_at** | Option<**String**> | Last time the content that this item is based on was updated | [optional][readonly]
**custom_description** | Option<**String**> | Custom description entered by the user, if present | [optional]
**custom_image_data_base64** | Option<**String**> | (Write-Only) base64 encoded image data | [optional]
**custom_image_url** | Option<**String**> | Custom image_url entered by the user, if present | [optional][readonly]
**custom_title** | Option<**String**> | Custom title entered by the user, if present | [optional]
**custom_url** | Option<**String**> | Custom url entered by the user, if present | [optional]
**dashboard_id** | Option<**i64**> | Dashboard to base this item on | [optional]
**description** | Option<**String**> | The actual description for display | [optional][readonly]
**favorite_count** | Option<**i64**> | Number of times content has been favorited, if present | [optional][readonly]
**homepage_section_id** | Option<**String**> | Associated Homepage Section | [optional]
**id** | Option<**String**> | Unique Id | [optional][readonly]
**image_url** | Option<**String**> | The actual image_url for display | [optional][readonly]
**location** | Option<**String**> | The container folder name of the content | [optional][readonly]
**look_id** | Option<**i64**> | Look to base this item on | [optional]
**lookml_dashboard_id** | Option<**String**> | LookML Dashboard to base this item on | [optional]
**order** | Option<**i64**> | An arbitrary integer representing the sort order within the section | [optional]
**section_fetch_time** | Option<**f32**> | Number of seconds it took to fetch the section this item is in | [optional][readonly]
**title** | Option<**String**> | The actual title for display | [optional][readonly]
**url** | Option<**String**> | The actual url for display | [optional][readonly]
**use_custom_description** | Option<**bool**> | Whether the custom description should be used instead of the content description, if the item is associated with content | [optional]
**use_custom_image** | Option<**bool**> | Whether the custom image should be used instead of the content image, if the item is associated with content | [optional]
**use_custom_title** | Option<**bool**> | Whether the custom title should be used instead of the content title, if the item is associated with content | [optional]
**use_custom_url** | Option<**bool**> | Whether the custom url should be used instead of the content url, if the item is associated with content | [optional]
**view_count** | Option<**i64**> | Number of times content has been viewed, if present | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


