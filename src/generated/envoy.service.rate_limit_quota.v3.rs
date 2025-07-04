// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitQuotaUsageReports {
    /// All quota requests must specify the domain. This enables sharing the quota
    /// server between different applications without fear of overlap.
    /// E.g., "envoy".
    ///
    /// Should only be provided in the first report, all subsequent messages on the same
    /// stream are considered to be in the same domain. In case the domain needs to be
    /// changes, close the stream, and reopen a new one with the different domain.
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    /// A list of quota usage reports. The list is processed by the RLQS server in the same order
    /// it's provided by the client.
    #[prost(message, repeated, tag = "2")]
    pub bucket_quota_usages: ::prost::alloc::vec::Vec<
        rate_limit_quota_usage_reports::BucketQuotaUsage,
    >,
}
/// Nested message and enum types in `RateLimitQuotaUsageReports`.
pub mod rate_limit_quota_usage_reports {
    /// The usage report for a bucket.
    ///
    /// .. note::
    ///    Note that the first report sent for a ``BucketId`` indicates to the RLQS server that
    ///    the RLQS client is subscribing for the future assignments for this ``BucketId``.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BucketQuotaUsage {
        /// ``BucketId`` for which request quota usage is reported.
        #[prost(message, optional, tag = "1")]
        pub bucket_id: ::core::option::Option<super::BucketId>,
        /// Time elapsed since the last report.
        #[prost(message, optional, tag = "2")]
        pub time_elapsed: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
        /// Requests the data plane has allowed through.
        #[prost(uint64, tag = "3")]
        pub num_requests_allowed: u64,
        /// Requests throttled.
        #[prost(uint64, tag = "4")]
        pub num_requests_denied: u64,
    }
    impl ::prost::Name for BucketQuotaUsage {
        const NAME: &'static str = "BucketQuotaUsage";
        const PACKAGE: &'static str = "envoy.service.rate_limit_quota.v3";
        fn full_name() -> ::prost::alloc::string::String {
            "envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports.BucketQuotaUsage"
                .into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports.BucketQuotaUsage"
                .into()
        }
    }
}
impl ::prost::Name for RateLimitQuotaUsageReports {
    const NAME: &'static str = "RateLimitQuotaUsageReports";
    const PACKAGE: &'static str = "envoy.service.rate_limit_quota.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports"
            .into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitQuotaResponse {
    /// An ordered list of actions to be applied to the buckets. The actions are applied in the
    /// given order, from top to bottom.
    #[prost(message, repeated, tag = "1")]
    pub bucket_action: ::prost::alloc::vec::Vec<rate_limit_quota_response::BucketAction>,
}
/// Nested message and enum types in `RateLimitQuotaResponse`.
pub mod rate_limit_quota_response {
    /// Commands the data plane to apply one of the actions to the bucket with the
    /// :ref:`bucket_id <envoy_v3_api_field_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.bucket_id>`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BucketAction {
        /// ``BucketId`` for which request the action is applied.
        #[prost(message, optional, tag = "1")]
        pub bucket_id: ::core::option::Option<super::BucketId>,
        #[prost(oneof = "bucket_action::BucketAction", tags = "2, 3")]
        pub bucket_action: ::core::option::Option<bucket_action::BucketAction>,
    }
    /// Nested message and enum types in `BucketAction`.
    pub mod bucket_action {
        /// Quota assignment for the bucket. Configures the rate limiting strategy and the duration
        /// for the given :ref:`bucket_id
        /// <envoy_v3_api_field_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.bucket_id>`.
        ///
        /// **Applying the first assignment to the bucket**
        ///
        /// Once the data plane receives the ``QuotaAssignmentAction``, it must send the current usage
        /// report for the bucket, and start rate limiting requests matched into the bucket
        /// using the strategy configured in the :ref:`rate_limit_strategy
        /// <envoy_v3_api_field_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction.rate_limit_strategy>`
        /// field. The assignment becomes bucket's ``active`` assignment.
        ///
        /// **Expiring the assignment**
        ///
        /// The duration of the assignment defined in the :ref:`assignment_time_to_live
        /// <envoy_v3_api_field_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction.assignment_time_to_live>`
        /// field. When the duration runs off, the assignment is ``expired``, and no longer ``active``.
        /// The data plane should stop applying the rate limiting strategy to the bucket, and transition
        /// the bucket to the "expired assignment" state. This activates the behavior configured in the
        /// :ref:`expired_assignment_behavior <envoy_v3_api_field_extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.expired_assignment_behavior>`
        /// field.
        ///
        /// **Replacing the assignment**
        ///
        /// * If the rate limiting strategy is different from bucket's ``active`` assignment, or
        ///    the current bucket assignment is ``expired``, the data plane must immediately
        ///    end the current assignment, report the bucket usage, and apply the new assignment.
        ///    The new assignment becomes bucket's ``active`` assignment.
        /// * If the rate limiting strategy is the same as the bucket's ``active`` (not ``expired``)
        ///    assignment, the data plane should extend the duration of the ``active`` assignment
        ///    for the duration of the new assignment provided in the :ref:`assignment_time_to_live
        ///    <envoy_v3_api_field_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction.assignment_time_to_live>`
        ///    field. The ``active`` assignment is considered unchanged.
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct QuotaAssignmentAction {
            /// A duration after which the assignment is be considered ``expired``. The process of the
            /// expiration is described :ref:`above
            /// <envoy_v3_api_msg_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction>`.
            ///
            /// * If unset, the assignment has no expiration date.
            /// * If set to ``0``, the assignment expires immediately, forcing the client into the
            ///    :ref:`"expired assignment"
            ///    <envoy_v3_api_field_extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior.expired_assignment_behavior_timeout>`
            ///    state. This may be used by the RLQS server in cases when it needs clients to proactively
            ///    fall back to the pre-configured :ref:`ExpiredAssignmentBehavior
            ///    <envoy_v3_api_msg_extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior>`,
            ///    f.e. before the server going into restart.
            ///
            /// .. attention::
            ///    Note that :ref:`expiring
            ///    <envoy_v3_api_msg_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction>`
            ///    the assignment is not the same as :ref:`abandoning
            ///    <envoy_v3_api_msg_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.AbandonAction>`
            ///    the assignment. While expiring the assignment just transitions the bucket to
            ///    the "expired assignment" state; abandoning the assignment completely erases
            ///    the bucket from the data plane memory, and stops the usage reports.
            #[prost(message, optional, tag = "2")]
            pub assignment_time_to_live: ::core::option::Option<
                super::super::super::super::super::super::google::protobuf::Duration,
            >,
            /// Configures the local rate limiter for the request matched to the bucket.
            /// If not set, allow all requests.
            #[prost(message, optional, tag = "3")]
            pub rate_limit_strategy: ::core::option::Option<
                super::super::super::super::super::r#type::v3::RateLimitStrategy,
            >,
        }
        impl ::prost::Name for QuotaAssignmentAction {
            const NAME: &'static str = "QuotaAssignmentAction";
            const PACKAGE: &'static str = "envoy.service.rate_limit_quota.v3";
            fn full_name() -> ::prost::alloc::string::String {
                "envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction"
                    .into()
            }
            fn type_url() -> ::prost::alloc::string::String {
                "type.googleapis.com/envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction"
                    .into()
            }
        }
        /// Abandon action for the bucket. Indicates that the RLQS server will no longer be
        /// sending updates for the given :ref:`bucket_id
        /// <envoy_v3_api_field_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.bucket_id>`.
        ///
        /// If no requests are reported for a bucket, after some time the server considers the bucket
        /// inactive. The server stops tracking the bucket, and instructs the the data plane to abandon
        /// the bucket via this message.
        ///
        /// **Abandoning the assignment**
        ///
        /// The data plane is to erase the bucket (including its usage data) from the memory.
        /// It should stop tracking the bucket, and stop reporting its usage. This effectively resets
        /// the data plane to the state prior to matching the first request into the bucket.
        ///
        /// **Restarting the subscription**
        ///
        /// If a new request is matched into a bucket previously abandoned, the data plane must behave
        /// as if it has never tracked the bucket, and it's the first request matched into it:
        ///
        /// 1. The process of :ref:`subscription and reporting
        ///     <envoy_v3_api_field_extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.reporting_interval>`
        ///     starts from the beginning.
        ///
        /// 2. The bucket transitions to the :ref:`"no assignment"
        ///     <envoy_v3_api_field_extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.no_assignment_behavior>`
        ///     state.
        ///
        /// 3. Once the new assignment is received, it's applied per
        ///     "Applying the first assignment to the bucket" section of the :ref:`QuotaAssignmentAction
        ///     <envoy_v3_api_msg_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction>`.
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct AbandonAction {}
        impl ::prost::Name for AbandonAction {
            const NAME: &'static str = "AbandonAction";
            const PACKAGE: &'static str = "envoy.service.rate_limit_quota.v3";
            fn full_name() -> ::prost::alloc::string::String {
                "envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.AbandonAction"
                    .into()
            }
            fn type_url() -> ::prost::alloc::string::String {
                "type.googleapis.com/envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.AbandonAction"
                    .into()
            }
        }
        #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
        pub enum BucketAction {
            /// Apply the quota assignment to the bucket.
            ///
            /// Commands the data plane to apply a rate limiting strategy to the bucket.
            /// The process of applying and expiring the rate limiting strategy is detailed in the
            /// :ref:`QuotaAssignmentAction
            /// <envoy_v3_api_msg_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction>`
            /// message.
            #[prost(message, tag = "2")]
            QuotaAssignmentAction(QuotaAssignmentAction),
            /// Abandon the bucket.
            ///
            /// Commands the data plane to abandon the bucket.
            /// The process of abandoning the bucket is described in the :ref:`AbandonAction
            /// <envoy_v3_api_msg_service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.AbandonAction>`
            /// message.
            #[prost(message, tag = "3")]
            AbandonAction(AbandonAction),
        }
    }
    impl ::prost::Name for BucketAction {
        const NAME: &'static str = "BucketAction";
        const PACKAGE: &'static str = "envoy.service.rate_limit_quota.v3";
        fn full_name() -> ::prost::alloc::string::String {
            "envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction"
                .into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction"
                .into()
        }
    }
}
impl ::prost::Name for RateLimitQuotaResponse {
    const NAME: &'static str = "RateLimitQuotaResponse";
    const PACKAGE: &'static str = "envoy.service.rate_limit_quota.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse"
            .into()
    }
}
/// The identifier for the bucket. Used to match the bucket between the control plane (RLQS server),
/// and the data plane (RLQS client), f.e.:
///
/// * the data plane sends a usage report for requests matched into the bucket with ``BucketId``
///    to the control plane
/// * the control plane sends an assignment for the bucket with ``BucketId`` to the data plane
///    Bucket ID.
///
/// Example:
///
/// .. validated-code-block:: yaml
///    :type-name: envoy.service.rate_limit_quota.v3.BucketId
///
///    bucket:
///      name: my_bucket
///      env: staging
///
/// .. note::
///    The order of ``BucketId`` keys do not matter. Buckets ``{ a: 'A', b: 'B' }`` and
///    ``{ b: 'B', a: 'A' }`` are identical.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketId {
    #[prost(map = "string, string", tag = "1")]
    pub bucket: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
