# LdapConfigTestResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**details** | Option<**String**> | Additional details for error cases | [optional][readonly]
**issues** | Option<[**Vec<crate::models::LdapConfigTestIssue>**](LDAPConfigTestIssue.md)> | Array of issues/considerations about the result | [optional][readonly]
**message** | Option<**String**> | Short human readable test about the result | [optional][readonly]
**status** | Option<**String**> | Test status code: always 'success' or 'error' | [optional][readonly]
**trace** | Option<**String**> | A more detailed trace of incremental results during auth tests | [optional][readonly]
**user** | Option<[**crate::models::LdapUser**](LDAPUser.md)> |  | [optional]
**url** | Option<**String**> | Link to ldap config | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


