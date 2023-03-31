## Some trading algorithms

All the algorithms use [traits](https://github.com/matiu2/trading_robot/blob/main/algorithms/src/candle.rs) for parts of candes (like, high, low, open and close) as input.
This way the algorithms are kept simple and pure, testing is easy.

Most of our internal Candle structures implement these traits already, including [the oanda model](https://github.com/matiu2/trading_robot/blob/main/oanda/src/model/candle/algorithms_compat.rs).
If you need to override it, just use [the NewType pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html).

If you're bringing your own candle model, just implement the traits for it.