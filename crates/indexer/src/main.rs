#![allow(non_snake_case)]

#[macro_export]
macro_rules! generate_event_query_root {
    (
        $(
            $event:ident { $($field:ident),* }
        ),* $(,)?
    ) => {
        use paste::paste;
        use async_graphql::*;
        use std::{collections::HashMap, sync::Arc};
        use tokio::sync::RwLock;

        paste! {
            $(
                #[derive(SimpleObject, Clone, Debug)]
                pub struct [<$event GQL>] {
                    // TODO: figure out how to add idParam
                    pub id: ID,
                    $(pub $field: String,)*
                    pub timestamp_: u64,
                    pub transactionHash_: String,
                }
            )*

            #[derive(Debug)]
            pub enum EventEnum {
                $(
                    $event($event),
                )*
            }

            #[derive(Debug)]
            pub struct EventDetails {
                id: ID,
                event: EventEnum,
                timestamp: u64,
                transactionHash: String,
            }

            pub type EventList = Arc<RwLock<HashMap<ID, EventDetails>>>;
            pub type EventStore = Arc<RwLock<HashMap<&'static str, EventList>>>;

            pub struct QueryRoot;

            #[Object]
            impl QueryRoot {
                $(
                    #[allow(non_snake_case)]
                    async fn [<$event s>](&self, ctx: &Context<'_>) -> Option<Vec<[<$event GQL>]>> {
                        let store = ctx.data_unchecked::<EventStore>();
                        let map = store.read().await;
                        if let Some(list) = map.get(stringify!($event)) {
                            let events = list.read().await;
                            Some(events.values().filter_map(|e| {
                                let EventEnum::$event(inner) = &e.event;
                                Some([<$event GQL>] {
                                    id: e.id.clone(),
                                    $(
                                        $field: format!("{:?}", inner.$field),
                                    )*
                                    timestamp_: e.timestamp.clone(),
                                    transactionHash_: e.transactionHash.clone(),
                                })
                            }).collect())
                        } else {
                            None
                        }
                    }

                    async fn $event(&self, ctx: &Context<'_>, id: ID) -> Option<[<$event GQL>]> {
                        let store = ctx.data_unchecked::<EventStore>();
                        let map = store.read().await;
                        if let Some(list) = map.get(stringify!($event)) {
                            let events = list.read().await;
                            if let Some(e) = events.get(&id) {
                                let EventEnum::$event(inner) = &e.event;
                                Some([<$event GQL>] {
                                    id: e.id.clone(),
                                    $(
                                        $field: format!("{:?}", inner.$field),
                                    )*
                                    timestamp_: e.timestamp.clone(),
                                    transactionHash_: e.transactionHash.clone(),
                                })
                            } else {
                                None
                            }
                        } else {
                            None
                        }

                    }
                )*
            }

            #[async_trait::async_trait]
            pub trait EventHandler {
                async fn handle<P: Provider>(provider: Arc<P>, addr: Address, store: EventStore);
            }

            $(
                #[async_trait::async_trait]
                impl EventHandler for $event {
                    async fn handle<P: Provider>(provider: Arc<P>, addr: Address, store: EventStore) {
                        let event = alloy::contract::Event::<(), Arc<P>, $event>::new(
                            provider,
                            Filter::new().address(addr),
                        );
                        let from_block = event.from_block(0u64);
                        let mut stream = from_block.watch().await.unwrap().into_stream();

                        while let Some(Ok((event, log))) = stream.next().await {
                            let mut map = store.write().await;
                            let entry = map
                                .entry(stringify!($event))
                                .or_insert_with(|| Arc::new(RwLock::new(HashMap::new())));
                            let tx_hash = log.transaction_hash.unwrap().to_string();
                            entry.write().await.insert(
                                ID(tx_hash.clone()),
                                EventDetails {
                                    id: ID(tx_hash.clone()),
                                    event: EventEnum::$event(event),
                                    timestamp: log.block_timestamp.unwrap(),
                                    transactionHash: tx_hash,
                                },
                            );
                        }
                    }
                }
            )*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::rpc::types::Filter;
    use alloy::{
        network::EthereumWallet, node_bindings::Anvil, primitives::U256,
        providers::ProviderBuilder, signers::local::PrivateKeySigner,
    };
    use alloy::{primitives::Address, providers::Provider};
    use async_graphql_warp::GraphQLResponse;
    use bindings::orderbook::OrderBook::{self, SubmitOrder};
    use futures::StreamExt;
    use graphql_client::{GraphQLQuery, Response};
    use warp::{http::Response as HttpResponse, Filter as _};

    generate_event_query_root!(SubmitOrder { orderId, sender, amount });

    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "src/orders_schema.graphql",
        query_path = "src/query_orders.graphql",
        response_derives = "Debug"
    )]
    pub struct GetOrders;

    impl GetOrders {
        pub async fn fetch_events() -> anyhow::Result<Vec<get_orders::GetOrdersSubmitOrders>> {
            let client = reqwest::Client::new();
            let variables = get_orders::Variables {};
            let request_body = GetOrders::build_query(variables);
            let res = client.post("http://localhost:8000").json(&request_body).send().await?;
            let response_body: Response<get_orders::ResponseData> = res.json().await?;
            println!("Response: {:?}", response_body);
            Ok(response_body.data.unwrap().submit_orders)
        }
    }

    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "src/orders_schema.graphql",
        query_path = "src/query_order_by_id.graphql",
        response_derives = "Debug"
    )]
    pub struct GetOrderById;

    impl GetOrderById {
        pub async fn fetch_event(
            id: String,
        ) -> anyhow::Result<Option<get_order_by_id::GetOrderByIdSubmitOrder>> {
            let client = reqwest::Client::new();
            let variables = get_order_by_id::Variables { id };
            let request_body = GetOrderById::build_query(variables);
            let res = client.post("http://localhost:8000").json(&request_body).send().await?;
            let response_body: Response<get_order_by_id::ResponseData> = res.json().await?;
            println!("Response: {:?}", response_body);
            Ok(response_body.data.unwrap().submit_order)
        }
    }

    #[tokio::test]
    async fn test_e2e() -> anyhow::Result<()> {
        let store: EventStore = Arc::new(RwLock::new(HashMap::new()));

        let anvil = Anvil::new().spawn();
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer);
        let provider = ProviderBuilder::new().wallet(wallet).on_http(anvil.endpoint().parse()?);
        let provider = Arc::new(provider);

        let order_book = OrderBook::deploy(provider.clone()).await?;
        let order_book_addr = order_book.address().clone();

        let store_clone = store.clone();
        let provider_clone = provider.clone();

        tokio::spawn(async move {
            SubmitOrder::handle(provider_clone, order_book_addr, store_clone).await;
        });

        let schema =
            Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(store.clone()).finish();

        let graphql_post = async_graphql_warp::graphql::<
            Schema<QueryRoot, EmptyMutation, EmptySubscription>,
        >(schema)
        .and_then(
            |(schema, request): (
                Schema<QueryRoot, EmptyMutation, EmptySubscription>,
                async_graphql::Request,
            )| async move {
                Ok::<GraphQLResponse, std::convert::Infallible>(GraphQLResponse::from(
                    schema.execute(request).await,
                ))
            },
        );

        let graphql_playground = warp::path("playground").map(|| {
            HttpResponse::builder().header("content-type", "text/html").body(
                async_graphql::http::playground_source(
                    async_graphql::http::GraphQLPlaygroundConfig::new("/"),
                ),
            )
        });

        println!("GraphQL Playground: http://localhost:8000/playground");

        let server_handle = tokio::spawn(async move {
            warp::serve(graphql_post.or(graphql_playground)).run(([127, 0, 0, 1], 8000)).await;
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let _receipt = order_book.submitOrder(U256::from(100)).send().await?.get_receipt().await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let events = GetOrders::fetch_events().await?;
        println!("Fetched events: {:?}", events);

        assert!(!events.is_empty(), "Expected at least one event");

        let id = events[0].id.clone();
        let event = GetOrderById::fetch_event(id).await?;
        println!("Fetched event by ID: {:?}", event);

        // server_handle.await?;
        server_handle.abort();

        Ok(())
    }
}
