use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use tonic::codegen::{Body, Service, StdError, http};

pub mod google {
    pub mod protobuf {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Timestamp {
            #[prost(int64, tag = "1")]
            pub seconds: i64,
            #[prost(int32, tag = "2")]
            pub nanos: i32,
        }
    }
}

pub mod platform {
    pub mod product_catalog {
        pub mod v1 {
            use super::super::super::google::protobuf::Timestamp;

            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GetActiveProductRequest {
                #[prost(string, tag = "1")]
                pub product_id: String,
            }

            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ResolveActiveDomainRequest {
                #[prost(string, tag = "1")]
                pub domain_name: String,
            }

            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GetPublishedConfigurationRequest {
                #[prost(string, tag = "1")]
                pub product_id: String,
                #[prost(uint32, tag = "2")]
                pub configuration_version: u32,
            }

            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ProductCatalogEntry {
                #[prost(string, tag = "1")]
                pub product_id: String,
                #[prost(string, tag = "2")]
                pub product_key: String,
                #[prost(string, tag = "3")]
                pub display_name: String,
                #[prost(string, tag = "4")]
                pub brand_name: String,
                #[prost(uint32, tag = "5")]
                pub configuration_version: u32,
                #[prost(string, tag = "6")]
                pub configuration_schema_version: String,
                #[prost(message, repeated, tag = "7")]
                pub domains: Vec<ProductDomain>,
                #[prost(message, repeated, tag = "8")]
                pub markets: Vec<ProductMarket>,
                #[prost(message, repeated, tag = "9")]
                pub surfaces: Vec<ProductSurface>,
                #[prost(message, repeated, tag = "10")]
                pub capabilities: Vec<ProductCapability>,
                #[prost(message, optional, tag = "11")]
                pub published_at: Option<Timestamp>,
            }

            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ProductConfiguration {
                #[prost(string, tag = "1")]
                pub product_id: String,
                #[prost(uint32, tag = "2")]
                pub version: u32,
                #[prost(string, tag = "3")]
                pub schema_version: String,
                #[prost(bytes = "vec", tag = "4")]
                pub canonical_json: Vec<u8>,
                #[prost(message, optional, tag = "5")]
                pub published_at: Option<Timestamp>,
            }

            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ProductDomain {
                #[prost(string, tag = "1")]
                pub domain_name: String,
                #[prost(string, tag = "2")]
                pub status: String,
            }

            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ProductMarket {
                #[prost(string, tag = "1")]
                pub country_code: String,
                #[prost(string, tag = "2")]
                pub currency_code: String,
                #[prost(string, tag = "3")]
                pub locale: String,
                #[prost(string, tag = "4")]
                pub status: String,
            }

            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ProductSurface {
                #[prost(string, tag = "1")]
                pub surface_code: String,
                #[prost(uint32, tag = "2")]
                pub configuration_version: u32,
                #[prost(string, tag = "3")]
                pub status: String,
            }

            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ProductCapability {
                #[prost(string, tag = "1")]
                pub capability_code: String,
                #[prost(string, tag = "2")]
                pub contract_version: String,
                #[prost(string, tag = "3")]
                pub implementation_status: String,
                #[prost(string, tag = "4")]
                pub product_status: String,
            }

            pub type BoxFuture<T> =
                std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'static>>;

            pub trait ProductCatalog: Send + Sync + 'static {
                fn get_active_product(
                    &self,
                    request: tonic::Request<GetActiveProductRequest>,
                ) -> BoxFuture<Result<tonic::Response<ProductCatalogEntry>, tonic::Status>>;

                fn resolve_active_domain(
                    &self,
                    request: tonic::Request<ResolveActiveDomainRequest>,
                ) -> BoxFuture<Result<tonic::Response<ProductCatalogEntry>, tonic::Status>>;

                fn get_published_configuration(
                    &self,
                    request: tonic::Request<GetPublishedConfigurationRequest>,
                ) -> BoxFuture<Result<tonic::Response<ProductConfiguration>, tonic::Status>>;
            }

