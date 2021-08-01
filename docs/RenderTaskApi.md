# \RenderTaskApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dashboard_render_task**](RenderTaskApi.md#create_dashboard_render_task) | **POST** /render_tasks/dashboards/{dashboard_id}/{result_format} | Create Dashboard Render Task
[**create_look_render_task**](RenderTaskApi.md#create_look_render_task) | **POST** /render_tasks/looks/{look_id}/{result_format} | Create Look Render Task
[**create_lookml_dashboard_render_task**](RenderTaskApi.md#create_lookml_dashboard_render_task) | **POST** /render_tasks/lookml_dashboards/{dashboard_id}/{result_format} | Create Lookml Dashboard Render Task
[**create_query_render_task**](RenderTaskApi.md#create_query_render_task) | **POST** /render_tasks/queries/{query_id}/{result_format} | Create Query Render Task
[**render_task**](RenderTaskApi.md#render_task) | **GET** /render_tasks/{render_task_id} | Get Render Task
[**render_task_results**](RenderTaskApi.md#render_task_results) | **GET** /render_tasks/{render_task_id}/results | Render Task Results



## create_dashboard_render_task

> crate::models::RenderTask create_dashboard_render_task(dashboard_id, result_format, width, height, body, fields, pdf_paper_size, pdf_landscape)
Create Dashboard Render Task

### Create a new task to render a dashboard to a document or image.  Returns a render task object. To check the status of a render task, pass the render_task.id to [Get Render Task](#!/RenderTask/get_render_task). Once the render task is complete, you can download the resulting document or image using [Get Render Task Results](#!/RenderTask/get_render_task_results).  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **i64** | Id of dashboard to render | [required] |
**result_format** | **String** | Output type: pdf, png, or jpg | [required] |
**width** | **i64** | Output width in pixels | [required] |
**height** | **i64** | Output height in pixels | [required] |
**body** | [**CreateDashboardRenderTask**](CreateDashboardRenderTask.md) | Dashboard render task parameters | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**pdf_paper_size** | Option<**String**> | Paper size for pdf. Value can be one of: [\"letter\",\"legal\",\"tabloid\",\"a0\",\"a1\",\"a2\",\"a3\",\"a4\",\"a5\"] |  |
**pdf_landscape** | Option<**bool**> | Whether to render pdf in landscape paper orientation |  |

### Return type

[**crate::models::RenderTask**](RenderTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_look_render_task

> crate::models::RenderTask create_look_render_task(look_id, result_format, width, height, fields)
Create Look Render Task

### Create a new task to render a look to an image.  Returns a render task object. To check the status of a render task, pass the render_task.id to [Get Render Task](#!/RenderTask/get_render_task). Once the render task is complete, you can download the resulting document or image using [Get Render Task Results](#!/RenderTask/get_render_task_results).  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**look_id** | **i64** | Id of look to render | [required] |
**result_format** | **String** | Output type: png, or jpg | [required] |
**width** | **i64** | Output width in pixels | [required] |
**height** | **i64** | Output height in pixels | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::RenderTask**](RenderTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_lookml_dashboard_render_task

> crate::models::RenderTask create_lookml_dashboard_render_task(dashboard_id, result_format, width, height, body, fields, pdf_paper_size, pdf_landscape)
Create Lookml Dashboard Render Task

### Create a new task to render a lookml dashboard to a document or image.  # DEPRECATED:  Use [create_dashboard_render_task()](#!/RenderTask/create_dashboard_render_task) in API 4.0+  Returns a render task object. To check the status of a render task, pass the render_task.id to [Get Render Task](#!/RenderTask/get_render_task). Once the render task is complete, you can download the resulting document or image using [Get Render Task Results](#!/RenderTask/get_render_task_results).  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | Id of lookml dashboard to render | [required] |
**result_format** | **String** | Output type: pdf, png, or jpg | [required] |
**width** | **i64** | Output width in pixels | [required] |
**height** | **i64** | Output height in pixels | [required] |
**body** | [**CreateDashboardRenderTask**](CreateDashboardRenderTask.md) | Dashboard render task parameters | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**pdf_paper_size** | Option<**String**> | Paper size for pdf. Value can be one of: [\"letter\",\"legal\",\"tabloid\",\"a0\",\"a1\",\"a2\",\"a3\",\"a4\",\"a5\"] |  |
**pdf_landscape** | Option<**bool**> | Whether to render pdf in landscape |  |

### Return type

[**crate::models::RenderTask**](RenderTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_query_render_task

> crate::models::RenderTask create_query_render_task(query_id, result_format, width, height, fields)
Create Query Render Task

### Create a new task to render an existing query to an image.  Returns a render task object. To check the status of a render task, pass the render_task.id to [Get Render Task](#!/RenderTask/get_render_task). Once the render task is complete, you can download the resulting document or image using [Get Render Task Results](#!/RenderTask/get_render_task_results).  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | **i64** | Id of the query to render | [required] |
**result_format** | **String** | Output type: png or jpg | [required] |
**width** | **i64** | Output width in pixels | [required] |
**height** | **i64** | Output height in pixels | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::RenderTask**](RenderTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_task

> crate::models::RenderTask render_task(render_task_id, fields)
Get Render Task

### Get information about a render task.  Returns a render task object. To check the status of a render task, pass the render_task.id to [Get Render Task](#!/RenderTask/get_render_task). Once the render task is complete, you can download the resulting document or image using [Get Render Task Results](#!/RenderTask/get_render_task_results).  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**render_task_id** | **String** | Id of render task | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::RenderTask**](RenderTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_task_results

> String render_task_results(render_task_id)
Render Task Results

### Get the document or image produced by a completed render task.  Note that the PDF or image result will be a binary blob in the HTTP response, as indicated by the Content-Type in the response headers. This may require specialized (or at least different) handling than text responses such as JSON. You may need to tell your HTTP client that the response is binary so that it does not attempt to parse the binary data as text.  If the render task exists but has not finished rendering the results, the response HTTP status will be **202 Accepted**, the response body will be empty, and the response will have a Retry-After header indicating that the caller should repeat the request at a later time.  Returns 404 if the render task cannot be found, if the cached result has expired, or if the caller does not have permission to view the results.  For detailed information about the status of the render task, use [Render Task](#!/RenderTask/render_task). Polling loops waiting for completion of a render task would be better served by polling **render_task(id)** until the task status reaches completion (or error) instead of polling **render_task_results(id)** alone. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**render_task_id** | **String** | Id of render task | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/png, application/pdf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

