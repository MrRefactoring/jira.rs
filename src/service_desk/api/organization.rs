// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Organization operations.
pub struct OrganizationService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> OrganizationService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// This method returns a list of organizations in the Jira Service Management instance. Use this method when you want to present a list of organizations or want to locate an organization by name.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Any. However, to fetch organizations based on `accountId` the user must have a Service Desk agent license.
    ///
    /// **Response limitations**: If the user is a customer, only those organizations of which the customer is a member are listed.
    pub fn get_organizations(&self) -> GetOrganizationsRequest<'a> {
        GetOrganizationsRequest::new(self.client)
    }

    /// This method creates an organization by passing the name of the organization.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk administrator or agent. Note: Permission to create organizations can be switched to users with the Jira administrator permission, using the **[Organization management](https://confluence.atlassian.com/servicedeskcloud/setting-up-service-desk-users-732528877.html#Settingupservicedeskusers-manageorgsManageorganizations)** feature.
    pub fn create_organization(&self, organization_create: OrganizationCreate) -> CreateOrganizationRequest<'a> {
        CreateOrganizationRequest::new(self.client, organization_create)
    }

    /// This method returns details of an organization. Use this method to get organization details whenever your application component is passed an organization ID but needs to display other organization details.
    ///
    /// To get organization detail field values which are visible in Jira Service Management, see the [Customer Service Management REST API](https://developer.atlassian.com/cloud/customer-service-management/rest/v1/api-group-organization/#api-group-organization).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Any
    ///
    /// **Response limitations**: Customers can only retrieve organization of which they are members.
    pub fn get_organization(&self, organization_id: i64) -> GetOrganizationRequest<'a> {
        GetOrganizationRequest::new(self.client, organization_id)
    }

    /// This method deletes an organization. Note that the organization is deleted regardless of other associations it may have. For example, associations with service desks.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Jira administrator.
    pub fn delete_organization(&self, organization_id: i64) -> DeleteOrganizationRequest<'a> {
        DeleteOrganizationRequest::new(self.client, organization_id)
    }

    /// Returns the keys of all organization properties. Organization properties are a type of entity property which are available to the API only, and not shown in Jira Service Management. [Learn more](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/).
    ///
    /// To get organization detail field values which are visible in Jira Service Management, see the [Customer Service Management REST API](https://developer.atlassian.com/cloud/customer-service-management/rest/v1/api-group-organization/#api-group-organization).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Any
    ///
    /// **Response limitations**: Customers can only access properties of organizations of which they are members.
    pub fn get_properties_keys(&self, organization_id: impl Into<String>) -> GetPropertiesKeysRequest<'a> {
        GetPropertiesKeysRequest::new(self.client, organization_id)
    }

    /// Returns the value of an organization property. Use this method to obtain the JSON content for an organization's property. Organization properties are a type of entity property which are available to the API only, and not shown in Jira Service Management. [Learn more](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/).
    ///
    /// To get organization detail field values which are visible in Jira Service Management, see the [Customer Service Management REST API](https://developer.atlassian.com/cloud/customer-service-management/rest/v1/api-group-organization/#api-group-organization).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Any
    ///
    /// **Response limitations**: Customers can only access properties of organizations of which they are members.
    pub fn get_property(
        &self,
        organization_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> GetPropertyRequest<'a> {
        GetPropertyRequest::new(self.client, organization_id, property_key)
    }

    /// Sets the value of an organization property. Use this resource to store custom data against an organization. Organization properties are a type of entity property which are available to the API only, and not shown in Jira Service Management. [Learn more](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/).
    ///
    /// To store organization detail field values which are visible in Jira Service Management, see the [Customer Service Management REST API](https://developer.atlassian.com/cloud/customer-service-management/rest/v1/api-group-organization/#api-group-organization).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service Desk Administrator or Agent.
    ///
    /// Note: Permission to manage organizations can be switched to users with the Jira administrator permission, using the **[Organization management](https://confluence.atlassian.com/servicedeskcloud/setting-up-service-desk-users-732528877.html#Settingupservicedeskusers-manageorgsManageorganizations)** feature.
    pub fn set_property(
        &self,
        organization_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetPropertyRequest<'a> {
        SetPropertyRequest::new(self.client, organization_id, property_key, body)
    }

    /// Removes an organization property. Organization properties are a type of entity property which are available to the API only, and not shown in Jira Service Management. [Learn more](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/).
    ///
    /// For operations relating to organization detail field values which are visible in Jira Service Management, see the [Customer Service Management REST API](https://developer.atlassian.com/cloud/customer-service-management/rest/v1/api-group-organization/#api-group-organization).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service Desk Administrator or Agent.
    ///
    /// Note: Permission to manage organizations can be switched to users with the Jira administrator permission, using the **[Organization management](https://confluence.atlassian.com/servicedeskcloud/setting-up-service-desk-users-732528877.html#Settingupservicedeskusers-manageorgsManageorganizations)** feature.
    pub fn delete_property(
        &self,
        organization_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> DeletePropertyRequest<'a> {
        DeletePropertyRequest::new(self.client, organization_id, property_key)
    }

    /// This method returns all the users associated with an organization. Use this method where you want to provide a list of users for an organization or determine if a user is associated with an organization.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk administrator or agent.
    pub fn get_users_in_organization(&self, organization_id: i64) -> GetUsersInOrganizationRequest<'a> {
        GetUsersInOrganizationRequest::new(self.client, organization_id)
    }

    /// This method adds users to an organization.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk administrator or agent. Note: Permission to add users to an organization can be switched to users with the Jira administrator permission, using the **[Organization management](https://confluence.atlassian.com/servicedeskcloud/setting-up-service-desk-users-732528877.html#Settingupservicedeskusers-manageorgsManageorganizations)** feature.
    pub fn add_users_to_organization(
        &self,
        organization_id: i64,
        body: UsersOrganizationUpdate,
    ) -> AddUsersToOrganizationRequest<'a> {
        AddUsersToOrganizationRequest::new(self.client, organization_id, body)
    }

    /// This method removes users from an organization.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk administrator or agent. Note: Permission to delete users from an organization can be switched to users with the Jira administrator permission, using the **[Organization management](https://confluence.atlassian.com/servicedeskcloud/setting-up-service-desk-users-732528877.html#Settingupservicedeskusers-manageorgsManageorganizations)** feature.
    pub fn remove_users_from_organization(
        &self,
        organization_id: i64,
        body: UsersOrganizationUpdate,
    ) -> RemoveUsersFromOrganizationRequest<'a> {
        RemoveUsersFromOrganizationRequest::new(self.client, organization_id, body)
    }

    /// This method returns a list of all organizations associated with a service desk.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk's agent.
    pub fn get_service_desk_organizations(
        &self,
        service_desk_id: impl Into<String>,
    ) -> GetServiceDeskOrganizationsRequest<'a> {
        GetServiceDeskOrganizationsRequest::new(self.client, service_desk_id)
    }

    /// This method adds an organization to a service desk. If the organization ID is already associated with the service desk, no change is made and the resource returns a 204 success code.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk's agent.
    pub fn add_organization(
        &self,
        service_desk_id: impl Into<String>,
        body: OrganizationServiceDeskUpdate,
    ) -> AddOrganizationRequest<'a> {
        AddOrganizationRequest::new(self.client, service_desk_id, body)
    }

    /// This method removes an organization from a service desk. If the organization ID does not match an organization associated with the service desk, no change is made and the resource returns a 204 success code.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk's agent.
    pub fn remove_organization(
        &self,
        service_desk_id: impl Into<String>,
        body: OrganizationServiceDeskUpdate,
    ) -> RemoveOrganizationRequest<'a> {
        RemoveOrganizationRequest::new(self.client, service_desk_id, body)
    }
}

