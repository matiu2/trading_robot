# Oanda client

Rust client for [the oanda broker REST api](https://developer.oanda.com/rest-live-v20/development-guide/).

We [model](https://github.com/matiu2/trading_robot/blob/main/oanda/src/model.rs) most of the JSON types oanda provides.

Becuase the oanda types have no way to express things like enums, many structs have a rust version and an oanda version. The rust version will usually have a builder, blocking mistakes at compile time.

This can be best seen [in the stop loss model](https://github.com/matiu2/trading_robot/blob/main/oanda/src/model/transaction/stop_loss.rs).

In oanda it has a `Option<price>` and `Option<distance>` and one or the other should be set. In the rust struct, we handle this using an enum.

This wasn't always the way and some models like [trade](https://github.com/matiu2/trading_robot/blob/main/oanda/src/model/trade.rs) still need to have builders added and other logic.

## Usage

Check out some of the tests or the main trader itself, but here's an example, to list the instruments under an account:

```
        let simple = client
            .accounts()
            .list_instruments(&account_id)
            .send()
            .await
            .unwrap();
```
