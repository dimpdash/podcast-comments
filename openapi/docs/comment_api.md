# comment_api

All URIs are relative to *http://127.0.0.1:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
**addComment**](comment_api.md#addComment) | **POST** /comment | Add a new comment to the podcast
**getComment**](comment_api.md#getComment) | **GET** /comment | Get all comments


# **addComment**
> models::Comment addComment(comment)
Add a new comment to the podcast



### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **comment** | [**Comment**](Comment.md)| Comment object that needs to be added to the podcast | 

### Return type

[**models::Comment**](Comment.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getComment**
> models::Comment getComment()
Get all comments



### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::Comment**](Comment.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

