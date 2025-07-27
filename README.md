## Exchange

An ongoing implementation of an exchange using limit order boook

### Implementation details

#### Order Book Data Structure 

Order book will use a Linked List where each item will be a FIFO queue. 

Linked Lists are ordered, which is something I need for best bid and asks.
For each price level, a FIFO will store the orders as they arrive (time preference).

I found this solution a good one for a first implementation. 
In the future I can change it to something better.

### Testing ideas

I found [some datasets](https://lobsterdata.com/info/DataSamples.php) which I can
use to mimic orders arriving at the exchange.

My idea is to stress the application, measure and improve performance as a iterative process.

### TODO

- [x] write unit tests for Price Level (FIFO Queue)
- [ ] Bid/Ask Linked Lists implementation (add, update, cancel) and unit tests
- [ ] Order Book implementation (add, update, cancel) and unit tests
- [ ] Order Matching implementation and unit tests
- [ ] interface for external calling (http, unix socket)
- [ ] app/script that reads historical dataset and call the Exchange to register orders
