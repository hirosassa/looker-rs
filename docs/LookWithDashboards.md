# LookWithDashboards

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**content_metadata_id** | Option<**i64**> | Id of content metadata | [optional][readonly]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**title** | Option<**String**> | Look Title | [optional]
**content_favorite_id** | Option<**i64**> | Content Favorite Id | [optional][readonly]
**created_at** | Option<**String**> | Time that the Look was created. | [optional][readonly]
**deleted** | Option<**bool**> | Whether or not a look is 'soft' deleted. | [optional]
**deleted_at** | Option<**String**> | Time that the Look was deleted. | [optional][readonly]
**deleter_id** | Option<**i64**> | Id of User that deleted the look. | [optional][readonly]
**description** | Option<**String**> | Description | [optional]
**embed_url** | Option<**String**> | Embed Url | [optional][readonly]
**excel_file_url** | Option<**String**> | Excel File Url | [optional][readonly]
**favorite_count** | Option<**i64**> | Number of times favorited | [optional][readonly]
**google_spreadsheet_formula** | Option<**String**> | Google Spreadsheet Formula | [optional][readonly]
**image_embed_url** | Option<**String**> | Image Embed Url | [optional][readonly]
**is_run_on_load** | Option<**bool**> | auto-run query when Look viewed | [optional]
**last_accessed_at** | Option<**String**> | Time that the Look was last accessed by any user | [optional][readonly]
**last_updater_id** | Option<**i64**> | Id of User that last updated the look. | [optional][readonly]
**last_viewed_at** | Option<**String**> | Time last viewed in the Looker web UI | [optional][readonly]
**model** | Option<[**crate::models::LookModel**](LookModel.md)> |  | [optional]
**public** | Option<**bool**> | Is Public | [optional]
**public_slug** | Option<**String**> | Public Slug | [optional][readonly]
**public_url** | Option<**String**> | Public Url | [optional][readonly]
**query_id** | Option<**i64**> | Query Id | [optional]
**short_url** | Option<**String**> | Short Url | [optional][readonly]
**folder** | Option<[**crate::models::FolderBase**](FolderBase.md)> |  | [optional]
**folder_id** | Option<**String**> | Folder Id | [optional]
**updated_at** | Option<**String**> | Time that the Look was updated. | [optional][readonly]
**user_id** | Option<**i64**> | User Id | [optional]
**view_count** | Option<**i64**> | Number of times viewed in the Looker web UI | [optional][readonly]
**user** | Option<[**crate::models::UserIdOnly**](UserIdOnly.md)> |  | [optional]
**space_id** | Option<**String**> | Space Id | [optional]
**space** | Option<[**crate::models::SpaceBase**](SpaceBase.md)> |  | [optional]
**dashboards** | Option<[**Vec<crate::models::DashboardBase>**](DashboardBase.md)> | Dashboards | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