impl ::prost::Name for BucketId {
    const NAME: &'static str = "BucketId";
    const PACKAGE: &'static str = "envoy.service.rate_limit_quota.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.service.rate_limit_quota.v3.BucketId".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.service.rate_limit_quota.v3.BucketId".into()
    }
}
/// Generated client implementations.
pub mod rate_limit_quota_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Defines the Rate Limit Quota Service (RLQS).
    #[derive(Debug, Clone)]
    pub struct RateLimitQuotaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RateLimitQuotaServiceClient<T>
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
        ) -> RateLimitQuotaServiceClient<InterceptedService<T, F>>
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
            RateLimitQuotaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Main communication channel: the data plane sends usage reports to the RLQS server,
        /// and the server asynchronously responding with the assignments.
        pub async fn stream_rate_limit_quotas(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::RateLimitQuotaUsageReports,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::RateLimitQuotaResponse>>,
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
                "/envoy.service.rate_limit_quota.v3.RateLimitQuotaService/StreamRateLimitQuotas",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.rate_limit_quota.v3.RateLimitQuotaService",
                        "StreamRateLimitQuotas",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod rate_limit_quota_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RateLimitQuotaServiceServer.
    #[async_trait]
    pub trait RateLimitQuotaService: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the StreamRateLimitQuotas method.
        type StreamRateLimitQuotasStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::RateLimitQuotaResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        /// Main communication channel: the data plane sends usage reports to the RLQS server,
        /// and the server asynchronously responding with the assignments.
        async fn stream_rate_limit_quotas(
            &self,
            request: tonic::Request<tonic::Streaming<super::RateLimitQuotaUsageReports>>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamRateLimitQuotasStream>,
            tonic::Status,
        >;
    }
    /// Defines the Rate Limit Quota Service (RLQS).
    #[derive(Debug)]
    pub struct RateLimitQuotaServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> RateLimitQuotaServiceServer<T> {
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
    for RateLimitQuotaServiceServer<T>
    where
        T: RateLimitQuotaService,
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
                "/envoy.service.rate_limit_quota.v3.RateLimitQuotaService/StreamRateLimitQuotas" => {
                    #[allow(non_camel_case_types)]
                    struct StreamRateLimitQuotasSvc<T: RateLimitQuotaService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: RateLimitQuotaService,
                    > tonic::server::StreamingService<super::RateLimitQuotaUsageReports>
                    for StreamRateLimitQuotasSvc<T> {
                        type Response = super::RateLimitQuotaResponse;
                        type ResponseStream = T::StreamRateLimitQuotasStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::RateLimitQuotaUsageReports>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RateLimitQuotaService>::stream_rate_limit_quotas(
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
                        let method = StreamRateLimitQuotasSvc(inner);
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
    impl<T> Clone for RateLimitQuotaServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "envoy.service.rate_limit_quota.v3.RateLimitQuotaService";
    impl<T> tonic::server::NamedService for RateLimitQuotaServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
