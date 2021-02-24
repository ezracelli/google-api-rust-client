#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template for address of a customer."]
pub struct Address {
    #[serde(rename = "addressLine1")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A customer's physical address. An address can be composed of one to three lines. The `addressline2` and `addressLine3` are optional."]
    pub address_line1: ::std::option::Option<::std::string::String>,
    #[serde(rename = "addressLine2")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Line 2 of the address."]
    pub address_line2: ::std::option::Option<::std::string::String>,
    #[serde(rename = "addressLine3")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Line 3 of the address."]
    pub address_line3: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The customer contact's name. This is required."]
    pub contact_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For `countryCode` information, see the ISO 3166 country code elements. Verify that country is approved for resale of Google products. This property is required when creating a new customer."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "address_defaults :: kind")]
    #[doc = "Identifies the resource as a customer address. Value: `customers#address`"]
    pub kind: ::std::string::String,
    #[serde(rename = "locality")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An example of a `locality` value is the city of `San Francisco`."]
    pub locality: ::std::option::Option<::std::string::String>,
    #[serde(rename = "organizationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The company or company division name. This is required."]
    pub organization_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A `postalCode` example is a postal zip code such as `94043`. This property is required when creating a new customer."]
    pub postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An example of a `region` value is `CA` for the state of California."]
    pub region: ::std::option::Option<::std::string::String>,
}
mod address_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("customers#address")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template for the ChangePlan rpc request."]
pub struct ChangePlanRequest {
    #[serde(rename = "dealCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google-issued code (100 char max) for discounted pricing on subscription plans. Deal code must be included in `changePlan` request in order to receive discounted rate. This property is optional. If a deal code has already been added to a subscription, this property may be left empty and the existing discounted rate will still apply (if not empty, only provide the deal code that is already present on the subscription). If a deal code has never been added to a subscription and this property is left blank, regular pricing will apply."]
    pub deal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "change_plan_request_defaults :: kind")]
    #[doc = "Identifies the resource as a subscription change plan request. Value: `subscriptions#changePlanRequest`"]
    pub kind: ::std::string::String,
    #[serde(rename = "planName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `planName` property is required. This is the name of the subscription's payment plan. For more information about the Google payment plans, see API concepts. Possible values are: - `ANNUAL_MONTHLY_PAY` - The annual commitment plan with monthly payments *Caution: *`ANNUAL_MONTHLY_PAY` is returned as `ANNUAL` in all API responses. - `ANNUAL_YEARLY_PAY` - The annual commitment plan with yearly payments - `FLEXIBLE` - The flexible plan - `TRIAL` - The 30-day free trial plan "]
    pub plan_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "purchaseOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is an optional property. This purchase order (PO) information is for resellers to use for their company tracking usage. If a `purchaseOrderId` value is given it appears in the API responses and shows up in the invoice. The property accepts up to 80 plain text characters."]
    pub purchase_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is a required property. The seats property is the number of user seat licenses."]
    pub seats: ::std::option::Option<::std::boxed::Box<Seats>>,
}
mod change_plan_request_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("subscriptions#changePlanRequest")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "When a Google customer's account is registered with a reseller, the customer's subscriptions for Google services are managed by this reseller. A customer is described by a primary domain name and a physical address."]
pub struct Customer {
    #[serde(rename = "alternateEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Like the \"Customer email\" in the reseller tools, this email is the secondary contact used if something happens to the customer's service such as service outage or a security issue. This property is required when creating a new customer and should not use the same domain as `customerDomain`."]
    pub alternate_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customerDomain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The customer's primary domain name string. `customerDomain` is required when creating a new customer. Do not include the `www` prefix in the domain when adding a customer."]
    pub customer_domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customerDomainVerified")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the customer's primary domain has been verified."]
    pub customer_domain_verified: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This property will always be returned in a response as the unique identifier generated by Google. In a request, this property can be either the primary domain or the unique identifier generated by Google."]
    pub customer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "customer_defaults :: kind")]
    #[doc = "Identifies the resource as a customer. Value: `reseller#customer`"]
    pub kind: ::std::string::String,
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Customer contact phone number. Must start with \"+\" followed by the country code. The rest of the number can be contiguous numbers or respect the phone local format conventions, but it must be a real phone number and not, for example, \"123\". This field is silently ignored if invalid."]
    pub phone_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A customer's address information. Each field has a limit of 255 charcters."]
    pub postal_address: ::std::option::Option<::std::boxed::Box<Address>>,
    #[serde(rename = "resourceUiUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to customer's Admin console dashboard. The read-only URL is generated by the API service. This is used if your client application requires the customer to complete a task in the Admin console."]
    pub resource_ui_url: ::std::option::Option<::std::string::String>,
}
mod customer_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("reseller#customer")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template for a subscription renewal settings."]
pub struct RenewalSettings {
    #[serde(rename = "kind")]
    #[serde(default = "renewal_settings_defaults :: kind")]
    #[doc = "Identifies the resource as a subscription renewal setting. Value: `subscriptions#renewalSettings`"]
    pub kind: ::std::string::String,
    #[serde(rename = "renewalType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Renewal settings for the annual commitment plan. For more detailed information, see renewal options in the administrator help center. When renewing a subscription, the `renewalType` is a required property."]
    pub renewal_type: ::std::option::Option<::std::string::String>,
}
mod renewal_settings_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("subscriptions#renewalSettings")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template for resellernotify getwatchdetails response."]
pub struct ResellernotifyGetwatchdetailsResponse {
    #[serde(rename = "serviceAccountEmailAddresses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of registered service accounts."]
    pub service_account_email_addresses:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topic name of the PubSub"]
    pub topic_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template for resellernotify response."]
pub struct ResellernotifyResource {
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topic name of the PubSub"]
    pub topic_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template for subscription seats."]
pub struct Seats {
    #[serde(rename = "kind")]
    #[serde(default = "seats_defaults :: kind")]
    #[doc = "Identifies the resource as a subscription seat setting. Value: `subscriptions#seats`"]
    pub kind: ::std::string::String,
    #[serde(rename = "licensedNumberOfSeats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only field containing the current number of users that are assigned a license for the product defined in `skuId`. This field's value is equivalent to the numerical count of users returned by the Enterprise License Manager API method: [`listForProductAndSku`](/admin-sdk/licensing/v1/reference/licenseAssignments/listForProductAndSku)."]
    pub licensed_number_of_seats: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maximumNumberOfSeats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is a required property and is exclusive to subscriptions with `FLEXIBLE` or `TRIAL` plans. This property sets the maximum number of licensed users allowed on a subscription. This quantity can be increased up to the maximum limit defined in the reseller's contract. The minimum quantity is the current number of users in the customer account. *Note: *G Suite subscriptions automatically assign a license to every user."]
    pub maximum_number_of_seats: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "numberOfSeats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is a required property and is exclusive to subscriptions with `ANNUAL_MONTHLY_PAY` and `ANNUAL_YEARLY_PAY` plans. This property sets the maximum number of licenses assignable to users on a subscription. The reseller can add more licenses, but once set, the `numberOfSeats` cannot be reduced until renewal. The reseller is invoiced based on the `numberOfSeats` value regardless of how many of these user licenses are assigned. *Note: *G Suite subscriptions automatically assign a license to every user."]
    pub number_of_seats: ::std::option::Option<::std::primitive::i64>,
}
mod seats_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("subscriptions#seats")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template for a subscription."]
pub struct Subscription {
    #[serde(rename = "billingMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only field that returns the current billing method for a subscription."]
    pub billing_method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `creationTime` property is the date when subscription was created. It is in milliseconds using the Epoch format. See an example Epoch converter."]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customerDomain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Primary domain name of the customer"]
    pub customer_domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This property will always be returned in a response as the unique identifier generated by Google. In a request, this property can be either the primary domain or the unique identifier generated by Google."]
    pub customer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dealCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google-issued code (100 char max) for discounted pricing on subscription plans. Deal code must be included in `insert` requests in order to receive discounted rate. This property is optional, regular pricing applies if left empty."]
    pub deal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "subscription_defaults :: kind")]
    #[doc = "Identifies the resource as a Subscription. Value: `reseller#subscription`"]
    pub kind: ::std::string::String,
    #[serde(rename = "plan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `plan` property is required. In this version of the API, the G Suite plans are the flexible plan, annual commitment plan, and the 30-day free trial plan. For more information about the API\"s payment plans, see the API concepts."]
    pub plan: ::std::option::Option<SubscriptionPlan>,
    #[serde(rename = "purchaseOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is an optional property. This purchase order (PO) information is for resellers to use for their company tracking usage. If a `purchaseOrderId` value is given it appears in the API responses and shows up in the invoice. The property accepts up to 80 plain text characters."]
    pub purchase_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "renewalSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Renewal settings for the annual commitment plan. For more detailed information, see renewal options in the administrator help center."]
    pub renewal_settings: ::std::option::Option<::std::boxed::Box<RenewalSettings>>,
    #[serde(rename = "resourceUiUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to customer's Subscriptions page in the Admin console. The read-only URL is generated by the API service. This is used if your client application requires the customer to complete a task using the Subscriptions page in the Admin console."]
    pub resource_ui_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is a required property. The number and limit of user seat licenses in the plan."]
    pub seats: ::std::option::Option<::std::boxed::Box<Seats>>,
    #[serde(rename = "skuId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A required property. The `skuId` is a unique system identifier for a product's SKU assigned to a customer in the subscription. For products and SKUs available in this version of the API, see Product and SKU IDs."]
    pub sku_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "skuName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only external display name for a product's SKU assigned to a customer in the subscription. SKU names are subject to change at Google's discretion. For products and SKUs available in this version of the API, see Product and SKU IDs."]
    pub sku_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is an optional property."]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subscriptionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `subscriptionId` is the subscription identifier and is unique for each customer. This is a required property. Since a `subscriptionId` changes when a subscription is updated, we recommend not using this ID as a key for persistent data. Use the `subscriptionId` as described in retrieve all reseller subscriptions."]
    pub subscription_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "suspensionReasons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only field containing an enumerable of all the current suspension reasons for a subscription. It is possible for a subscription to have many concurrent, overlapping suspension reasons. A subscription's `STATUS` is `SUSPENDED` until all pending suspensions are removed. Possible options include: - `PENDING_TOS_ACCEPTANCE` - The customer has not logged in and accepted the G Suite Resold Terms of Services. - `RENEWAL_WITH_TYPE_CANCEL` - The customer's commitment ended and their service was cancelled at the end of their term. - `RESELLER_INITIATED` - A manual suspension invoked by a Reseller. - `TRIAL_ENDED` - The customer's trial expired without a plan selected. - `OTHER` - The customer is suspended for an internal Google reason (e.g. abuse or otherwise). "]
    pub suspension_reasons: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "transferInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only transfer related information for the subscription. For more information, see retrieve transferable subscriptions for a customer."]
    pub transfer_info: ::std::option::Option<SubscriptionTransferInfo>,
    #[serde(rename = "trialSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The G Suite annual commitment and flexible payment plans can be in a 30-day free trial. For more information, see the API concepts."]
    pub trial_settings: ::std::option::Option<SubscriptionTrialSettings>,
}
mod subscription_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("reseller#subscription")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `plan` property is required. In this version of the API, the G Suite plans are the flexible plan, annual commitment plan, and the 30-day free trial plan. For more information about the API\"s payment plans, see the API concepts."]
pub struct SubscriptionPlan {
    #[serde(rename = "commitmentInterval")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "In this version of the API, annual commitment plan's interval is one year. *Note: *When `billingMethod` value is `OFFLINE`, the subscription property object `plan.commitmentInterval` is omitted in all API responses. "]
    pub commitment_interval: ::std::option::Option<SubscriptionPlanCommitmentInterval>,
    #[serde(rename = "isCommitmentPlan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `isCommitmentPlan` property's boolean value identifies the plan as an annual commitment plan: - `true` — The subscription's plan is an annual commitment plan. - `false` — The plan is not an annual commitment plan. "]
    pub is_commitment_plan: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "planName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `planName` property is required. This is the name of the subscription's plan. For more information about the Google payment plans, see the API concepts. Possible values are: - `ANNUAL_MONTHLY_PAY` — The annual commitment plan with monthly payments. *Caution: *`ANNUAL_MONTHLY_PAY` is returned as `ANNUAL` in all API responses. - `ANNUAL_YEARLY_PAY` — The annual commitment plan with yearly payments - `FLEXIBLE` — The flexible plan - `TRIAL` — The 30-day free trial plan. A subscription in trial will be suspended after the 30th free day if no payment plan is assigned. Calling `changePlan` will assign a payment plan to a trial but will not activate the plan. A trial will automatically begin its assigned payment plan after its 30th free day or immediately after calling `startPaidService`. - `FREE` — The free plan is exclusive to the Cloud Identity SKU and does not incur any billing. "]
    pub plan_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "In this version of the API, annual commitment plan's interval is one year. *Note: *When `billingMethod` value is `OFFLINE`, the subscription property object `plan.commitmentInterval` is omitted in all API responses. "]
