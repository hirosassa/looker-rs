# \ProjectApi

All URIs are relative to *https://m3lookerdev.cloud.looker.com:443/api/3.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_git_branches**](ProjectApi.md#all_git_branches) | **GET** /projects/{project_id}/git_branches | Get All Git Branches
[**all_git_connection_tests**](ProjectApi.md#all_git_connection_tests) | **GET** /projects/{project_id}/git_connection_tests | Get All Git Connection Tests
[**all_lookml_tests**](ProjectApi.md#all_lookml_tests) | **GET** /projects/{project_id}/lookml_tests | Get All LookML Tests
[**all_project_files**](ProjectApi.md#all_project_files) | **GET** /projects/{project_id}/files | Get All Project Files
[**all_projects**](ProjectApi.md#all_projects) | **GET** /projects | Get All Projects
[**create_git_branch**](ProjectApi.md#create_git_branch) | **POST** /projects/{project_id}/git_branch | Checkout New Git Branch
[**create_git_deploy_key**](ProjectApi.md#create_git_deploy_key) | **POST** /projects/{project_id}/git/deploy_key | Create Deploy Key
[**create_project**](ProjectApi.md#create_project) | **POST** /projects | Create Project
[**delete_git_branch**](ProjectApi.md#delete_git_branch) | **DELETE** /projects/{project_id}/git_branch/{branch_name} | Delete a Git Branch
[**delete_repository_credential**](ProjectApi.md#delete_repository_credential) | **DELETE** /projects/{root_project_id}/credential/{credential_id} | Delete Repository Credential
[**deploy_ref_to_production**](ProjectApi.md#deploy_ref_to_production) | **POST** /projects/{project_id}/deploy_ref_to_production | Deploy Remote Branch or Ref to Production
[**deploy_to_production**](ProjectApi.md#deploy_to_production) | **POST** /projects/{project_id}/deploy_to_production | Deploy To Production
[**find_git_branch**](ProjectApi.md#find_git_branch) | **GET** /projects/{project_id}/git_branch/{branch_name} | Find a Git Branch
[**get_all_repository_credentials**](ProjectApi.md#get_all_repository_credentials) | **GET** /projects/{root_project_id}/credentials | Get All Repository Credentials
[**git_branch**](ProjectApi.md#git_branch) | **GET** /projects/{project_id}/git_branch | Get Active Git Branch
[**git_deploy_key**](ProjectApi.md#git_deploy_key) | **GET** /projects/{project_id}/git/deploy_key | Git Deploy Key
[**manifest**](ProjectApi.md#manifest) | **GET** /projects/{project_id}/manifest | Get Manifest
[**project**](ProjectApi.md#project) | **GET** /projects/{project_id} | Get Project
[**project_file**](ProjectApi.md#project_file) | **GET** /projects/{project_id}/files/file | Get Project File
[**project_validation_results**](ProjectApi.md#project_validation_results) | **GET** /projects/{project_id}/validate | Cached Project Validation Results
[**project_workspace**](ProjectApi.md#project_workspace) | **GET** /projects/{project_id}/current_workspace | Get Project Workspace
[**reset_project_to_production**](ProjectApi.md#reset_project_to_production) | **POST** /projects/{project_id}/reset_to_production | Reset To Production
[**reset_project_to_remote**](ProjectApi.md#reset_project_to_remote) | **POST** /projects/{project_id}/reset_to_remote | Reset To Remote
[**run_git_connection_test**](ProjectApi.md#run_git_connection_test) | **GET** /projects/{project_id}/git_connection_tests/{test_id} | Run Git Connection Test
[**run_lookml_test**](ProjectApi.md#run_lookml_test) | **GET** /projects/{project_id}/lookml_tests/run | Run LookML Test
[**tag_ref**](ProjectApi.md#tag_ref) | **POST** /projects/{project_id}/tag | Tag Ref
[**update_git_branch**](ProjectApi.md#update_git_branch) | **PUT** /projects/{project_id}/git_branch | Update Project Git Branch
[**update_project**](ProjectApi.md#update_project) | **PATCH** /projects/{project_id} | Update Project
[**update_repository_credential**](ProjectApi.md#update_repository_credential) | **PUT** /projects/{root_project_id}/credential/{credential_id} | Create Repository Credential
[**validate_project**](ProjectApi.md#validate_project) | **POST** /projects/{project_id}/validate | Validate Project



## all_git_branches

> Vec<crate::models::GitBranch> all_git_branches(project_id)
Get All Git Branches

### Get All Git Branches  Returns a list of git branches in the project repository 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |

### Return type

[**Vec<crate::models::GitBranch>**](GitBranch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_git_connection_tests

> Vec<crate::models::GitConnectionTest> all_git_connection_tests(project_id, remote_url)
Get All Git Connection Tests

### Get All Git Connection Tests  dev mode required.   - Call `update_session` to select the 'dev' workspace.  Returns a list of tests which can be run against a project's (or the dependency project for the provided remote_url) git connection. Call [Run Git Connection Test](#!/Project/run_git_connection_test) to execute each test in sequence.  Tests are ordered by increasing specificity. Tests should be run in the order returned because later tests require functionality tested by tests earlier in the test list.  For example, a late-stage test for write access is meaningless if connecting to the git server (an early test) is failing. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**remote_url** | Option<**String**> | (Optional: leave blank for root project) The remote url for remote dependency to test. |  |

### Return type

[**Vec<crate::models::GitConnectionTest>**](GitConnectionTest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_lookml_tests

> Vec<crate::models::LookmlTest> all_lookml_tests(project_id, file_id)
Get All LookML Tests

### Get All LookML Tests  Returns a list of tests which can be run to validate a project's LookML code and/or the underlying data, optionally filtered by the file id. Call [Run LookML Test](#!/Project/run_lookml_test) to execute tests. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**file_id** | Option<**String**> | File Id |  |

### Return type

[**Vec<crate::models::LookmlTest>**](LookmlTest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_project_files

> Vec<crate::models::ProjectFile> all_project_files(project_id, fields)
Get All Project Files

### Get All Project Files  Returns a list of the files in the project 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**fields** | Option<**String**> | Requested fields |  |

### Return type

[**Vec<crate::models::ProjectFile>**](ProjectFile.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_projects

> Vec<crate::models::Project> all_projects(fields)
Get All Projects

### Get All Projects  Returns all projects visible to the current user 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Requested fields |  |

### Return type

[**Vec<crate::models::Project>**](Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_git_branch

> crate::models::GitBranch create_git_branch(project_id, body)
Checkout New Git Branch

### Create and Checkout a Git Branch  Creates and checks out a new branch in the given project repository Only allowed in development mode   - Call `update_session` to select the 'dev' workspace.  Optionally specify a branch name, tag name or commit SHA as the start point in the ref field.   If no ref is specified, HEAD of the current branch will be used as the start point for the new branch.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**body** | [**GitBranch**](GitBranch.md) | Git Branch | [required] |

### Return type

[**crate::models::GitBranch**](GitBranch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_git_deploy_key

> String create_git_deploy_key(project_id)
Create Deploy Key

### Create Git Deploy Key  Create a public/private key pair for authenticating ssh git requests from Looker to a remote git repository for a particular Looker project.  Returns the public key of the generated ssh key pair.  Copy this public key to your remote git repository's ssh keys configuration so that the remote git service can validate and accept git requests from the Looker server. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project

> crate::models::Project create_project(body)
Create Project

### Create A Project  dev mode required. - Call `update_session` to select the 'dev' workspace.  `name` is required. `git_remote_url` is not allowed. To configure Git for the newly created project, follow the instructions in `update_project`.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Project**](Project.md) | Project | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_git_branch

> String delete_git_branch(project_id, branch_name)
Delete a Git Branch

### Delete the specified Git Branch  Delete git branch specified in branch_name path param from local and remote of specified project repository 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**branch_name** | **String** | Branch Name | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_repository_credential

> String delete_repository_credential(root_project_id, credential_id)
Delete Repository Credential

### Repository Credential for a remote dependency  Admin required.  `root_project_id` is required. `credential_id` is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_project_id** | **String** | Root Project Id | [required] |
**credential_id** | **String** | Credential Id | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deploy_ref_to_production

> String deploy_ref_to_production(project_id, branch, _ref)
Deploy Remote Branch or Ref to Production

### Deploy a Remote Branch or Ref to Production  Git must have been configured and deploy permission required.  Deploy is a one/two step process 1. If this is the first deploy of this project, create the production project with git repository. 2. Pull the branch or ref into the production project.  Can only specify either a branch or a ref.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Id of project | [required] |
**branch** | Option<**String**> | Branch to deploy to production |  |
**_ref** | Option<**String**> | Ref to deploy to production |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deploy_to_production

> String deploy_to_production(project_id)
Deploy To Production

### Deploy LookML from this Development Mode Project to Production  Git must have been configured, must be in dev mode and deploy permission required  Deploy is a two / three step process:  1. Push commits in current branch of dev mode project to the production branch (origin/master).    Note a. This step is skipped in read-only projects.    Note b. If this step is unsuccessful for any reason (e.g. rejected non-fastforward because production branch has              commits not in current branch), subsequent steps will be skipped. 2. If this is the first deploy of this project, create the production project with git repository. 3. Pull the production branch into the production project.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Id of project | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_git_branch

> crate::models::GitBranch find_git_branch(project_id, branch_name)
Find a Git Branch

### Get the specified Git Branch  Returns the git branch specified in branch_name path param if it exists in the given project repository 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**branch_name** | **String** | Branch Name | [required] |

### Return type

[**crate::models::GitBranch**](GitBranch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_repository_credentials

> Vec<crate::models::RepositoryCredential> get_all_repository_credentials(root_project_id)
Get All Repository Credentials

### Get all Repository Credentials for a project  `root_project_id` is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_project_id** | **String** | Root Project Id | [required] |

### Return type

[**Vec<crate::models::RepositoryCredential>**](RepositoryCredential.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## git_branch

> crate::models::GitBranch git_branch(project_id)
Get Active Git Branch

### Get the Current Git Branch  Returns the git branch currently checked out in the given project repository 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |

### Return type

[**crate::models::GitBranch**](GitBranch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## git_deploy_key

> String git_deploy_key(project_id)
Git Deploy Key

### Git Deploy Key  Returns the ssh public key previously created for a project's git repository. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manifest

> crate::models::Manifest manifest(project_id)
Get Manifest

### Get A Projects Manifest object  Returns the project with the given project id 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |

### Return type

[**crate::models::Manifest**](Manifest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project

> crate::models::Project project(project_id, fields)
Get Project

### Get A Project  Returns the project with the given project id 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**fields** | Option<**String**> | Requested fields |  |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_file

> crate::models::ProjectFile project_file(project_id, file_id, fields)
Get Project File

### Get Project File Info  Returns information about a file in the project 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**file_id** | **String** | File Id | [required] |
**fields** | Option<**String**> | Requested fields |  |

### Return type

[**crate::models::ProjectFile**](ProjectFile.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_validation_results

> crate::models::ProjectValidationCache project_validation_results(project_id, fields)
Cached Project Validation Results

### Get Cached Project Validation Results  Returns the cached results of a previous project validation calculation, if any. Returns http status 204 No Content if no validation results exist.  Validating the content of all the files in a project can be computationally intensive for large projects. Use this API to simply fetch the results of the most recent project validation rather than revalidating the entire project from scratch.  A value of `\"stale\": true` in the response indicates that the project has changed since the cached validation results were computed. The cached validation results may no longer reflect the current state of the project. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**fields** | Option<**String**> | Requested fields |  |

### Return type

[**crate::models::ProjectValidationCache**](ProjectValidationCache.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_workspace

> crate::models::ProjectWorkspace project_workspace(project_id, fields)
Get Project Workspace

### Get Project Workspace  Returns information about the state of the project files in the currently selected workspace 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**fields** | Option<**String**> | Requested fields |  |

### Return type

[**crate::models::ProjectWorkspace**](ProjectWorkspace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_project_to_production

> String reset_project_to_production(project_id)
Reset To Production

### Reset a project to the revision of the project that is in production.  **DANGER** this will delete any changes that have not been pushed to a remote repository. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Id of project | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_project_to_remote

> String reset_project_to_remote(project_id)
Reset To Remote

### Reset a project development branch to the revision of the project that is on the remote.  **DANGER** this will delete any changes that have not been pushed to a remote repository. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Id of project | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_git_connection_test

> crate::models::GitConnectionTestResult run_git_connection_test(project_id, test_id, remote_url, use_production)
Run Git Connection Test

### Run a git connection test  Run the named test on the git service used by this project (or the dependency project for the provided remote_url) and return the result. This is intended to help debug git connections when things do not work properly, to give more helpful information about why a git url is not working with Looker.  Tests should be run in the order they are returned by [Get All Git Connection Tests](#!/Project/all_git_connection_tests). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**test_id** | **String** | Test Id | [required] |
**remote_url** | Option<**String**> | (Optional: leave blank for root project) The remote url for remote dependency to test. |  |
**use_production** | Option<**String**> | (Optional: leave blank for dev credentials) Whether to use git production credentials. |  |

### Return type

[**crate::models::GitConnectionTestResult**](GitConnectionTestResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_lookml_test

> Vec<crate::models::LookmlTestResult> run_lookml_test(project_id, file_id, test, model)
Run LookML Test

### Run LookML Tests  Runs all tests in the project, optionally filtered by file, test, and/or model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**file_id** | Option<**String**> | File Name |  |
**test** | Option<**String**> | Test Name |  |
**model** | Option<**String**> | Model Name |  |

### Return type

[**Vec<crate::models::LookmlTestResult>**](LookmlTestResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tag_ref

> crate::models::Project tag_ref(project_id, body, commit_sha, tag_name, tag_message)
Tag Ref

### Creates a tag for the most recent commit, or a specific ref is a SHA is provided  This is an internal-only, undocumented route. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**body** | [**Project**](Project.md) | Project | [required] |
**commit_sha** | Option<**String**> | (Optional): Commit Sha to Tag |  |
**tag_name** | Option<**String**> | Tag Name |  |
**tag_message** | Option<**String**> | (Optional): Tag Message |  |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_git_branch

> crate::models::GitBranch update_git_branch(project_id, body)
Update Project Git Branch

### Checkout and/or reset --hard an existing Git Branch  Only allowed in development mode   - Call `update_session` to select the 'dev' workspace.  Checkout an existing branch if name field is different from the name of the currently checked out branch.  Optionally specify a branch name, tag name or commit SHA to which the branch should be reset.   **DANGER** hard reset will be force pushed to the remote. Unsaved changes and commits may be permanently lost.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**body** | [**GitBranch**](GitBranch.md) | Git Branch | [required] |

### Return type

[**crate::models::GitBranch**](GitBranch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> crate::models::Project update_project(project_id, body, fields)
Update Project

### Update Project Configuration  Apply changes to a project's configuration.   #### Configuring Git for a Project  To set up a Looker project with a remote git repository, follow these steps:  1. Call `update_session` to select the 'dev' workspace. 1. Call `create_git_deploy_key` to create a new deploy key for the project 1. Copy the deploy key text into the remote git repository's ssh key configuration 1. Call `update_project` to set project's `git_remote_url` ()and `git_service_name`, if necessary).  When you modify a project's `git_remote_url`, Looker connects to the remote repository to fetch metadata. The remote git repository MUST be configured with the Looker-generated deploy key for this project prior to setting the project's `git_remote_url`.  To set up a Looker project with a git repository residing on the Looker server (a 'bare' git repo):  1. Call `update_session` to select the 'dev' workspace. 1. Call `update_project` setting `git_remote_url` to null and `git_service_name` to \"bare\".  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**body** | [**Project**](Project.md) | Project | [required] |
**fields** | Option<**String**> | Requested fields |  |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository_credential

> crate::models::RepositoryCredential update_repository_credential(root_project_id, credential_id, body)
Create Repository Credential

### Configure Repository Credential for a remote dependency  Admin required.  `root_project_id` is required. `credential_id` is required.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_project_id** | **String** | Root Project Id | [required] |
**credential_id** | **String** | Credential Id | [required] |
**body** | [**RepositoryCredential**](RepositoryCredential.md) | Remote Project Information | [required] |

### Return type

[**crate::models::RepositoryCredential**](RepositoryCredential.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_project

> crate::models::ProjectValidation validate_project(project_id, fields)
Validate Project

### Validate Project  Performs lint validation of all lookml files in the project. Returns a list of errors found, if any.  Validating the content of all the files in a project can be computationally intensive for large projects. For best performance, call `validate_project(project_id)` only when you really want to recompute project validation. To quickly display the results of the most recent project validation (without recomputing), use `project_validation_results(project_id)` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project Id | [required] |
**fields** | Option<**String**> | Requested fields |  |

### Return type

[**crate::models::ProjectValidation**](ProjectValidation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

