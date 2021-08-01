# \ConnectionApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_connections**](ConnectionApi.md#all_connections) | **GET** /connections | Get All Connections
[**all_dialect_infos**](ConnectionApi.md#all_dialect_infos) | **GET** /dialect_info | Get All Dialect Infos
[**connection**](ConnectionApi.md#connection) | **GET** /connections/{connection_name} | Get Connection
[**create_connection**](ConnectionApi.md#create_connection) | **POST** /connections | Create Connection
[**delete_connection**](ConnectionApi.md#delete_connection) | **DELETE** /connections/{connection_name} | Delete Connection
[**delete_connection_override**](ConnectionApi.md#delete_connection_override) | **DELETE** /connections/{connection_name}/connection_override/{override_context} | Delete Connection Override
[**test_connection**](ConnectionApi.md#test_connection) | **PUT** /connections/{connection_name}/test | Test Connection
[**test_connection_config**](ConnectionApi.md#test_connection_config) | **PUT** /connections/test | Test Connection Configuration
[**update_connection**](ConnectionApi.md#update_connection) | **PATCH** /connections/{connection_name} | Update Connection



## all_connections

> Vec<crate::models::DbConnection> all_connections(fields)
Get All Connections

### Get information about all connections. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::DbConnection>**](DBConnection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_dialect_infos

> Vec<crate::models::DialectInfo> all_dialect_infos(fields)
Get All Dialect Infos

### Get information about all dialects. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**Vec<crate::models::DialectInfo>**](DialectInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connection

> crate::models::DbConnection connection(connection_name, fields)
Get Connection

### Get information about a connection. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_name** | **String** | Name of connection | [required] |
**fields** | Option<**String**> | Requested fields. |  |

### Return type

[**crate::models::DbConnection**](DBConnection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_connection

> crate::models::DbConnection create_connection(body)
Create Connection

### Create a connection using the specified configuration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DbConnection**](DbConnection.md) | Connection | [required] |

### Return type

[**crate::models::DbConnection**](DBConnection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connection

> String delete_connection(connection_name)
Delete Connection

### Delete a connection. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_name** | **String** | Name of connection | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connection_override

> String delete_connection_override(connection_name, override_context)
Delete Connection Override

### Delete a connection override. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_name** | **String** | Name of connection | [required] |
**override_context** | **String** | Context of connection override | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_connection

> Vec<crate::models::DbConnectionTestResult> test_connection(connection_name, tests)
Test Connection

### Test an existing connection.  Note that a connection's 'dialect' property has a 'connection_tests' property that lists the specific types of tests that the connection supports.  This API is rate limited.  Unsupported tests in the request will be ignored. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_name** | **String** | Name of connection | [required] |
**tests** | Option<[**Vec<String>**](String.md)> | Array of names of tests to run |  |

### Return type

[**Vec<crate::models::DbConnectionTestResult>**](DBConnectionTestResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_connection_config

> Vec<crate::models::DbConnectionTestResult> test_connection_config(body, tests)
Test Connection Configuration

### Test a connection configuration.  Note that a connection's 'dialect' property has a 'connection_tests' property that lists the specific types of tests that the connection supports.  This API is rate limited.  Unsupported tests in the request will be ignored. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DbConnection**](DbConnection.md) | Connection | [required] |
**tests** | Option<[**Vec<String>**](String.md)> | Array of names of tests to run |  |

### Return type

[**Vec<crate::models::DbConnectionTestResult>**](DBConnectionTestResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_connection

> crate::models::DbConnection update_connection(connection_name, body)
Update Connection

### Update a connection using the specified configuration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_name** | **String** | Name of connection | [required] |
**body** | [**DbConnection**](DbConnection.md) | Connection | [required] |

### Return type

[**crate::models::DbConnection**](DBConnection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

