/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  The Configuration API is an internal Airbyte API that is designed for communications between different Airbyte components. * Its main purpose is to enable the Airbyte Engineering team to configure the internal state of [Airbyte Cloud](https://airbyte.com/airbyte-cloud) * It is also sometimes used by OSS users to configure their own Self-Hosted Airbyte deployment (internal state, etc)  WARNING * Airbyte does NOT have active commitments to support this API long-term. * OSS users can utilize the Configuration API, but at their own risk. * This API is utilized internally by the Airbyte Engineering team and may be modified in the future if the need arises. * Modifications by the Airbyte Engineering team could create breaking changes and OSS users would need to update their code to catch up to any backwards incompatible changes in the API.  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/api/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/api/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Authentication (OSS): * When authenticating to the Configuration API, you must use Basic Authentication by setting the Authentication Header to Basic and base64 encoding the username and password (which are `airbyte` and `password` by default - so base64 encoding `airbyte:password` results in `YWlyYnl0ZTpwYXNzd29yZA==`). So the full header reads `'Authorization': \"Basic YWlyYnl0ZTpwYXNzd29yZA==\"`
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`create_connector_builder_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateConnectorBuilderProjectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_connector_builder_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteConnectorBuilderProjectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_connector_builder_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetConnectorBuilderProjectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_connector_builder_projects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListConnectorBuilderProjectsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`publish_connector_builder_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublishConnectorBuilderProjectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_connector_builder_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateConnectorBuilderProjectError {
    UnknownValue(serde_json::Value),
}

pub async fn create_connector_builder_project(
    configuration: &configuration::Configuration,
    connector_builder_project_with_workspace_id: crate::models::ConnectorBuilderProjectWithWorkspaceId,
) -> Result<
    crate::models::ConnectorBuilderProjectIdWithWorkspaceId,
    Error<CreateConnectorBuilderProjectError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/connector_builder_projects/create",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.json(&connector_builder_project_with_workspace_id);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateConnectorBuilderProjectError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_connector_builder_project(
    configuration: &configuration::Configuration,
    connector_builder_project_id_with_workspace_id: crate::models::ConnectorBuilderProjectIdWithWorkspaceId,
) -> Result<(), Error<DeleteConnectorBuilderProjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/connector_builder_projects/delete",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.json(&connector_builder_project_id_with_workspace_id);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteConnectorBuilderProjectError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_connector_builder_project(
    configuration: &configuration::Configuration,
    connector_builder_project_id_with_workspace_id: crate::models::ConnectorBuilderProjectIdWithWorkspaceId,
) -> Result<crate::models::ConnectorBuilderProjectRead, Error<GetConnectorBuilderProjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/connector_builder_projects/get_with_manifest",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.json(&connector_builder_project_id_with_workspace_id);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetConnectorBuilderProjectError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_connector_builder_projects(
    configuration: &configuration::Configuration,
    workspace_id_request_body: crate::models::WorkspaceIdRequestBody,
) -> Result<crate::models::ConnectorBuilderProjectReadList, Error<ListConnectorBuilderProjectsError>>
{
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/connector_builder_projects/list",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&workspace_id_request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListConnectorBuilderProjectsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn publish_connector_builder_project(
    configuration: &configuration::Configuration,
    connector_builder_publish_request_body: crate::models::ConnectorBuilderPublishRequestBody,
) -> Result<crate::models::SourceDefinitionIdBody, Error<PublishConnectorBuilderProjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/connector_builder_projects/publish",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&connector_builder_publish_request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PublishConnectorBuilderProjectError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_connector_builder_project(
    configuration: &configuration::Configuration,
    existing_connector_builder_project_with_workspace_id: crate::models::ExistingConnectorBuilderProjectWithWorkspaceId,
) -> Result<(), Error<UpdateConnectorBuilderProjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/connector_builder_projects/update",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.json(&existing_connector_builder_project_with_workspace_id);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateConnectorBuilderProjectError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}