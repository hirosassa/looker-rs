# GitBranch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can** | Option<**::std::collections::HashMap<String, bool>**> | Operations the current user is able to perform on this object | [optional][readonly]
**name** | Option<**String**> | The short name on the local. Updating `name` results in `git checkout <new_name>` | [optional]
**remote** | Option<**String**> | The name of the remote | [optional][readonly]
**remote_name** | Option<**String**> | The short name on the remote | [optional][readonly]
**error** | Option<**String**> | Name of error | [optional][readonly]
**message** | Option<**String**> | Message describing an error if present | [optional][readonly]
**owner_name** | Option<**String**> | Name of the owner of a personal branch | [optional][readonly]
**readonly** | Option<**bool**> | Whether or not this branch is readonly | [optional][readonly]
**personal** | Option<**bool**> | Whether or not this branch is a personal branch - readonly for all developers except the owner | [optional][readonly]
**is_local** | Option<**bool**> | Whether or not a local ref exists for the branch | [optional][readonly]
**is_remote** | Option<**bool**> | Whether or not a remote ref exists for the branch | [optional][readonly]
**is_production** | Option<**bool**> | Whether or not this is the production branch | [optional][readonly]
**ahead_count** | Option<**i64**> | Number of commits the local branch is ahead of the remote | [optional][readonly]
**behind_count** | Option<**i64**> | Number of commits the local branch is behind the remote | [optional][readonly]
**commit_at** | Option<**i64**> | UNIX timestamp at which this branch was last committed. | [optional][readonly]
**_ref** | Option<**String**> | The resolved ref of this branch. Updating `ref` results in `git reset --hard <new_ref>``. | [optional]
**remote_ref** | Option<**String**> | The resolved ref of this branch remote. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


