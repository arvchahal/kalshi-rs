
# Full Kalshi API → kalshi-rs Mapping Table

| API Endpoint                                               | kalshi-rs Function                     | Module                      | Status          |
| ---------------------------------------------------------- | -------------------------------------- | --------------------------- | --------------- |
| Get Exchange Status                                        | `get_exchange_status()`                | exchange/endpoints.rs       | ✅ Implemented  |
| Get Exchange Announcements                                 | `get_exchange_announcements()`         | exchange/endpoints.rs       | ✅ Implemented  |
| Get Series Fee Changes                                     |                                        |                             | ❌ Not Implemented |
| Get Exchange Schedule                                      | `get_exchange_schedule()`              | exchange/endpoints.rs       | ✅ Implemented  |
| Get User Data Timestamp                                    | `get_user_data_timestamp()`            | exchange/endpoints.rs       | ✅ Implemented  |
| Get Order Groups                                           | `get_order_groups()`                   | portfolio/endpoints.rs      | ✅ Implemented  |
| Create Order Group                                         | `create_order_group()`                 | portfolio/endpoints.rs      | ✅ Implemented  |
| Get Order Group                                            | `get_order_group()`                    | portfolio/endpoints.rs      | ✅ Implemented  |
| Delete Order Group                                         | `delete_order_group()`                 | portfolio/endpoints.rs      | ✅ Implemented  |
| Reset Order Group                                          | `reset_order_group()`                  | portfolio/endpoints.rs      | ✅ Implemented  |
| Get Orders                                                 | `get_orders()`                         | portfolio/endpoints.rs      | ✅ Implemented  |
| Create Order                                               | `create_order()`                       | portfolio/endpoints.rs      | ✅ Implemented  |
| Batch Create Orders                                        | `batch_create_orders()`                | portfolio/endpoints.rs      | ✅ Implemented  |
| Batch Cancel Orders                                        | `batch_cancel_orders()`                | portfolio/endpoints.rs      | ✅ Implemented  |
| Get Queue Positions for Orders                             | `get_queue_positions()`                | portfolio/endpoints.rs      | ✅ Implemented  |
| Get Order                                                  | `get_order()`                          | portfolio/endpoints.rs      | ✅ Implemented  |
| Cancel Order                                               | `cancel_order()`                       | portfolio/endpoints.rs      | ✅ Implemented  |
| Amend Order                                                | `amend_order()`                        | portfolio/endpoints.rs      | ✅ Implemented  |
| Decrease Order                                             | `decrease_order()`                     | portfolio/endpoints.rs      | ✅ Implemented  |
| Get Order Queue Position                                   | `get_order_queue_position()`           | portfolio/endpoints.rs      | ✅ Implemented  |
| Get Balance                                                | `get_balance()`                        | portfolio/endpoints.rs      | ✅ Implemented  |
| Get Positions                                              | `get_positions()`                      | portfolio/endpoints.rs      | ✅ Implemented  |
| Get Settlements                                            | `get_settlements()`                    | portfolio/endpoints.rs      | ✅ Implemented  |
| Get Total Resting Order Value                              | `get_total_resting_order_value()`      | portfolio/endpoints.rs      | ✅ Implemented  |
| Get Fills                                                  | `get_fills()`                          | portfolio/endpoints.rs      | ✅ Implemented  |
| Get API Keys                                               | `get_api_keys()`                       | api_keys/endpoints.rs       | ✅ Implemented  |
| Create API Key                                             |                                        |                             | ❌ Not Implemented |
| Generate API Key                                           | `generate_api_key()`                   | api_keys/endpoints.rs       | ✅ Implemented  |
| Delete API Key                                             | `delete_api_key()`                     | api_keys/endpoints.rs       | ✅ Implemented  |
| Get Tags for Series Categories                             |                                        |                             | ❌ Not Implemented |
| Get Filters for Sports                                     |                                        |                             | ❌ Not Implemented |
| Get Market Candlesticks                                    | `get_market_candlesticks()`            | markets/endpoints.rs        | ✅ Implemented  |
| Get Trades                                                 | `get_trades()`                         | markets/endpoints.rs        | ✅ Implemented  |
| Get Market Orderbook                                       | `get_market_orderbook()`               | markets/endpoints.rs        | ✅ Implemented  |
| Get Series                                                 | `get_series_by_ticker()`               | series/endpoints.rs         | ✅ Implemented  |
| Get Series List                                            | `get_all_series()`                     | series/endpoints.rs         | ✅ Implemented  |
| Get Markets                                                | `get_all_markets()`                    | markets/endpoints.rs        | ✅ Implemented  |
| Get Market                                                 | `get_market()`                         | markets/endpoints.rs        | ✅ Implemented  |
| Get Event Candlesticks                                     |                                        |                             | ❌ Not Implemented |
| Get Events                                                 | `get_all_events()`                     | events/endpoints.rs         | ✅ Implemented  |
| Get Multivariate Events                                    |                                        |                             | ❌ Not Implemented |
| Get Event                                                  | `get_event()`                          | events/endpoints.rs         | ✅ Implemented  |
| Get Event Metadata                                         | `get_event_metadata()`                 | events/endpoints.rs         | ✅ Implemented  |
| Get Event Forecast Percentile History                      |                                        |                             | ❌ Not Implemented |
| Get Live Data                                              |                                        |                             | ❌ Not Implemented |
| Get Multiple Live Data                                     |                                        |                             | ❌ Not Implemented |
| Get Volume Incentives                                      |                                        |                             | ❌ Not Implemented |
| Get FCM Orders                                             |                                        |                             | ❌ Not Implemented |
| Get FCM Positions                                          |                                        |                             | ❌ Not Implemented |
| Get Structured Targets                                     | `get_all_structured_targets()`         | structured_targets/endpoints.rs | ✅ Implemented  |
| Get Structured Target                                      | `get_structured_target()`              | structured_targets/endpoints.rs | ✅ Implemented  |
| Get Milestone                                              | `get_milestone()`                      | milestones/endpoints.rs     | ✅ Implemented  |
| Get Milestones                                             | `get_milestones()`                     | milestones/endpoints.rs     | ✅ Implemented  |
| Get Communications ID                                      | `get_communications_id()`              | communications/endpoints.rs | ✅ Implemented  |
| Get RFQs                                                   | `get_rfqs()`                           | communications/endpoints.rs | ✅ Implemented  |
| Create RFQ                                                 | `create_rfq()`                         | communications/endpoints.rs | ✅ Implemented  |
| Get RFQ                                                    | `get_rfq()`                            | communications/endpoints.rs | ✅ Implemented  |
| Delete RFQ                                                 | `delete_rfq()`                         | communications/endpoints.rs | ✅ Implemented  |
| Get Quotes                                                 | `get_quotes()`                         | communications/endpoints.rs | ✅ Implemented  |
| Create Quote                                               | `create_quote()`                       | communications/endpoints.rs | ✅ Implemented  |
| Get Quote                                                  | `get_quote()`                          | communications/endpoints.rs | ✅ Implemented  |
| Delete Quote                                               | `delete_quote()`                       | communications/endpoints.rs | ✅ Implemented  |
| Accept Quote                                               | `accept_quote()`                       | communications/endpoints.rs | ✅ Implemented  |
| Confirm Quote                                              | `confirm_quote()`                      | communications/endpoints.rs | ✅ Implemented  |
| Get Multivariate Event Collection                          | `get_multivariate_event_collection()`  | multivariate_collections/endpoints.rs | ✅ Implemented  |
| Create Market In Multivariate Event Collection             |                                        |                             | ❌ Not Implemented |
| Get Multivariate Event Collections                         | `get_multivariate_event_collections()` | multivariate_collections/endpoints.rs | ✅ Implemented  |
| Get Multivariate Event Collection Lookup History           |                                        |                             | ❌ Not Implemented |
| Lookup Tickers For Market In Multivariate Event Collection |                                        |                             | ❌ Not Implemented |


## Summary

- **Total API Endpoints**: 75
- ** Implemented**: 58 
- ** Not Implemented**: 17 

## Not Implemented Endpoints

These endpoints are available in the Kalshi API but not yet implemented in kalshi-rs:

1. **Exchange**: Get Series Fee Changes
2. **API Keys**: Create API Key (use `generate_api_key()` instead)
3. **Search**: Get Tags for Series Categories, Get Filters for Sports
4. **Events**: Get Event Candlesticks, Get Multivariate Events, Get Event Forecast Percentile History
5. **Live Data**: Get Live Data, Get Multiple Live Data ie websockets
6. **Incentives**: Get Volume Incentives
7. **FCM**: Get FCM Orders, Get FCM Positions
8. **Multivariate**: Create Market In Multivariate Event Collection, Get Multivariate Event Collection Lookup History, Lookup Tickers For Market In Multivariate Event Collection

## Contributing

If you'd like to implement any of the missing endpoints, contributions are welcome! See the [src/README.md](src/README.md) for details on how to add new endpoints.