/// This method returns a list of organizations in the Jira Service Management instance. Use this method when you want to present a list of organizations or want to locate an organization by name.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Any. However, to fetch organizations based on `accountId` the user must have a Service Desk agent license.
///
/// **Response limitations**: If the user is a customer, only those organizations of which the customer is a member are listed.
pub struct GetOrganizationsRequest<'a> {
    client: &'a crate::core::Client,
    start: Option<i64>,
    limit: Option<i64>,
    account_id: Option<String>,
}

impl<'a> GetOrganizationsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start: None, limit: None, account_id: None }
    }

    /// The starting index of the returned objects. Base index: 0. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of organizations to return per page. Default: 50. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/servicedeskapi/organization".to_owned());

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Organization>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method creates an organization by passing the name of the organization.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk administrator or agent. Note: Permission to create organizations can be switched to users with the Jira administrator permission, using the **[Organization management](https://confluence.atlassian.com/servicedeskcloud/setting-up-service-desk-users-732528877.html#Settingupservicedeskusers-manageorgsManageorganizations)** feature.
pub struct CreateOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_create: OrganizationCreate,
}

impl<'a> CreateOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_create: OrganizationCreate) -> Self {
        Self { client, organization_create }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/servicedeskapi/organization".to_owned());

        let body = match serde_json::to_value(&self.organization_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Organization> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method returns details of an organization. Use this method to get organization details whenever your application component is passed an organization ID but needs to display other organization details.
///
/// To get organization detail field values which are visible in Jira Service Management, see the [Customer Service Management REST API](https://developer.atlassian.com/cloud/customer-service-management/rest/v1/api-group-organization/#api-group-organization).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Any
///
/// **Response limitations**: Customers can only retrieve organization of which they are members.
pub struct GetOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: i64,
}

