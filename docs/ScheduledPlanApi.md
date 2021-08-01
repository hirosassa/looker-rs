# \ScheduledPlanApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_scheduled_plans**](ScheduledPlanApi.md#all_scheduled_plans) | **GET** /scheduled_plans | Get All Scheduled Plans
[**create_scheduled_plan**](ScheduledPlanApi.md#create_scheduled_plan) | **POST** /scheduled_plans | Create Scheduled Plan
[**delete_scheduled_plan**](ScheduledPlanApi.md#delete_scheduled_plan) | **DELETE** /scheduled_plans/{scheduled_plan_id} | Delete Scheduled Plan
[**scheduled_plan**](ScheduledPlanApi.md#scheduled_plan) | **GET** /scheduled_plans/{scheduled_plan_id} | Get Scheduled Plan
[**scheduled_plan_run_once**](ScheduledPlanApi.md#scheduled_plan_run_once) | **POST** /scheduled_plans/run_once | Run Scheduled Plan Once
[**scheduled_plan_run_once_by_id**](ScheduledPlanApi.md#scheduled_plan_run_once_by_id) | **POST** /scheduled_plans/{scheduled_plan_id}/run_once | Run Scheduled Plan Once by Id
[**scheduled_plans_for_dashboard**](ScheduledPlanApi.md#scheduled_plans_for_dashboard) | **GET** /scheduled_plans/dashboard/{dashboard_id} | Scheduled Plans for Dashboard
[**scheduled_plans_for_look**](ScheduledPlanApi.md#scheduled_plans_for_look) | **GET** /scheduled_plans/look/{look_id} | Scheduled Plans for Look
[**scheduled_plans_for_lookml_dashboard**](ScheduledPlanApi.md#scheduled_plans_for_lookml_dashboard) | **GET** /scheduled_plans/lookml_dashboard/{lookml_dashboard_id} | Scheduled Plans for LookML Dashboard
[**scheduled_plans_for_space**](ScheduledPlanApi.md#scheduled_plans_for_space) | **GET** /scheduled_plans/space/{space_id} | Scheduled Plans for Space
[**update_scheduled_plan**](ScheduledPlanApi.md#update_scheduled_plan) | **PATCH** /scheduled_plans/{scheduled_plan_id} | Update Scheduled Plan



## all_scheduled_plans

> Vec<crate::models::ScheduledPlan> all_scheduled_plans(user_id, fields, all_users)
Get All Scheduled Plans

### List All Scheduled Plans  Returns all scheduled plans which belong to the caller or given user.  If no user_id is provided, this function returns the scheduled plans owned by the caller.   To list all schedules for all users, pass `all_users=true`.   The caller must have `see_schedules` permission to see other users' scheduled plans.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**i64**> | Return scheduled plans belonging to this user_id. If not provided, returns scheduled plans owned by the caller. |  |
**fields** | Option<**String**> | Comma delimited list of field names. If provided, only the fields specified will be included in the response |  |
**all_users** | Option<**bool**> | Return scheduled plans belonging to all users (caller needs see_schedules permission) |  |

### Return type

[**Vec<crate::models::ScheduledPlan>**](ScheduledPlan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_scheduled_plan

> crate::models::ScheduledPlan create_scheduled_plan(body)
Create Scheduled Plan

### Create a Scheduled Plan  Create a scheduled plan to render a Look or Dashboard on a recurring schedule.  To create a scheduled plan, you MUST provide values for the following fields: `name` and `look_id`, `dashboard_id`, `lookml_dashboard_id`, or `query_id` and `cron_tab` or `datagroup` and at least one scheduled_plan_destination  A scheduled plan MUST have at least one scheduled_plan_destination defined.  When `look_id` is set, `require_no_results`, `require_results`, and `require_change` are all required.  If `create_scheduled_plan` fails with a 422 error, be sure to look at the error messages in the response which will explain exactly what fields are missing or values that are incompatible.  The queries that provide the data for the look or dashboard are run in the context of user account that owns the scheduled plan.  When `run_as_recipient` is `false` or not specified, the queries that provide the data for the look or dashboard are run in the context of user account that owns the scheduled plan.  When `run_as_recipient` is `true` and all the email recipients are Looker user accounts, the queries are run in the context of each recipient, so different recipients may see different data from the same scheduled render of a look or dashboard. For more details, see [Run As Recipient](https://looker.com/docs/r/admin/run-as-recipient).  Admins can create and modify scheduled plans on behalf of other users by specifying a user id. Non-admin users may not create or modify scheduled plans by or for other users.  #### Email Permissions:  For details about permissions required to schedule delivery to email and the safeguards Looker offers to protect against sending to unauthorized email destinations, see [Email Domain Whitelist for Scheduled Looks](https://docs.looker.com/r/api/embed-permissions).   #### Scheduled Plan Destination Formats  Scheduled plan destinations must specify the data format to produce and send to the destination.  Formats:  | format | Description | :-----------: | :--- | | json | A JSON object containing a `data` property which contains an array of JSON objects, one per row. No metadata. | json_detail | Row data plus metadata describing the fields, pivots, table calcs, and other aspects of the query | inline_json | Same as the JSON format, except that the `data` property is a string containing JSON-escaped row data. Additional properties describe the data operation. This format is primarily used to send data to web hooks so that the web hook doesn't have to re-encode the JSON row data in order to pass it on to its ultimate destination. | csv | Comma separated values with a header | txt | Tab separated values with a header | html | Simple html | xlsx | MS Excel spreadsheet | wysiwyg_pdf | Dashboard rendered in a tiled layout to produce a PDF document | assembled_pdf | Dashboard rendered in a single column layout to produce a PDF document | wysiwyg_png | Dashboard rendered in a tiled layout to produce a PNG image ||  Valid formats vary by destination type and source object. `wysiwyg_pdf` is only valid for dashboards, for example.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ScheduledPlan**](ScheduledPlan.md) | Scheduled Plan | [required] |

### Return type

[**crate::models::ScheduledPlan**](ScheduledPlan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scheduled_plan

> String delete_scheduled_plan(scheduled_plan_id)
Delete Scheduled Plan

### Delete a Scheduled Plan  Normal users can only delete their own scheduled plans. Admins can delete other users' scheduled plans. This delete cannot be undone. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheduled_plan_id** | **i64** | Scheduled Plan Id | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scheduled_plan

> crate::models::ScheduledPlan scheduled_plan(scheduled_plan_id, fields)
Get Scheduled Plan

### Get Information About a Scheduled Plan  Admins can fetch information about other users' Scheduled Plans. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheduled_plan_id** | **i64** | Scheduled Plan Id | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::ScheduledPlan**](ScheduledPlan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scheduled_plan_run_once

> crate::models::ScheduledPlan scheduled_plan_run_once(body)
Run Scheduled Plan Once

### Run a Scheduled Plan Immediately  Create a scheduled plan that runs only once, and immediately.  This can be useful for testing a Scheduled Plan before committing to a production schedule.  Admins can create scheduled plans on behalf of other users by specifying a user id.  This API is rate limited to prevent it from being used for relay spam or DoS attacks  #### Email Permissions:  For details about permissions required to schedule delivery to email and the safeguards Looker offers to protect against sending to unauthorized email destinations, see [Email Domain Whitelist for Scheduled Looks](https://docs.looker.com/r/api/embed-permissions).   #### Scheduled Plan Destination Formats  Scheduled plan destinations must specify the data format to produce and send to the destination.  Formats:  | format | Description | :-----------: | :--- | | json | A JSON object containing a `data` property which contains an array of JSON objects, one per row. No metadata. | json_detail | Row data plus metadata describing the fields, pivots, table calcs, and other aspects of the query | inline_json | Same as the JSON format, except that the `data` property is a string containing JSON-escaped row data. Additional properties describe the data operation. This format is primarily used to send data to web hooks so that the web hook doesn't have to re-encode the JSON row data in order to pass it on to its ultimate destination. | csv | Comma separated values with a header | txt | Tab separated values with a header | html | Simple html | xlsx | MS Excel spreadsheet | wysiwyg_pdf | Dashboard rendered in a tiled layout to produce a PDF document | assembled_pdf | Dashboard rendered in a single column layout to produce a PDF document | wysiwyg_png | Dashboard rendered in a tiled layout to produce a PNG image ||  Valid formats vary by destination type and source object. `wysiwyg_pdf` is only valid for dashboards, for example.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ScheduledPlan**](ScheduledPlan.md) | Scheduled Plan | [required] |

### Return type

[**crate::models::ScheduledPlan**](ScheduledPlan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scheduled_plan_run_once_by_id

> crate::models::ScheduledPlan scheduled_plan_run_once_by_id(scheduled_plan_id, body)
Run Scheduled Plan Once by Id

### Run a Scheduled Plan By Id Immediately This function creates a run-once schedule plan based on an existing scheduled plan, applies modifications (if any) to the new scheduled plan, and runs the new schedule plan immediately. This can be useful for testing modifications to an existing scheduled plan before committing to a production schedule.  This function internally performs the following operations:  1. Copies the properties of the existing scheduled plan into a new scheduled plan 2. Copies any properties passed in the JSON body of this request into the new scheduled plan (replacing the original values) 3. Creates the new scheduled plan 4. Runs the new scheduled plan  The original scheduled plan is not modified by this operation. Admins can create, modify, and run scheduled plans on behalf of other users by specifying a user id. Non-admins can only create, modify, and run their own scheduled plans.  #### Email Permissions:  For details about permissions required to schedule delivery to email and the safeguards Looker offers to protect against sending to unauthorized email destinations, see [Email Domain Whitelist for Scheduled Looks](https://docs.looker.com/r/api/embed-permissions).   #### Scheduled Plan Destination Formats  Scheduled plan destinations must specify the data format to produce and send to the destination.  Formats:  | format | Description | :-----------: | :--- | | json | A JSON object containing a `data` property which contains an array of JSON objects, one per row. No metadata. | json_detail | Row data plus metadata describing the fields, pivots, table calcs, and other aspects of the query | inline_json | Same as the JSON format, except that the `data` property is a string containing JSON-escaped row data. Additional properties describe the data operation. This format is primarily used to send data to web hooks so that the web hook doesn't have to re-encode the JSON row data in order to pass it on to its ultimate destination. | csv | Comma separated values with a header | txt | Tab separated values with a header | html | Simple html | xlsx | MS Excel spreadsheet | wysiwyg_pdf | Dashboard rendered in a tiled layout to produce a PDF document | assembled_pdf | Dashboard rendered in a single column layout to produce a PDF document | wysiwyg_png | Dashboard rendered in a tiled layout to produce a PNG image ||  Valid formats vary by destination type and source object. `wysiwyg_pdf` is only valid for dashboards, for example.    This API is rate limited to prevent it from being used for relay spam or DoS attacks  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheduled_plan_id** | **i64** | Id of schedule plan to copy and run | [required] |
**body** | Option<[**WriteScheduledPlan**](WriteScheduledPlan.md)> | Property values to apply to the newly copied scheduled plan before running it |  |

### Return type

[**crate::models::ScheduledPlan**](ScheduledPlan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scheduled_plans_for_dashboard

> Vec<crate::models::ScheduledPlan> scheduled_plans_for_dashboard(dashboard_id, user_id, all_users, fields)
Scheduled Plans for Dashboard

### Get Scheduled Plans for a Dashboard  Returns all scheduled plans for a dashboard which belong to the caller or given user.  If no user_id is provided, this function returns the scheduled plans owned by the caller.   To list all schedules for all users, pass `all_users=true`.   The caller must have `see_schedules` permission to see other users' scheduled plans.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **i64** | Dashboard Id | [required] |
**user_id** | Option<**i64**> | User Id (default is requesting user if not specified) |  |
**all_users** | Option<**bool**> | Return scheduled plans belonging to all users for the dashboard |  |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::ScheduledPlan>**](ScheduledPlan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scheduled_plans_for_look

> Vec<crate::models::ScheduledPlan> scheduled_plans_for_look(look_id, user_id, fields, all_users)
Scheduled Plans for Look

### Get Scheduled Plans for a Look  Returns all scheduled plans for a look which belong to the caller or given user.  If no user_id is provided, this function returns the scheduled plans owned by the caller.   To list all schedules for all users, pass `all_users=true`.   The caller must have `see_schedules` permission to see other users' scheduled plans.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**look_id** | **i64** | Look Id | [required] |
**user_id** | Option<**i64**> | User Id (default is requesting user if not specified) |  |
**fields** | Option<**String**> | Requested fields. |  |
**all_users** | Option<**bool**> | Return scheduled plans belonging to all users for the look |  |

### Return type

[**Vec<crate::models::ScheduledPlan>**](ScheduledPlan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scheduled_plans_for_lookml_dashboard

> Vec<crate::models::ScheduledPlan> scheduled_plans_for_lookml_dashboard(lookml_dashboard_id, user_id, fields, all_users)
Scheduled Plans for LookML Dashboard

### Get Scheduled Plans for a LookML Dashboard  Returns all scheduled plans for a LookML Dashboard which belong to the caller or given user.  If no user_id is provided, this function returns the scheduled plans owned by the caller.   To list all schedules for all users, pass `all_users=true`.   The caller must have `see_schedules` permission to see other users' scheduled plans.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookml_dashboard_id** | **String** | LookML Dashboard Id | [required] |
**user_id** | Option<**i64**> | User Id (default is requesting user if not specified) |  |
**fields** | Option<**String**> | Requested fields. |  |
**all_users** | Option<**bool**> | Return scheduled plans belonging to all users for the dashboard |  |

### Return type

[**Vec<crate::models::ScheduledPlan>**](ScheduledPlan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scheduled_plans_for_space

> Vec<crate::models::ScheduledPlan> scheduled_plans_for_space(space_id, fields)
Scheduled Plans for Space

### Get Scheduled Plans for a Space  Returns scheduled plans owned by the caller for a given space id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**space_id** | **i64** | Space Id | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::ScheduledPlan>**](ScheduledPlan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_scheduled_plan

> crate::models::ScheduledPlan update_scheduled_plan(scheduled_plan_id, body)
Update Scheduled Plan

### Update a Scheduled Plan  Admins can update other users' Scheduled Plans.  Note: Any scheduled plan destinations specified in an update will **replace** all scheduled plan destinations currently defined for the scheduled plan.  For Example: If a scheduled plan has destinations A, B, and C, and you call update on this scheduled plan specifying only B in the destinations, then destinations A and C will be deleted by the update.  Updating a scheduled plan to assign null or an empty array to the scheduled_plan_destinations property is an error, as a scheduled plan must always have at least one destination.  If you omit the scheduled_plan_destinations property from the object passed to update, then the destinations defined on the original scheduled plan will remain unchanged.  #### Email Permissions:  For details about permissions required to schedule delivery to email and the safeguards Looker offers to protect against sending to unauthorized email destinations, see [Email Domain Whitelist for Scheduled Looks](https://docs.looker.com/r/api/embed-permissions).   #### Scheduled Plan Destination Formats  Scheduled plan destinations must specify the data format to produce and send to the destination.  Formats:  | format | Description | :-----------: | :--- | | json | A JSON object containing a `data` property which contains an array of JSON objects, one per row. No metadata. | json_detail | Row data plus metadata describing the fields, pivots, table calcs, and other aspects of the query | inline_json | Same as the JSON format, except that the `data` property is a string containing JSON-escaped row data. Additional properties describe the data operation. This format is primarily used to send data to web hooks so that the web hook doesn't have to re-encode the JSON row data in order to pass it on to its ultimate destination. | csv | Comma separated values with a header | txt | Tab separated values with a header | html | Simple html | xlsx | MS Excel spreadsheet | wysiwyg_pdf | Dashboard rendered in a tiled layout to produce a PDF document | assembled_pdf | Dashboard rendered in a single column layout to produce a PDF document | wysiwyg_png | Dashboard rendered in a tiled layout to produce a PNG image ||  Valid formats vary by destination type and source object. `wysiwyg_pdf` is only valid for dashboards, for example.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheduled_plan_id** | **i64** | Scheduled Plan Id | [required] |
**body** | [**ScheduledPlan**](ScheduledPlan.md) | Scheduled Plan | [required] |

### Return type

[**crate::models::ScheduledPlan**](ScheduledPlan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

