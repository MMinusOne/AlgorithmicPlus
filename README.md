# AlgorithmicPlus 

## Overview

AlgorithmicPlus is a framework for developping, testing and optimizing financial algorithmic trading models,It is made with Rust on the backend and with TypeScript on the frontend, the frontend is made with React, And the whole application is made with Tauri, the backend and frontend communicate is handled by Tauri's way of IPC communication.

The over-all code architecture is meant to be as abstracted for people who just want to develop the strategies, meaning there's a bit more abstraction for a better developer expierence for making strategies.

## Why use it? 

Here's some very good reasons to use AlgorutmicPlus.

1) It's open source and free for ever.
2) The backtesting part is written in Rust which is by default a performant language, and the UI is made with the best frontend framework out there, React.
3) For people who want to make strategies only, it's very easy to use since there are abstractions that make developping the strategy itself easier.
4) It's performance oriented, alot of the parts of it are diligently optimized, which you won't get that result making your own backtesting engine in a few days, it's fast.
5) The code is structed in a way that makes it very easy to implement whatever you want.
6) It supports OHLCV, news, bidask data by default, and you can make it support any kind of data really.
7) The development process it-self is better than other frameworks, since the data is first downloaded, then it's pre-processed which we call composing the data, and only then is the strategy ran, meaning it would be very easy and performant to incorportate your data structures before backtesting, for example, making a strategy that has multiple pairs, multiple timeframes, bid-ask, news data and OHLCV data at the same time is no issue here, since it supports composing the data before backtesting.
8) Im really focused on optimization, I used every trick in the book, it's multi-threaded, uses memory-mapped files, efficient data structures, O(1) implementations of technical indicators.

So over-all, it's hella performant, supported, low-barrier to entry, you can do ANYTHING, and has alot of future plans.

## How do I use it?

Currently there's no documentation on how to use this, be patient heh, Im working myself to the bone here.
But if you are really dedicated and want to use this, you can go take a look at the code, and everything for developping a strategy would happen in the `user` folder.

## Who made it?

MMinusOne, or me :3, started it solo, as a project that I will use personally and that would be good on my resume.

## How can I contribute?

Again, no documentation on how the code works right now, I hope Im able to make it soon.

## Current phase

It's still lacking alot of features, the only features currently available are to download the data and to visualize it, the feature for composing data is coming very soon though.

## Future changes and plans

### Todo List  
- ❌ Fix Binance provider.
- ❌ Finish the data composer
- ❌ Implement technical indicators
- ❌ Implement standard backtesting 
- ❌ Implement optimization Algorithmic
- ❌ GUI Settings Menu 
- ❌ Re-write performance-sensitive parts in C++ 
- ❌ Make the code documentation 
- ❌ Implement more data providers. 
- ❌ Make the metric formulas.

I have plans to keep working on this.




