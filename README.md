# Nothing to see here

This is a private repo, but I've made it public to showcase some of my work.

Eventually I may release the oanda client part.

License: You can look at the code in github. Don't copy it. Don't sue me.

## To potential recruiters

This repo shows off my latest rust skills. It's a work in progress and I never planned to actually share it with the world.

It doesn't show my collaboration skills. I would program and comment differently in collaboritive environment.

## Overview

This is a trading robot in progress. There are 

The main part of the robot [is here](https://github.com/matiu2/trading_robot/blob/main/trader/src/main.rs). So far all it does is download some candles, run some algorithms, and is still deciding if it wants to enter a trade. 

Packages:

 * [oanda](https://github.com/matiu2/trading_robot/tree/main/oanda) - There isn't a good rust client for oanda, so I'm writing it myself. I may release this part once it's more complete.
 * [algorithms](https://github.com/matiu2/trading_robot/tree/main/algorithms) - A small library of trading algorithms.
 * [trader](https://github.com/matiu2/trading_robot/blob/main/trader/src/main.rs) - The binary that will eventually run periodically and open, maintain and close my trades