impl<'a> GetOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_id: i64) -> Self {
        Self { client, organization_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/servicedeskapi/organization/{}", self.organization_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Organization> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method deletes an organization. Note that the organization is deleted regardless of other associations it may have. For example, associations with service desks.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Jira administrator.
pub struct DeleteOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: i64,
}

impl<'a> DeleteOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_id: i64) -> Self {
        Self { client, organization_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/servicedeskapi/organization/{}", self.organization_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the keys of all organization properties. Organization properties are a type of entity property which are available to the API only, and not shown in Jira Service Management. [Learn more](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/).
///
/// To get organization detail field values which are visible in Jira Service Management, see the [Customer Service Management REST API](https://developer.atlassian.com/cloud/customer-service-management/rest/v1/api-group-organization/#api-group-organization).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Any
///
/// **Response limitations**: Customers can only access properties of organizations of which they are members.
pub struct GetPropertiesKeysRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: String,
}

impl<'a> GetPropertiesKeysRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_id: impl Into<String>) -> Self {
        Self { client, organization_id: organization_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/organization/{}/property",
                crate::core::encode_path_segment(&self.organization_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PropertyKeys> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the value of an organization property. Use this method to obtain the JSON content for an organization's property. Organization properties are a type of entity property which are available to the API only, and not shown in Jira Service Management. [Learn more](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/).
///
/// To get organization detail field values which are visible in Jira Service Management, see the [Customer Service Management REST API](https://developer.atlassian.com/cloud/customer-service-management/rest/v1/api-group-organization/#api-group-organization).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Any
///
/// **Response limitations**: Customers can only access properties of organizations of which they are members.
pub struct GetPropertyRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: String,
    property_key: String,
}

impl<'a> GetPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        organization_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> Self {
        Self { client, organization_id: organization_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/organization/{}/property/{}",
                crate::core::encode_path_segment(&self.organization_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EntityProperty> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the value of an organization property. Use this resource to store custom data against an organization. Organization properties are a type of entity property which are available to the API only, and not shown in Jira Service Management. [Learn more](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/).
///
/// To store organization detail field values which are visible in Jira Service Management, see the [Customer Service Management REST API](https://developer.atlassian.com/cloud/customer-service-management/rest/v1/api-group-organization/#api-group-organization).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service Desk Administrator or Agent.
///
/// Note: Permission to manage organizations can be switched to users with the Jira administrator permission, using the **[Organization management](https://confluence.atlassian.com/servicedeskcloud/setting-up-service-desk-users-732528877.html#Settingupservicedeskusers-manageorgsManageorganizations)** feature.
pub struct SetPropertyRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: String,
    property_key: String,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        organization_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, organization_id: organization_id.into(), property_key: property_key.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/servicedeskapi/organization/{}/property/{}",
                crate::core::encode_path_segment(&self.organization_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Removes an organization property. Organization properties are a type of entity property which are available to the API only, and not shown in Jira Service Management. [Learn more](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/).
///
/// For operations relating to organization detail field values which are visible in Jira Service Management, see the [Customer Service Management REST API](https://developer.atlassian.com/cloud/customer-service-management/rest/v1/api-group-organization/#api-group-organization).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service Desk Administrator or Agent.
///
/// Note: Permission to manage organizations can be switched to users with the Jira administrator permission, using the **[Organization management](https://confluence.atlassian.com/servicedeskcloud/setting-up-service-desk-users-732528877.html#Settingupservicedeskusers-manageorgsManageorganizations)** feature.
pub struct DeletePropertyRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: String,
    property_key: String,
}

impl<'a> DeletePropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        organization_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> Self {
        Self { client, organization_id: organization_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/servicedeskapi/organization/{}/property/{}",
                crate::core::encode_path_segment(&self.organization_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method returns all the users associated with an organization. Use this method where you want to provide a list of users for an organization or determine if a user is associated with an organization.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk administrator or agent.
pub struct GetUsersInOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: i64,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetUsersInOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_id: i64) -> Self {
        Self { client, organization_id, start: None, limit: None }
    }

    /// The starting index of the returned objects. Base index: 0. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of users to return per page. Default: 50. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/servicedeskapi/organization/{}/user", self.organization_id),
        );

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<User>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method adds users to an organization.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk administrator or agent. Note: Permission to add users to an organization can be switched to users with the Jira administrator permission, using the **[Organization management](https://confluence.atlassian.com/servicedeskcloud/setting-up-service-desk-users-732528877.html#Settingupservicedeskusers-manageorgsManageorganizations)** feature.
pub struct AddUsersToOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: i64,
    body: UsersOrganizationUpdate,
}

impl<'a> AddUsersToOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_id: i64, body: UsersOrganizationUpdate) -> Self {
        Self { client, organization_id, body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/servicedeskapi/organization/{}/user", self.organization_id),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method removes users from an organization.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk administrator or agent. Note: Permission to delete users from an organization can be switched to users with the Jira administrator permission, using the **[Organization management](https://confluence.atlassian.com/servicedeskcloud/setting-up-service-desk-users-732528877.html#Settingupservicedeskusers-manageorgsManageorganizations)** feature.
pub struct RemoveUsersFromOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: i64,
    body: UsersOrganizationUpdate,
}

impl<'a> RemoveUsersFromOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_id: i64, body: UsersOrganizationUpdate) -> Self {
        Self { client, organization_id, body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/servicedeskapi/organization/{}/user", self.organization_id),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method returns a list of all organizations associated with a service desk.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk's agent.
pub struct GetServiceDeskOrganizationsRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    start: Option<i64>,
    limit: Option<i64>,
    account_id: Option<String>,
}

impl<'a> GetServiceDeskOrganizationsRequest<'a> {
    fn new(client: &'a crate::core::Client, service_desk_id: impl Into<String>) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), start: None, limit: None, account_id: None }
    }

    /// The starting index of the returned objects. Base index: 0. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of items to return per page. Default: 50. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/servicedesk/{}/organization",
                crate::core::encode_path_segment(&self.service_desk_id)
            ),
        );

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Organization>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method adds an organization to a service desk. If the organization ID is already associated with the service desk, no change is made and the resource returns a 204 success code.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk's agent.
pub struct AddOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    body: OrganizationServiceDeskUpdate,
}

impl<'a> AddOrganizationRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        service_desk_id: impl Into<String>,
        body: OrganizationServiceDeskUpdate,
    ) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/servicedeskapi/servicedesk/{}/organization",
                crate::core::encode_path_segment(&self.service_desk_id)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method removes an organization from a service desk. If the organization ID does not match an organization associated with the service desk, no change is made and the resource returns a 204 success code.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Service desk's agent.
pub struct RemoveOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    body: OrganizationServiceDeskUpdate,
}

impl<'a> RemoveOrganizationRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        service_desk_id: impl Into<String>,
        body: OrganizationServiceDeskUpdate,
    ) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/servicedeskapi/servicedesk/{}/organization",
                crate::core::encode_path_segment(&self.service_desk_id)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
