// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Customers operations.
pub struct CustomersService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> CustomersService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates a customer that is not associated with a service project.
    ///
    /// The customer's username is their email address. They can set a password by clicking "Forgotten your password" onthe portal login screen, or a Jira administrator can set one in User Management. By default, the customer canemail requests to [public service projects](https://confluence.atlassian.com/display/SERVICEDESKSERVER032/Managing+access+to+your+service+desk). If they have a password, they can also raise requests in customer portals that allowpublic signup. To raise requests in closed service projects, the customer must be added to a service projectusing [Add customers](#servicedeskapi-servicedesk-{serviceDeskId}-customer-post).
    ///
    /// This operation does not cause invitation email to be sent to the newly created customer.
    ///
    /// Jira administrator global permission is required to create a customer.
    pub fn create_customer(&self) -> CreateCustomerRequest<'a> {
        CreateCustomerRequest::new(self.client)
    }

    /// Adds one or more existing customers to the given service project. If you need to create a customer, see Create customer.
    ///
    /// Administer project permission is required, or agents if public signups and invites are enabled for the service project.)
    pub fn add_customers(&self, service_desk_id: impl Into<String>) -> AddCustomersRequest<'a> {
        AddCustomersRequest::new(self.client, service_desk_id)
    }
}

/// Creates a customer that is not associated with a service project.
///
/// The customer's username is their email address. They can set a password by clicking "Forgotten your password" onthe portal login screen, or a Jira administrator can set one in User Management. By default, the customer canemail requests to [public service projects](https://confluence.atlassian.com/display/SERVICEDESKSERVER032/Managing+access+to+your+service+desk). If they have a password, they can also raise requests in customer portals that allowpublic signup. To raise requests in closed service projects, the customer must be added to a service projectusing [Add customers](#servicedeskapi-servicedesk-{serviceDeskId}-customer-post).
///
/// This operation does not cause invitation email to be sent to the newly created customer.
///
/// Jira administrator global permission is required to create a customer.
pub struct CreateCustomerRequest<'a> {
    client: &'a crate::core::Client,
    customer_create: Option<CustomerCreate>,
}

impl<'a> CreateCustomerRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, customer_create: None }
    }

    #[must_use]
    pub fn customer_create(mut self, value: CustomerCreate) -> Self {
        self.customer_create = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/servicedeskapi/customer".to_owned());

        let body = match serde_json::to_value(&self.customer_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<User> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds one or more existing customers to the given service project. If you need to create a customer, see Create customer.
///
/// Administer project permission is required, or agents if public signups and invites are enabled for the service project.)
pub struct AddCustomersRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    service_desk_customer_add: Option<ServiceDeskCustomerAdd>,
}

impl<'a> AddCustomersRequest<'a> {
    fn new(client: &'a crate::core::Client, service_desk_id: impl Into<String>) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), service_desk_customer_add: None }
    }

    #[must_use]
    pub fn service_desk_customer_add(mut self, value: ServiceDeskCustomerAdd) -> Self {
        self.service_desk_customer_add = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/servicedeskapi/servicedesk/{}/customer", self.service_desk_id),
        );

        let body = match serde_json::to_value(&self.service_desk_customer_add)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<User> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
