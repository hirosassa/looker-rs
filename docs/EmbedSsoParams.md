# EmbedSsoParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target_url** | **String** | The complete URL of the Looker UI page to display in the embed context. For example, to display the dashboard with id 34, `target_url` would look like: `https://mycompany.looker.com:9999/dashboards/34`. `target_uri` MUST contain a scheme (HTTPS), domain name, and URL path. Port must be included if it is required to reach the Looker server from browser clients. If the Looker instance is behind a load balancer or other proxy, `target_uri` must be the public-facing domain name and port required to reach the Looker instance, not the actual internal network machine name of the Looker instance. | 
**session_length** | Option<**i64**> | Number of seconds the SSO embed session will be valid after the embed session is started. Defaults to 300 seconds. Maximum session length accepted is 2592000 seconds (30 days). | [optional]
**force_logout_login** | Option<**bool**> | When true, the embed session will purge any residual Looker login state (such as in browser cookies) before creating a new login state with the given embed user info. Defaults to true. | [optional]
**external_user_id** | Option<**String**> | A value from an external system that uniquely identifies the embed user. Since the user_ids of Looker embed users may change with every embed session, external_user_id provides a way to assign a known, stable user identifier across multiple embed sessions. | [optional]
**first_name** | Option<**String**> | First name of the embed user. Defaults to 'Embed' if not specified | [optional]
**last_name** | Option<**String**> | Last name of the embed user. Defaults to 'User' if not specified | [optional]
**user_timezone** | Option<**String**> | Sets the user timezone for the embed user session, if the User Specific Timezones setting is enabled in the Looker admin settings. A value of `null` forces the embed user to use the Looker Application Default Timezone. You MUST omit this property from the request if the User Specific Timezones setting is disabled. Timezone values are validated against the IANA Timezone standard and can be seen in the Application Time Zone dropdown list on the Looker General Settings admin page. | [optional]
**permissions** | Option<**Vec<String>**> | List of Looker permission names to grant to the embed user. Requested permissions will be filtered to permissions allowed for embed sessions. | [optional]
**models** | Option<**Vec<String>**> | List of model names that the embed user may access | [optional]
**group_ids** | Option<**Vec<i64>**> | List of Looker group ids in which to enroll the embed user | [optional]
**external_group_id** | Option<**String**> | A unique value identifying an embed-exclusive group. Multiple embed users using the same `external_group_id` value will be able to share Looker content with each other. Content and embed users associated with the `external_group_id` will not be accessible to normal Looker users or embed users not associated with this `external_group_id`. | [optional]
**user_attributes** | Option<**::std::collections::HashMap<String, String>**> | A dictionary of name-value pairs associating a Looker user attribute name with a value. | [optional]
**secret_id** | Option<**i64**> | Id of the embed secret to use to sign this SSO url. If specified, the value must be an id of a valid (active) secret defined in the Looker instance. If not specified, the URL will be signed with the newest active embed secret defined in the Looker instance. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


