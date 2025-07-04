// This file is @generated by prost-build.
/// Defines supported protocols etc, so the management server can assign proper
/// endpoints to healthcheck.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Capability {
    #[prost(enumeration = "capability::Protocol", repeated, tag = "1")]
    pub health_check_protocols: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `Capability`.
pub mod capability {
    /// Different Envoy instances may have different capabilities (e.g. Redis)
    /// and/or have ports enabled for different protocols.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Protocol {
        Http = 0,
        Tcp = 1,
        Redis = 2,
    }
    impl Protocol {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Http => "HTTP",
                Self::Tcp => "TCP",
                Self::Redis => "REDIS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HTTP" => Some(Self::Http),
                "TCP" => Some(Self::Tcp),
                "REDIS" => Some(Self::Redis),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for Capability {
    const NAME: &'static str = "Capability";
    const PACKAGE: &'static str = "envoy.service.health.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.health.v3.Capability".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.health.v3.Capability".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckRequest {
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<super::super::super::config::core::v3::Node>,
    #[prost(message, optional, tag = "2")]
    pub capability: ::core::option::Option<Capability>,
}
impl ::prost::Name for HealthCheckRequest {
    const NAME: &'static str = "HealthCheckRequest";
    const PACKAGE: &'static str = "envoy.service.health.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.health.v3.HealthCheckRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.health.v3.HealthCheckRequest".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointHealth {
    #[prost(message, optional, tag = "1")]
    pub endpoint: ::core::option::Option<
        super::super::super::config::endpoint::v3::Endpoint,
    >,
    #[prost(
        enumeration = "super::super::super::config::core::v3::HealthStatus",
        tag = "2"
    )]
    pub health_status: i32,
}
impl ::prost::Name for EndpointHealth {
    const NAME: &'static str = "EndpointHealth";
    const PACKAGE: &'static str = "envoy.service.health.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.health.v3.EndpointHealth".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.health.v3.EndpointHealth".into()
    }
}
/// Group endpoint health by locality under each cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalityEndpointsHealth {
    #[prost(message, optional, tag = "1")]
    pub locality: ::core::option::Option<
        super::super::super::config::core::v3::Locality,
    >,
    #[prost(message, repeated, tag = "2")]
    pub endpoints_health: ::prost::alloc::vec::Vec<EndpointHealth>,
}
impl ::prost::Name for LocalityEndpointsHealth {
    const NAME: &'static str = "LocalityEndpointsHealth";
    const PACKAGE: &'static str = "envoy.service.health.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.health.v3.LocalityEndpointsHealth".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.health.v3.LocalityEndpointsHealth".into()
    }
}
/// The health status of endpoints in a cluster. The cluster name and locality
/// should match the corresponding fields in ClusterHealthCheck message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterEndpointsHealth {
    #[prost(string, tag = "1")]
    pub cluster_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub locality_endpoints_health: ::prost::alloc::vec::Vec<LocalityEndpointsHealth>,
}
impl ::prost::Name for ClusterEndpointsHealth {
    const NAME: &'static str = "ClusterEndpointsHealth";
    const PACKAGE: &'static str = "envoy.service.health.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.health.v3.ClusterEndpointsHealth".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.health.v3.ClusterEndpointsHealth".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointHealthResponse {
    /// Deprecated - Flat list of endpoint health information.
    #[deprecated]
    #[prost(message, repeated, tag = "1")]
    pub endpoints_health: ::prost::alloc::vec::Vec<EndpointHealth>,
    /// Organize Endpoint health information by cluster.
    #[prost(message, repeated, tag = "2")]
    pub cluster_endpoints_health: ::prost::alloc::vec::Vec<ClusterEndpointsHealth>,
}
impl ::prost::Name for EndpointHealthResponse {
    const NAME: &'static str = "EndpointHealthResponse";
    const PACKAGE: &'static str = "envoy.service.health.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.health.v3.EndpointHealthResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.health.v3.EndpointHealthResponse".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckRequestOrEndpointHealthResponse {
    #[prost(
        oneof = "health_check_request_or_endpoint_health_response::RequestType",
        tags = "1, 2"
    )]
    pub request_type: ::core::option::Option<
        health_check_request_or_endpoint_health_response::RequestType,
    >,
}
/// Nested message and enum types in `HealthCheckRequestOrEndpointHealthResponse`.
pub mod health_check_request_or_endpoint_health_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestType {
        #[prost(message, tag = "1")]
        HealthCheckRequest(super::HealthCheckRequest),
        #[prost(message, tag = "2")]
        EndpointHealthResponse(super::EndpointHealthResponse),
    }
}
impl ::prost::Name for HealthCheckRequestOrEndpointHealthResponse {
    const NAME: &'static str = "HealthCheckRequestOrEndpointHealthResponse";
    const PACKAGE: &'static str = "envoy.service.health.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.health.v3.HealthCheckRequestOrEndpointHealthResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.health.v3.HealthCheckRequestOrEndpointHealthResponse"
            .into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalityEndpoints {
    #[prost(message, optional, tag = "1")]
    pub locality: ::core::option::Option<
        super::super::super::config::core::v3::Locality,
    >,
    #[prost(message, repeated, tag = "2")]
    pub endpoints: ::prost::alloc::vec::Vec<
        super::super::super::config::endpoint::v3::Endpoint,
    >,
}
impl ::prost::Name for LocalityEndpoints {
    const NAME: &'static str = "LocalityEndpoints";
    const PACKAGE: &'static str = "envoy.service.health.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.health.v3.LocalityEndpoints".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.health.v3.LocalityEndpoints".into()
    }
}
/// The cluster name and locality is provided to Envoy for the endpoints that it
/// health checks to support statistics reporting, logging and debugging by the
/// Envoy instance (outside of HDS). For maximum usefulness, it should match the
/// same cluster structure as that provided by EDS.
/// \[#next-free-field: 6\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterHealthCheck {
    #[prost(string, tag = "1")]
    pub cluster_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub health_checks: ::prost::alloc::vec::Vec<
        super::super::super::config::core::v3::HealthCheck,
    >,
    #[prost(message, repeated, tag = "3")]
    pub locality_endpoints: ::prost::alloc::vec::Vec<LocalityEndpoints>,
    /// Optional map that gets filtered by :ref:`health_checks.transport_socket_match_criteria <envoy_v3_api_field_config.core.v3.HealthCheck.transport_socket_match_criteria>`
    /// on connection when health checking. For more details, see
    /// :ref:`config.cluster.v3.Cluster.transport_socket_matches <envoy_v3_api_field_config.cluster.v3.Cluster.transport_socket_matches>`.
    #[prost(message, repeated, tag = "4")]
    pub transport_socket_matches: ::prost::alloc::vec::Vec<
        super::super::super::config::cluster::v3::cluster::TransportSocketMatch,
    >,
    /// Optional configuration used to bind newly established upstream connections.
    /// If the address and port are empty, no bind will be performed.
    #[prost(message, optional, tag = "5")]
    pub upstream_bind_config: ::core::option::Option<
        super::super::super::config::core::v3::BindConfig,
    >,
}
impl ::prost::Name for ClusterHealthCheck {
    const NAME: &'static str = "ClusterHealthCheck";
    const PACKAGE: &'static str = "envoy.service.health.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.health.v3.ClusterHealthCheck".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.health.v3.ClusterHealthCheck".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckSpecifier {
    #[prost(message, repeated, tag = "1")]
    pub cluster_health_checks: ::prost::alloc::vec::Vec<ClusterHealthCheck>,
    /// The default is 1 second.
    #[prost(message, optional, tag = "2")]
    pub interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
}
impl ::prost::Name for HealthCheckSpecifier {
    const NAME: &'static str = "HealthCheckSpecifier";
    const PACKAGE: &'static str = "envoy.service.health.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.health.v3.HealthCheckSpecifier".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.health.v3.HealthCheckSpecifier".into()
    }
}
/// \[#not-implemented-hide:\] Not configuration. Workaround c++ protobuf issue with importing
/// services: <https://github.com/google/protobuf/issues/4221> and protoxform to upgrade the file.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HdsDummy {}
impl ::prost::Name for HdsDummy {
    const NAME: &'static str = "HdsDummy";
    const PACKAGE: &'static str = "envoy.service.health.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.health.v3.HdsDummy".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.health.v3.HdsDummy".into()
    }
}
/// Generated client implementations.
pub mod health_discovery_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// HDS is Health Discovery Service. It compliments Envoy’s health checking
    /// service by designating this Envoy to be a healthchecker for a subset of hosts
    /// in the cluster. The status of these health checks will be reported to the
    /// management server, where it can be aggregated etc and redistributed back to
    /// Envoy through EDS.
    #[derive(Debug, Clone)]
    pub struct HealthDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> HealthDiscoveryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> HealthDiscoveryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            HealthDiscoveryServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// 1. Envoy starts up and if its can_healthcheck option in the static
        ///    bootstrap config is enabled, sends HealthCheckRequest to the management
        ///    server. It supplies its capabilities (which protocol it can health check
        ///    with, what zone it resides in, etc.).
        /// 2. In response to (1), the management server designates this Envoy as a
        ///    healthchecker to health check a subset of all upstream hosts for a given
        ///    cluster (for example upstream Host 1 and Host 2). It streams
        ///    HealthCheckSpecifier messages with cluster related configuration for all
        ///    clusters this Envoy is designated to health check. Subsequent
        ///    HealthCheckSpecifier message will be sent on changes to:
        ///    a. Endpoints to health checks
        ///    b. Per cluster configuration change
        /// 3. Envoy creates a health probe based on the HealthCheck config and sends
        ///    it to endpoint(ip:port) of Host 1 and 2. Based on the HealthCheck
        ///    configuration Envoy waits upon the arrival of the probe response and
        ///    looks at the content of the response to decide whether the endpoint is
        ///    healthy or not. If a response hasn't been received within the timeout
        ///    interval, the endpoint health status is considered TIMEOUT.
        /// 4. Envoy reports results back in an EndpointHealthResponse message.
        ///    Envoy streams responses as often as the interval configured by the
        ///    management server in HealthCheckSpecifier.
        /// 5. The management Server collects health statuses for all endpoints in the
        ///    cluster (for all clusters) and uses this information to construct
        ///    EndpointDiscoveryResponse messages.
        /// 6. Once Envoy has a list of upstream endpoints to send traffic to, it load
        ///    balances traffic to them without additional health checking. It may
        ///    use inline healthcheck (i.e. consider endpoint UNHEALTHY if connection
        ///    failed to a particular endpoint to account for health status propagation
        ///    delay between HDS and EDS).
        /// By default, can_healthcheck is true. If can_healthcheck is false, Cluster
        /// configuration may not contain HealthCheck message.
        /// TODO(htuch): How is can_healthcheck communicated to CDS to ensure the above
        /// invariant?
        /// TODO(htuch): Add @amb67's diagram.
        pub async fn stream_health_check(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::HealthCheckRequestOrEndpointHealthResponse,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::HealthCheckSpecifier>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.health.v3.HealthDiscoveryService/StreamHealthCheck",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.health.v3.HealthDiscoveryService",
                        "StreamHealthCheck",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// TODO(htuch): Unlike the gRPC version, there is no stream-based binding of
        /// request/response. Should we add an identifier to the HealthCheckSpecifier
        /// to bind with the response?
        pub async fn fetch_health_check(
            &mut self,
            request: impl tonic::IntoRequest<
                super::HealthCheckRequestOrEndpointHealthResponse,
            >,
        ) -> std::result::Result<
            tonic::Response<super::HealthCheckSpecifier>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.health.v3.HealthDiscoveryService/FetchHealthCheck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.health.v3.HealthDiscoveryService",
                        "FetchHealthCheck",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod health_discovery_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with HealthDiscoveryServiceServer.
    #[async_trait]
    pub trait HealthDiscoveryService: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the StreamHealthCheck method.
        type StreamHealthCheckStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::HealthCheckSpecifier, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        /// 1. Envoy starts up and if its can_healthcheck option in the static
        ///    bootstrap config is enabled, sends HealthCheckRequest to the management
        ///    server. It supplies its capabilities (which protocol it can health check
        ///    with, what zone it resides in, etc.).
        /// 2. In response to (1), the management server designates this Envoy as a
        ///    healthchecker to health check a subset of all upstream hosts for a given
        ///    cluster (for example upstream Host 1 and Host 2). It streams
        ///    HealthCheckSpecifier messages with cluster related configuration for all
        ///    clusters this Envoy is designated to health check. Subsequent
        ///    HealthCheckSpecifier message will be sent on changes to:
        ///    a. Endpoints to health checks
        ///    b. Per cluster configuration change
        /// 3. Envoy creates a health probe based on the HealthCheck config and sends
        ///    it to endpoint(ip:port) of Host 1 and 2. Based on the HealthCheck
        ///    configuration Envoy waits upon the arrival of the probe response and
        ///    looks at the content of the response to decide whether the endpoint is
        ///    healthy or not. If a response hasn't been received within the timeout
        ///    interval, the endpoint health status is considered TIMEOUT.
        /// 4. Envoy reports results back in an EndpointHealthResponse message.
        ///    Envoy streams responses as often as the interval configured by the
        ///    management server in HealthCheckSpecifier.
        /// 5. The management Server collects health statuses for all endpoints in the
        ///    cluster (for all clusters) and uses this information to construct
        ///    EndpointDiscoveryResponse messages.
        /// 6. Once Envoy has a list of upstream endpoints to send traffic to, it load
        ///    balances traffic to them without additional health checking. It may
        ///    use inline healthcheck (i.e. consider endpoint UNHEALTHY if connection
        ///    failed to a particular endpoint to account for health status propagation
        ///    delay between HDS and EDS).
        /// By default, can_healthcheck is true. If can_healthcheck is false, Cluster
        /// configuration may not contain HealthCheck message.
        /// TODO(htuch): How is can_healthcheck communicated to CDS to ensure the above
        /// invariant?
        /// TODO(htuch): Add @amb67's diagram.
        async fn stream_health_check(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::HealthCheckRequestOrEndpointHealthResponse>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::StreamHealthCheckStream>,
            tonic::Status,
        >;
        /// TODO(htuch): Unlike the gRPC version, there is no stream-based binding of
        /// request/response. Should we add an identifier to the HealthCheckSpecifier
        /// to bind with the response?
        async fn fetch_health_check(
            &self,
            request: tonic::Request<super::HealthCheckRequestOrEndpointHealthResponse>,
        ) -> std::result::Result<
            tonic::Response<super::HealthCheckSpecifier>,
            tonic::Status,
        >;
    }
    /// HDS is Health Discovery Service. It compliments Envoy’s health checking
    /// service by designating this Envoy to be a healthchecker for a subset of hosts
    /// in the cluster. The status of these health checks will be reported to the
    /// management server, where it can be aggregated etc and redistributed back to
    /// Envoy through EDS.
    #[derive(Debug)]
    pub struct HealthDiscoveryServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> HealthDiscoveryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for HealthDiscoveryServiceServer<T>
    where
        T: HealthDiscoveryService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/envoy.service.health.v3.HealthDiscoveryService/StreamHealthCheck" => {
                    #[allow(non_camel_case_types)]
                    struct StreamHealthCheckSvc<T: HealthDiscoveryService>(pub Arc<T>);
                    impl<
                        T: HealthDiscoveryService,
                    > tonic::server::StreamingService<
                        super::HealthCheckRequestOrEndpointHealthResponse,
                    > for StreamHealthCheckSvc<T> {
                        type Response = super::HealthCheckSpecifier;
                        type ResponseStream = T::StreamHealthCheckStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::HealthCheckRequestOrEndpointHealthResponse,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as HealthDiscoveryService>::stream_health_check(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StreamHealthCheckSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/envoy.service.health.v3.HealthDiscoveryService/FetchHealthCheck" => {
                    #[allow(non_camel_case_types)]
                    struct FetchHealthCheckSvc<T: HealthDiscoveryService>(pub Arc<T>);
                    impl<
                        T: HealthDiscoveryService,
                    > tonic::server::UnaryService<
                        super::HealthCheckRequestOrEndpointHealthResponse,
                    > for FetchHealthCheckSvc<T> {
                        type Response = super::HealthCheckSpecifier;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::HealthCheckRequestOrEndpointHealthResponse,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as HealthDiscoveryService>::fetch_health_check(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = FetchHealthCheckSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for HealthDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "envoy.service.health.v3.HealthDiscoveryService";
    impl<T> tonic::server::NamedService for HealthDiscoveryServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
