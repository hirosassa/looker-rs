# ScheduledPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of this scheduled plan | [optional]
**user_id** | Option<**i64**> | User Id which owns this scheduled plan | [optional]
**run_as_recipient** | Option<**bool**> | Whether schedule is run as recipient (only applicable for email recipients) | [optional]
**enabled** | Option<**bool**> | Whether the ScheduledPlan is enabled | [optional]
**look_id** | Option<**i64**> | Id of a look | [optional]
**dashboard_id** | Option<**i64**> | Id of a dashboard | [optional]
**lookml_dashboard_id** | Option<**String**> | Id of a LookML dashboard | [optional]
**filters_string** | Option<**String**> | Query string to run look or dashboard with | [optional]
**dashboard_filters** | Option<**String**> | (DEPRECATED) Alias for filters_string field | [optional]
**require_results** | Option<**bool**> | Delivery should occur if running the dashboard or look returns results | [optional]
**require_no_results** | Option<**bool**> | Delivery should occur if the dashboard look does not return results | [optional]
**require_change** | Option<**bool**> | Delivery should occur if data have changed since the last run | [optional]
**send_all_results** | Option<**bool**> | Will run an unlimited query and send all results. | [optional]
**crontab** | Option<**String**> | Vixie-Style crontab specification when to run | [optional]
**datagroup** | Option<**String**> | Name of a datagroup; if specified will run when datagroup triggered (can't be used with cron string) | [optional]
**timezone** | Option<**String**> | Timezone for interpreting the specified crontab (default is Looker instance timezone) | [optional]
**query_id** | Option<**String**> | Query id | [optional]
**scheduled_plan_destination** | Option<[**Vec<crate::models::ScheduledPlanDestination>**](ScheduledPlanDestination.md)> | Scheduled plan destinations | [optional]
**run_once** | Option<**bool**> | Whether the plan in question should only be run once (usually for testing) | [optional]
**include_links** | Option<**bool**> | Whether links back to Looker should be included in this ScheduledPlan | [optional]
**pdf_paper_size** | Option<**String**> | The size of paper the PDF should be formatted to fit. Valid values are: \"letter\", \"legal\", \"tabloid\", \"a0\", \"a1\", \"a2\", \"a3\", \"a4\", \"a5\". | [optional]
**pdf_landscape** | Option<**bool**> | Whether the PDF should be formatted for landscape orientation | [optional]
**embed** | Option<**bool**> | Whether this schedule is in an embed context or not | [optional]
**color_theme** | Option<**String**> | Color scheme of the dashboard if applicable | [optional]
**long_tables** | Option<**bool**> | Whether or not to expand table vis to full length | [optional]
**inline_table_width** | Option<**i64**> | The pixel width at which we render the inline table visualizations | [optional]
**id** | Option<**i64**> | Unique Id | [optional][readonly]
**created_at** | Option<**String**> | Date and time when ScheduledPlan was created | [optional][readonly]
**updated_at** | Option<**String**> | Date and time when ScheduledPlan was last updated | [optional][readonly]
**title** | Option<**String**> | Title | [optional][readonly]
**user** | Option<[**crate::models::UserPublic**](UserPublic.md)> |  | [optional]
**next_run_at** | Option<**String**> | When the ScheduledPlan will next run (null if running once) | [optional][readonly]
**last_run_at** | Option<**String**> | When the ScheduledPlan was last run | [optional][readonly]
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