            #[derive(Debug, Clone)]
            pub struct ProductCatalogClient<T> {
                inner: tonic::client::Grpc<T>,
            }

            impl ProductCatalogClient<tonic::transport::Channel> {
                pub async fn connect<D>(destination: D) -> Result<Self, tonic::transport::Error>
                where
                    D: TryInto<tonic::transport::Endpoint>,
                    D::Error: Into<tonic::codegen::StdError>,
                {
                    let connection = tonic::transport::Endpoint::new(destination)?
                        .connect()
                        .await?;
                    Ok(Self::new(connection))
                }
            }

            impl<T> ProductCatalogClient<T>
            where
                T: tonic::client::GrpcService<tonic::body::Body>,
                T::Error: Into<tonic::codegen::StdError>,
                T::ResponseBody: tonic::codegen::Body<Data = tonic::codegen::Bytes>
                    + Send
                    + 'static,
                <T::ResponseBody as tonic::codegen::Body>::Error:
                    Into<tonic::codegen::StdError> + Send,
            {
                pub fn new(inner: T) -> Self {
                    Self {
                        inner: tonic::client::Grpc::new(inner),
                    }
                }

                pub async fn get_active_product(
                    &mut self,
                    request: GetActiveProductRequest,
                ) -> Result<tonic::Response<ProductCatalogEntry>, tonic::Status> {
                    self.inner.ready().await.map_err(|error| {
                        tonic::Status::unknown(format!(
                            "PLATFORM ProductCatalog service was not ready: {}",
                            error.into()
                        ))
                    })?;
                    self.inner
                        .unary(
                            tonic::Request::new(request),
                            http::uri::PathAndQuery::from_static(
                                "/ensambria.platform.productcatalog.v1.ProductCatalog/GetActiveProduct",
                            ),
                            tonic_prost::ProstCodec::default(),
                        )
                        .await
                }

                pub async fn resolve_active_domain(
                    &mut self,
                    request: ResolveActiveDomainRequest,
                ) -> Result<tonic::Response<ProductCatalogEntry>, tonic::Status> {
                    self.inner.ready().await.map_err(|error| {
                        tonic::Status::unknown(format!(
                            "PLATFORM ProductCatalog service was not ready: {}",
                            error.into()
                        ))
                    })?;
                    self.inner
                        .unary(
                            tonic::Request::new(request),
                            http::uri::PathAndQuery::from_static(
                                "/ensambria.platform.productcatalog.v1.ProductCatalog/ResolveActiveDomain",
                            ),
                            tonic_prost::ProstCodec::default(),
                        )
                        .await
                }

                pub async fn get_published_configuration(
                    &mut self,
                    request: GetPublishedConfigurationRequest,
                ) -> Result<tonic::Response<ProductConfiguration>, tonic::Status> {
                    self.inner.ready().await.map_err(|error| {
                        tonic::Status::unknown(format!(
                            "PLATFORM ProductCatalog service was not ready: {}",
                            error.into()
                        ))
                    })?;
                    self.inner
                        .unary(
                            tonic::Request::new(request),
                            http::uri::PathAndQuery::from_static(
                                "/ensambria.platform.productcatalog.v1.ProductCatalog/GetPublishedConfiguration",
                            ),
                            tonic_prost::ProstCodec::default(),
                        )
                        .await
                }
            }

            #[derive(Debug)]
            pub struct ProductCatalogServer<T> {
                inner: Arc<T>,
            }

            impl<T> ProductCatalogServer<T> {
                pub fn new(inner: T) -> Self {
                    Self {
                        inner: Arc::new(inner),
                    }
                }
            }

            impl<T> Clone for ProductCatalogServer<T> {
                fn clone(&self) -> Self {
                    Self {
                        inner: Arc::clone(&self.inner),
                    }
                }
            }

