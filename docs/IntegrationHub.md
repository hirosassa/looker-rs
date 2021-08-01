# IntegrationHub

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**id** | Option<**i64**> | ID of the hub. | [optional][readonly]
**url** | Option<**String**> | URL of the hub. | [optional]
**label** | Option<**String**> | Label of the hub. | [optional][readonly]
**official** | Option<**bool**> | Whether this hub is a first-party integration hub operated by Looker. | [optional][readonly]
**fetch_error_message** | Option<**String**> | An error message, present if the integration hub metadata could not be fetched. If this is present, the integration hub is unusable. | [optional][readonly]
**authorization_token** | Option<**String**> | (Write-Only) An authorization key that will be sent to the integration hub on every request. | [optional]
**has_authorization_token** | Option<**bool**> | Whether the authorization_token is set for the hub. | [optional][readonly]
**legal_agreement_signed** | Option<**bool**> | Whether the legal agreement message has been signed by the user. This only matters if legal_agreement_required is true. | [optional][readonly]
**legal_agreement_required** | Option<**bool**> | Whether the legal terms for the integration hub are required before use. | [optional][readonly]
**legal_agreement_text** | Option<**String**> | The legal agreement text for this integration hub. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