pub struct SubscriptionPlanCommitmentInterval {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An annual commitment plan's interval's `endTime` in milliseconds using the UNIX Epoch format. See an example Epoch converter."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An annual commitment plan's interval's `startTime` in milliseconds using UNIX Epoch format. See an example Epoch converter."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Read-only transfer related information for the subscription. For more information, see retrieve transferable subscriptions for a customer."]
pub struct SubscriptionTransferInfo {
    #[serde(rename = "minimumTransferableSeats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When inserting a subscription, this is the minimum number of seats listed in the transfer order for this product. For example, if the customer has 20 users, the reseller cannot place a transfer order of 15 seats. The minimum is 20 seats."]
    pub minimum_transferable_seats: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "transferabilityExpirationTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when transfer token or intent to transfer will expire. The time is in milliseconds using UNIX Epoch format."]
    pub transferability_expiration_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The G Suite annual commitment and flexible payment plans can be in a 30-day free trial. For more information, see the API concepts."]
pub struct SubscriptionTrialSettings {
    #[serde(rename = "isInTrial")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Determines if a subscription's plan is in a 30-day free trial or not: - `true` — The plan is in trial. - `false` — The plan is not in trial. "]
    pub is_in_trial: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "trialEndTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date when the trial ends. The value is in milliseconds using the UNIX Epoch format. See an example Epoch converter."]
    pub trial_end_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A subscription manages the relationship of a Google customer's payment plan with a product's SKU, user licenses, 30-day free trial status, and renewal options. A primary role of a reseller is to manage the Google customer's subscriptions."]
pub struct Subscriptions {
    #[serde(rename = "kind")]
    #[serde(default = "subscriptions_defaults :: kind")]
    #[doc = "Identifies the resource as a collection of subscriptions. Value: reseller#subscriptions"]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subscriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The subscriptions in this page of results."]
    pub subscriptions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Subscription>>>,
}
mod subscriptions_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("reseller#subscriptions")
    }
}