            impl<T, B> Service<http::Request<B>> for ProductCatalogServer<T>
            where
                T: ProductCatalog,
                B: Body + Send + 'static,
                B::Error: Into<StdError> + Send + 'static,
            {
                type Response = http::Response<tonic::body::Body>;
                type Error = Infallible;
                type Future = Pin<
                    Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
                >;

                fn poll_ready(
                    &mut self,
                    _context: &mut Context<'_>,
                ) -> Poll<Result<(), Self::Error>> {
                    Poll::Ready(Ok(()))
                }

                fn call(&mut self, request: http::Request<B>) -> Self::Future {
                    match request.uri().path() {
                        "/ensambria.platform.productcatalog.v1.ProductCatalog/GetActiveProduct" => {
                            struct Method<T: ProductCatalog>(Arc<T>);
                            impl<T: ProductCatalog> tonic::server::UnaryService<GetActiveProductRequest>
                                for Method<T>
                            {
                                type Response = ProductCatalogEntry;
                                type Future = BoxFuture<
                                    Result<tonic::Response<Self::Response>, tonic::Status>,
                                >;

                                fn call(
                                    &mut self,
                                    request: tonic::Request<GetActiveProductRequest>,
                                ) -> Self::Future {
                                    self.0.get_active_product(request)
                                }
                            }
                            let inner = Arc::clone(&self.inner);
                            Box::pin(async move {
                                let mut grpc = tonic::server::Grpc::new(
                                    tonic_prost::ProstCodec::default(),
                                );
                                Ok(grpc.unary(Method(inner), request).await)
                            })
                        }
                        "/ensambria.platform.productcatalog.v1.ProductCatalog/ResolveActiveDomain" => {
                            struct Method<T: ProductCatalog>(Arc<T>);
                            impl<T: ProductCatalog> tonic::server::UnaryService<ResolveActiveDomainRequest>
                                for Method<T>
                            {
                                type Response = ProductCatalogEntry;
                                type Future = BoxFuture<
                                    Result<tonic::Response<Self::Response>, tonic::Status>,
                                >;

                                fn call(
                                    &mut self,
                                    request: tonic::Request<ResolveActiveDomainRequest>,
                                ) -> Self::Future {
                                    self.0.resolve_active_domain(request)
                                }
                            }
                            let inner = Arc::clone(&self.inner);
                            Box::pin(async move {
                                let mut grpc = tonic::server::Grpc::new(
                                    tonic_prost::ProstCodec::default(),
                                );
                                Ok(grpc.unary(Method(inner), request).await)
                            })
                        }
                        "/ensambria.platform.productcatalog.v1.ProductCatalog/GetPublishedConfiguration" => {
                            struct Method<T: ProductCatalog>(Arc<T>);
                            impl<T: ProductCatalog> tonic::server::UnaryService<GetPublishedConfigurationRequest>
                                for Method<T>
                            {
                                type Response = ProductConfiguration;
                                type Future = BoxFuture<
                                    Result<tonic::Response<Self::Response>, tonic::Status>,
                                >;

                                fn call(
                                    &mut self,
                                    request: tonic::Request<GetPublishedConfigurationRequest>,
                                ) -> Self::Future {
                                    self.0.get_published_configuration(request)
                                }
                            }
                            let inner = Arc::clone(&self.inner);
                            Box::pin(async move {
                                let mut grpc = tonic::server::Grpc::new(
                                    tonic_prost::ProstCodec::default(),
                                );
                                Ok(grpc.unary(Method(inner), request).await)
                            })
                        }
                        _ => Box::pin(async move {
                            Ok(http::Response::builder()
                                .status(200)
                                .body(tonic::body::Body::empty())
                                .expect("empty gRPC response must be constructible"))
                        }),
                    }
                }
            }

            impl<T: ProductCatalog> tonic::server::NamedService for ProductCatalogServer<T> {
                const NAME: &'static str = "ensambria.platform.productcatalog.v1.ProductCatalog";
            }
        }
    }
}
