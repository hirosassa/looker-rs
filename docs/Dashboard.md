# Dashboard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**content_favorite_id** | Option<**i64**> | Content Favorite Id | [optional][readonly]
**content_metadata_id** | Option<**i64**> | Id of content metadata | [optional][readonly]
**description** | Option<**String**> | Description | [optional]
**hidden** | Option<**bool**> | Is Hidden | [optional]
**id** | Option<**String**> | Unique Id | [optional][readonly]
**model** | Option<[**crate::models::LookModel**](LookModel.md)> |  | [optional]
**query_timezone** | Option<**String**> | Timezone in which the Dashboard will run by default. | [optional]
**readonly** | Option<**bool**> | Is Read-only | [optional][readonly]
**refresh_interval** | Option<**String**> | Refresh Interval, as a time duration phrase like \"2 hours 30 minutes\". A number with no time units will be interpreted as whole seconds. | [optional]
**refresh_interval_to_i** | Option<**i64**> | Refresh Interval in milliseconds | [optional][readonly]
**folder** | Option<[**crate::models::FolderBase**](FolderBase.md)> |  | [optional]
**title** | Option<**String**> | Dashboard Title | [optional]
**user_id** | Option<**i64**> | Id of User | [optional][readonly]
**slug** | Option<**String**> | Content Metadata Slug | [optional]
**preferred_viewer** | Option<**String**> | The preferred route for viewing this dashboard (ie: dashboards or dashboards-next) | [optional]
**space** | Option<[**crate::models::SpaceBase**](SpaceBase.md)> |  | [optional]
**alert_sync_with_dashboard_filter_enabled** | Option<**bool**> | Enables alerts to keep in sync with dashboard filter changes - only available in alerts 2.0 (beta) | [optional]
**background_color** | Option<**String**> | Background color | [optional]
**created_at** | Option<**String**> | Time that the Dashboard was created. | [optional][readonly]
**crossfilter_enabled** | Option<**bool**> | Enables crossfiltering in dashboards - only available in dashboards-next (beta) | [optional]
**dashboard_elements** | Option<[**Vec<crate::models::DashboardElement>**](DashboardElement.md)> | Elements | [optional][readonly]
**dashboard_filters** | Option<[**Vec<crate::models::DashboardFilter>**](DashboardFilter.md)> | Filters | [optional][readonly]
**dashboard_layouts** | Option<[**Vec<crate::models::DashboardLayout>**](DashboardLayout.md)> | Layouts | [optional][readonly]
**deleted** | Option<**bool**> | Whether or not a dashboard is 'soft' deleted. | [optional]
**deleted_at** | Option<**String**> | Time that the Dashboard was 'soft' deleted. | [optional][readonly]
**deleter_id** | Option<**i64**> | Id of User that 'soft' deleted the dashboard. | [optional][readonly]
**edit_uri** | Option<**String**> | Relative path of URI of LookML file to edit the dashboard (LookML dashboard only). | [optional][readonly]
**favorite_count** | Option<**i64**> | Number of times favorited | [optional][readonly]
**last_accessed_at** | Option<**String**> | Time the dashboard was last accessed | [optional][readonly]
**last_viewed_at** | Option<**String**> | Time last viewed in the Looker web UI | [optional][readonly]
**load_configuration** | Option<**String**> | configuration option that governs how dashboard loading will happen. | [optional]
**lookml_link_id** | Option<**String**> | Links this dashboard to a particular LookML dashboard such that calling a **sync** operation on that LookML dashboard will update this dashboard to match. | [optional]
**show_filters_bar** | Option<**bool**> | Show filters bar.  **Security Note:** This property only affects the *cosmetic* appearance of the dashboard, not a user's ability to access data. Hiding the filters bar does **NOT** prevent users from changing filters by other means. For information on how to set up secure data access control policies, see [Control User Access to Data](https://looker.com/docs/r/api/control-access) | [optional]
**show_title** | Option<**bool**> | Show title | [optional]
**space_id** | Option<**String**> | Id of Space | [optional]
**folder_id** | Option<**String**> | Id of folder | [optional]
**text_tile_text_color** | Option<**String**> | Color of text on text tiles | [optional]
**tile_background_color** | Option<**String**> | Tile background color | [optional]
**tile_text_color** | Option<**String**> | Tile text color | [optional]
**title_color** | Option<**String**> | Title color | [optional]
**view_count** | Option<**i64**> | Number of times viewed in the Looker web UI | [optional][readonly]
**appearance** | Option<[**crate::models::DashboardAppearance**](DashboardAppearance.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


