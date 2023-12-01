#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fact {
    /// The id to state a fact about.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The actual fact.
    #[prost(int64, tag = "2")]
    pub value: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interpretation {
    #[prost(message, repeated, tag = "1")]
    pub facts: ::prost::alloc::vec::Vec<Fact>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropositionInterpretationPair {
    /// The proposition to interpret.
    #[prost(message, optional, tag = "1")]
    pub proposition: ::core::option::Option<super::puan_core::Composite>,
    /// The interpretation to interpret the proposition with.
    #[prost(message, optional, tag = "2")]
    pub interpretation: ::core::option::Option<Interpretation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropositionInterpretationPairSet {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<PropositionInterpretationPair>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropositionInterpretationSet {
    /// The proposition to interpret.
    #[prost(message, repeated, tag = "1")]
    pub propositions: ::prost::alloc::vec::Vec<super::puan_core::Composite>,
    /// The interpretation to interpret the proposition with.
    #[prost(message, repeated, tag = "2")]
    pub interpretations: ::prost::alloc::vec::Vec<Interpretation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundSet {
    #[prost(message, repeated, tag = "1")]
    pub bounds: ::prost::alloc::vec::Vec<super::puan_core::Bound>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundCollection {
    /// The evaluated propositions are bounds. If the lower and upper bound
    /// are equal, the proposition is a fact or definite.
    /// This is a set of sets of bounds.
    #[prost(message, repeated, tag = "1")]
    pub bound_sets: ::prost::alloc::vec::Vec<BoundSet>,
}
/// Generated client implementations.
pub mod evaluation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct EvaluationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EvaluationServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> EvaluationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> EvaluationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            EvaluationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Evaluates the given proposition-interpretation pairs, either streamed or not.
        pub async fn evaluate_pairs(
            &mut self,
            request: impl tonic::IntoRequest<super::PropositionInterpretationPairSet>,
        ) -> std::result::Result<tonic::Response<super::BoundSet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/puan_eval.EvaluationService/EvaluatePairs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("puan_eval.EvaluationService", "EvaluatePairs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn evaluate_pair_streamed(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::PropositionInterpretationPair,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::super::puan_core::Bound>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/puan_eval.EvaluationService/EvaluatePairStreamed",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "puan_eval.EvaluationService",
                        "EvaluatePairStreamed",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// Evaluates the given proposition-interpretation sets.
        pub async fn evaluate_product(
            &mut self,
            request: impl tonic::IntoRequest<super::PropositionInterpretationSet>,
        ) -> std::result::Result<
            tonic::Response<super::BoundCollection>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/puan_eval.EvaluationService/EvaluateProduct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("puan_eval.EvaluationService", "EvaluateProduct"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod evaluation_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EvaluationServiceServer.
    #[async_trait]
    pub trait EvaluationService: Send + Sync + 'static {
        /// Evaluates the given proposition-interpretation pairs, either streamed or not.
        async fn evaluate_pairs(
            &self,
            request: tonic::Request<super::PropositionInterpretationPairSet>,
        ) -> std::result::Result<tonic::Response<super::BoundSet>, tonic::Status>;
        /// Server streaming response type for the EvaluatePairStreamed method.
        type EvaluatePairStreamedStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::super::puan_core::Bound, tonic::Status>,
            >
            + Send
            + 'static;
        async fn evaluate_pair_streamed(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::PropositionInterpretationPair>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::EvaluatePairStreamedStream>,
            tonic::Status,
        >;
        /// Evaluates the given proposition-interpretation sets.
        async fn evaluate_product(
            &self,
            request: tonic::Request<super::PropositionInterpretationSet>,
        ) -> std::result::Result<tonic::Response<super::BoundCollection>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct EvaluationServiceServer<T: EvaluationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EvaluationService> EvaluationServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EvaluationServiceServer<T>
    where
        T: EvaluationService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/puan_eval.EvaluationService/EvaluatePairs" => {
                    #[allow(non_camel_case_types)]
                    struct EvaluatePairsSvc<T: EvaluationService>(pub Arc<T>);
                    impl<
                        T: EvaluationService,
                    > tonic::server::UnaryService<
                        super::PropositionInterpretationPairSet,
                    > for EvaluatePairsSvc<T> {
                        type Response = super::BoundSet;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PropositionInterpretationPairSet,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EvaluationService>::evaluate_pairs(&inner, request)
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
                        let inner = inner.0;
                        let method = EvaluatePairsSvc(inner);
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
                "/puan_eval.EvaluationService/EvaluatePairStreamed" => {
                    #[allow(non_camel_case_types)]
                    struct EvaluatePairStreamedSvc<T: EvaluationService>(pub Arc<T>);
                    impl<
                        T: EvaluationService,
                    > tonic::server::StreamingService<
                        super::PropositionInterpretationPair,
                    > for EvaluatePairStreamedSvc<T> {
                        type Response = super::super::puan_core::Bound;
                        type ResponseStream = T::EvaluatePairStreamedStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::PropositionInterpretationPair>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EvaluationService>::evaluate_pair_streamed(
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
                        let inner = inner.0;
                        let method = EvaluatePairStreamedSvc(inner);
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
                "/puan_eval.EvaluationService/EvaluateProduct" => {
                    #[allow(non_camel_case_types)]
                    struct EvaluateProductSvc<T: EvaluationService>(pub Arc<T>);
                    impl<
                        T: EvaluationService,
                    > tonic::server::UnaryService<super::PropositionInterpretationSet>
                    for EvaluateProductSvc<T> {
                        type Response = super::BoundCollection;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PropositionInterpretationSet>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EvaluationService>::evaluate_product(&inner, request)
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
                        let inner = inner.0;
                        let method = EvaluateProductSvc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: EvaluationService> Clone for EvaluationServiceServer<T> {
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
    impl<T: EvaluationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EvaluationService> tonic::server::NamedService
    for EvaluationServiceServer<T> {
        const NAME: &'static str = "puan_eval.EvaluationService";
    }
}
