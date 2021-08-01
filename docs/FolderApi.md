# \FolderApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_folders**](FolderApi.md#all_folders) | **GET** /folders | Get All Folders
[**create_folder**](FolderApi.md#create_folder) | **POST** /folders | Create Folder
[**delete_folder**](FolderApi.md#delete_folder) | **DELETE** /folders/{folder_id} | Delete Folder
[**folder**](FolderApi.md#folder) | **GET** /folders/{folder_id} | Get Folder
[**folder_ancestors**](FolderApi.md#folder_ancestors) | **GET** /folders/{folder_id}/ancestors | Get Folder Ancestors
[**folder_children**](FolderApi.md#folder_children) | **GET** /folders/{folder_id}/children | Get Folder Children
[**folder_children_search**](FolderApi.md#folder_children_search) | **GET** /folders/{folder_id}/children/search | Search Folder Children
[**folder_dashboards**](FolderApi.md#folder_dashboards) | **GET** /folders/{folder_id}/dashboards | Get Folder Dashboards
[**folder_looks**](FolderApi.md#folder_looks) | **GET** /folders/{folder_id}/looks | Get Folder Looks
[**folder_parent**](FolderApi.md#folder_parent) | **GET** /folders/{folder_id}/parent | Get Folder Parent
[**search_folders**](FolderApi.md#search_folders) | **GET** /folders/search | Search Folders
[**update_folder**](FolderApi.md#update_folder) | **PATCH** /folders/{folder_id} | Update Folder



## all_folders

> Vec<crate::models::Folder> all_folders(fields)
Get All Folders

### Get information about all folders.  In API 3.x, this will not return empty personal folders, unless they belong to the calling user. In API 4.0+, all personal folders will be returned.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Folder>**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_folder

> crate::models::Folder create_folder(body)
Create Folder

### Create a folder with specified information.  Caller must have permission to edit the parent folder and to create folders, otherwise the request returns 404 Not Found. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateFolder**](CreateFolder.md) | Folder parameters | [required] |

### Return type

[**crate::models::Folder**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_folder

> String delete_folder(folder_id)
Delete Folder

### Delete the folder with a specific id including any children folders. **DANGER** this will delete all looks and dashboards in the folder. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Id of folder | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folder

> crate::models::Folder folder(folder_id, fields)
Get Folder

### Get information about the folder with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Id of folder | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Folder**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folder_ancestors

> Vec<crate::models::Folder> folder_ancestors(folder_id, fields)
Get Folder Ancestors

### Get the ancestors of a folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Id of folder | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Folder>**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folder_children

> Vec<crate::models::Folder> folder_children(folder_id, fields, page, per_page, sorts)
Get Folder Children

### Get the children of a folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Id of folder | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**page** | Option<**i64**> | Requested page. |  |
**per_page** | Option<**i64**> | Results per page. |  |
**sorts** | Option<**String**> | Fields to sort by. |  |

### Return type

[**Vec<crate::models::Folder>**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folder_children_search

> Vec<crate::models::Folder> folder_children_search(folder_id, fields, sorts, name)
Search Folder Children

### Search the children of a folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Id of folder | [required] |
**fields** | Option<**String**> | Requested fields. |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**name** | Option<**String**> | Match folder name. |  |

### Return type

[**Vec<crate::models::Folder>**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folder_dashboards

> Vec<crate::models::Dashboard> folder_dashboards(folder_id, fields)
Get Folder Dashboards

### Get the dashboards in a folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Id of folder | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::Dashboard>**](Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folder_looks

> Vec<crate::models::LookWithQuery> folder_looks(folder_id, fields)
Get Folder Looks

### Get all looks in a folder. In API 3.x, this will return all looks in a folder, including looks in the trash. In API 4.0+, all looks in a folder will be returned, excluding looks in the trash. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Id of folder | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::LookWithQuery>**](LookWithQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folder_parent

> crate::models::Folder folder_parent(folder_id, fields)
Get Folder Parent

### Get the parent of a folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Id of folder | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::Folder**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_folders

> Vec<crate::models::Folder> search_folders(fields, page, per_page, limit, offset, sorts, name, id, parent_id, creator_id, filter_or, is_shared_root)
Search Folders

Search for folders by creator id, parent id, name, etc

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |
**page** | Option<**i64**> | Requested page. |  |
**per_page** | Option<**i64**> | Results per page. |  |
**limit** | Option<**i64**> | Number of results to return. (used with offset and takes priority over page and per_page) |  |
**offset** | Option<**i64**> | Number of results to skip before returning any. (used with limit and takes priority over page and per_page) |  |
**sorts** | Option<**String**> | Fields to sort by. |  |
**name** | Option<**String**> | Match Space title. |  |
**id** | Option<**i64**> | Match Space id |  |
**parent_id** | Option<**String**> | Filter on a children of a particular folder. |  |
**creator_id** | Option<**String**> | Filter on folder created by a particular user. |  |
**filter_or** | Option<**bool**> | Combine given search criteria in a boolean OR expression |  |
**is_shared_root** | Option<**bool**> | Match is shared root |  |

### Return type

[**Vec<crate::models::Folder>**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_folder

> crate::models::Folder update_folder(folder_id, body)
Update Folder

### Update the folder with a specific id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Id of folder | [required] |
**body** | [**UpdateFolder**](UpdateFolder.md) | Folder parameters | [required] |

### Return type

[**crate::models::Folder**](Folder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

