use std::{
    collections::{LinkedList, VecDeque},
    usize,
};

// #[derive(Debug, Clone, Copy)]
#[derive(Debug)]
struct Order {
    pub id: u16,
    pub price: u16,
    pub size: u16,
}

#[derive(Debug)]
struct Queue {
    items: VecDeque<Order>,
}

impl Queue {
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    fn new_with_order(order: Order) -> Self {
        Self {
            items: VecDeque::from([order]),
        }
    }

    fn add(&mut self, order: Order) {
        self.items.push_back(order);
    }

    fn dequeue(&mut self) -> Option<Order> {
        self.items.pop_front()
    }

    fn get(&self) -> Option<&Order> {
        self.items.get(0)
    }

    fn get_mut(&mut self) -> Option<&mut Order> {
        self.items.get_mut(0)
    }

    fn can_consume(&self, amount: u16) -> Vec<Order> {
        let mut remaining: u16 = amount;

        let mut result: Vec<Order> = Vec::new();

        let mut items = self.items.iter();

        while remaining > 0 {
            match items.next() {
                None => break,
                Some(next_item) => match next_item.size <= remaining {
                    true => {
                        remaining -= next_item.size;

                        result.push(Order {
                            id: next_item.id,
                            price: next_item.price,
                            size: next_item.size,
                        });
                    }
                    false => {
                        result.push(Order {
                            id: next_item.id,
                            price: next_item.price,
                            size: remaining,
                        });

                        remaining = 0;
                    }
                },
            }
        }
        result
    }

    fn consume(&mut self, amount: u16, buf: &mut Vec<Order>) -> u16 {
        let mut remaining: u16 = amount;

        while remaining > 0 {
            if self.get().is_none() {
                break;
            }
            let next_item = self.get_mut().unwrap();
            match next_item.size <= remaining {
                true => {
                    remaining -= next_item.size;

                    if let Some(order) = self.dequeue() {
                        buf.push(order);
                    }
                }
                false => {
                    next_item.size -= remaining;

                    buf.push(Order {
                        id: next_item.id,
                        price: next_item.price,
                        size: remaining,
                    });

                    remaining = 0;
                }
            }
        }
        amount - remaining
    }
}

#[derive(Debug)]
struct PriceLevel {
    price: u16,
    volume: u16,
    queue: Queue,
}

impl PriceLevel {
    fn new(order: Order) -> Self {
        Self {
            price: order.price,
            volume: order.size,
            queue: Queue::new_with_order(order),
        }
    }

    fn consume(&mut self, amount: u16, buf: &mut Vec<Order>) -> u16 {
        let consumed_volume = self.queue.consume(amount, buf);
        self.volume -= consumed_volume;
        consumed_volume
    }
}

#[derive(Debug)]
struct Book {
    last_consumed_orders: Vec<Order>,
    highest_bid: u16,
    lowest_ask: u16,
    bid: LinkedList<PriceLevel>,
    ask: LinkedList<PriceLevel>,
}

enum AddOrderErrors {
    InsufficientMatch,
}

impl Book {
    fn new() -> Self {
        Self {
            highest_bid: 0,
            lowest_ask: 0,
            bid: LinkedList::new(),
            ask: LinkedList::new(),
            last_consumed_orders: vec![],
        }
    }

    fn ask_price_levels_count(&self) -> usize {
        self.ask.len()
    }

    fn bid_price_levels_count(&self) -> usize {
        self.bid.len()
    }

    fn match_ask(&mut self, order: &Order) {
        let mut remaining = order.size;
        let mut bids = self.bid.iter_mut();
        let mut delete_lvls_count = 0;

        // TODO: I must somehow return the ids of the matched orders
        while remaining > 0 {
            match bids.next() {
                None => break,
                Some(item) => {
                    remaining -= item.consume(remaining, &mut self.last_consumed_orders);

                    if item.volume == 0 {
                        delete_lvls_count += 1;
                    }
                }
            }
        }

        if delete_lvls_count > 0 {
            for _ in 0..delete_lvls_count {
                self.bid.pop_front();
            }
        }

        self.update_best_values();
    }

    fn update_best_values(&mut self) {
        if let Some(highest_bid_price_lvl) = self.bid.front() {
            self.highest_bid = highest_bid_price_lvl.price;
        } else {
            self.highest_bid = 0;
        }

        if let Some(lowest_ask_price_lvl) = self.ask.front() {
            self.lowest_ask = lowest_ask_price_lvl.price;
        } else {
            self.lowest_ask = 0;
        }
    }

    fn can_match_ask(&mut self, order: &Order) -> bool {
        let mut remaining = order.size;
        let mut bids = self.bid.iter();

        while remaining > 0 {
            match bids.next() {
                None => break,
                Some(item) => {
                    if item.price < order.price {
                        break;
                    }

                    remaining -= item
                        .queue
                        .can_consume(remaining)
                        .iter()
                        .map(|x| x.size)
                        .sum::<u16>();
                }
            }
        }

        if remaining > 0 {
            return false;
        }

        return true;
    }

    fn add_ask(&mut self, order: Order) -> Result<(), AddOrderErrors> {
        if order.price <= self.highest_bid {
            if self.can_match_ask(&order) {
                self.match_ask(&order);

                return Ok(());
            } else {
                return Err(AddOrderErrors::InsufficientMatch);
            }
        }

        if let Some(lowest_ask_item) = self.ask.front_mut() {
            match self.lowest_ask.cmp(&order.price) {
                std::cmp::Ordering::Less => {
                    // nothing
                }
                std::cmp::Ordering::Equal => {
                    lowest_ask_item.queue.add(order);
                    return Ok(());
                }
                std::cmp::Ordering::Greater => {
                    self.lowest_ask = order.price;
                    self.ask.push_front(PriceLevel::new(order));
                    return Ok(());
                }
            };
        }

        for price_level in self.ask.iter_mut() {
            if price_level.price == order.price {
                price_level.queue.add(order);
                return Ok(());
            } else {
                // go to a higher ask price level
            }
        }

        if self.ask.front().is_none() {
            self.lowest_ask = order.price;
        }

        self.ask.push_back(PriceLevel::new(order));

        Ok(())
    }

    fn add_bid(&mut self, order: Order) {
        if let Some(highest_bid_item) = self.bid.front_mut() {
            match self.highest_bid.cmp(&order.price) {
                std::cmp::Ordering::Less => {
                    self.highest_bid = order.price;
                    self.bid.push_front(PriceLevel::new(order));
                    return;
                }
                std::cmp::Ordering::Equal => {
                    highest_bid_item.queue.add(order);
                    return;
                }
                std::cmp::Ordering::Greater => {
                    // nothing
                }
            };
        }

        for price_level in self.bid.iter_mut() {
            if price_level.price == order.price {
                price_level.queue.add(order);
                return;
            } else {
                // go to a lower bid price level
            }
        }

        if self.bid.front().is_none() {
            self.highest_bid = order.price;
        }

        self.bid.push_back(PriceLevel::new(order));
    }
}

fn main() {
    let mut book = Book {
        last_consumed_orders: vec![],
        highest_bid: 40,
        bid: LinkedList::from([PriceLevel::new(Order {
            price: 40,
            size: 100,
            id: 1,
        })]),
        lowest_ask: 50,
        ask: LinkedList::from([PriceLevel::new(Order {
            price: 50,
            size: 100,
            id: 2,
        })]),
    };

    let a = Order {
        id: 3,
        size: 100,
        price: 40,
    };

    book.add_ask(a);

    // println!("Result: {:?}", result);
    // println!("Queue: {:?}", queue);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_queue() {
        let mut queue = Queue::new();

        let a = Order {
            id: 1,
            size: 100,
            price: 100,
        };
        let b = Order {
            id: 2,
            size: 200,
            price: 100,
        };
        let c = Order {
            id: 3,
            size: 50,
            price: 100,
        };
        let d = Order {
            id: 4,
            size: 150,
            price: 100,
        };

        queue.add(a);
        queue.add(b);
        queue.add(c);
        queue.add(d);

        let mut buf = vec![];
        queue.consume(375, &mut buf);
        let remaining_item = queue.get().unwrap();

        assert_eq!(buf.len(), 4);
        assert_eq!(buf.iter().map(|item| item.size).sum::<u16>(), 375);

        assert_eq!(remaining_item.id, 4);
        assert_eq!(remaining_item.size, 125);
    }

    // ####################################
    // BID TESTS
    // ####################################

    #[test]
    fn add_first_bid() {
        let mut book = Book::new();

        let a = Order {
            id: 1,
            size: 100,
            price: 100,
        };
        let b = Order {
            id: 2,
            size: 200,
            price: 100,
        };

        book.add_bid(a);
        book.add_bid(b);

        assert_eq!(book.highest_bid, 100);
        assert_eq!(book.bid_price_levels_count(), 1);

        let price_level = book.bid.front().unwrap();
        assert_eq!(price_level.queue.items.len(), 2);
        assert_eq!(
            price_level.queue.items.iter().map(|i| i.size).sum::<u16>(),
            300
        );
    }

    #[test]
    fn higher_bid_price_level() {
        let mut book = Book {
            last_consumed_orders: vec![],
            lowest_ask: 0,
            highest_bid: 50,
            ask: LinkedList::new(),
            bid: LinkedList::from([PriceLevel::new(Order {
                price: 50,
                size: 100,
                id: 1,
            })]),
        };

        let a = Order {
            id: 2,
            size: 100,
            price: 100,
        };
        let b = Order {
            id: 3,
            size: 200,
            price: 100,
        };

        book.add_bid(a);
        book.add_bid(b);

        assert_eq!(book.highest_bid, 100);
        assert_eq!(book.bid_price_levels_count(), 2);

        let highest_price_level = book.bid.front().unwrap();
        assert_eq!(highest_price_level.queue.items.len(), 2);
        assert_eq!(
            highest_price_level
                .queue
                .items
                .iter()
                .map(|i| i.size)
                .sum::<u16>(),
            300
        );
    }

    #[test]
    fn add_lower_bid_price_level() {
        let mut book = Book {
            last_consumed_orders: vec![],
            lowest_ask: 0,
            highest_bid: 500,
            ask: LinkedList::new(),
            bid: LinkedList::from([PriceLevel::new(Order {
                price: 500,
                size: 100,
                id: 1,
            })]),
        };

        let a = Order {
            id: 2,
            size: 100,
            price: 100,
        };
        let b = Order {
            id: 3,
            size: 200,
            price: 100,
        };

        book.add_bid(a);
        book.add_bid(b);

        assert_eq!(book.highest_bid, 500);
        assert_eq!(book.bid_price_levels_count(), 2);

        let lowest_price_level = book.bid.back().unwrap();
        assert_eq!(lowest_price_level.queue.items.len(), 2);
        assert_eq!(
            lowest_price_level
                .queue
                .items
                .iter()
                .map(|i| i.size)
                .sum::<u16>(),
            300
        );
    }

    // ####################################
    // ASK TESTS
    // ####################################

    #[test]
    fn add_first_ask() {
        let mut book = Book::new();

        let a = Order {
            id: 1,
            size: 100,
            price: 100,
        };
        let b = Order {
            id: 2,
            size: 200,
            price: 100,
        };

        book.add_ask(a);
        book.add_ask(b);

        assert_eq!(book.lowest_ask, 100);
        assert_eq!(book.ask_price_levels_count(), 1);

        let price_level = book.ask.front().unwrap();
        assert_eq!(price_level.queue.items.len(), 2);
        assert_eq!(
            price_level.queue.items.iter().map(|i| i.size).sum::<u16>(),
            300
        );
    }

    #[test]
    fn lower_ask_price_level() {
        let mut book = Book {
            last_consumed_orders: vec![],
            lowest_ask: 200,
            highest_bid: 0,
            bid: LinkedList::new(),
            ask: LinkedList::from([PriceLevel::new(Order {
                price: 200,
                size: 100,
                id: 1,
            })]),
        };

        let a = Order {
            id: 2,
            size: 100,
            price: 100,
        };
        let b = Order {
            id: 3,
            size: 200,
            price: 100,
        };

        book.add_ask(a);
        book.add_ask(b);

        assert_eq!(book.lowest_ask, 100);
        assert_eq!(book.ask_price_levels_count(), 2);

        let lowest_ask_price_level = book.ask.front().unwrap();
        assert_eq!(lowest_ask_price_level.queue.items.len(), 2);
        assert_eq!(
            lowest_ask_price_level
                .queue
                .items
                .iter()
                .map(|i| i.size)
                .sum::<u16>(),
            300
        );
    }

    #[test]
    fn add_higher_ask_price_level() {
        let mut book = Book {
            last_consumed_orders: vec![],
            lowest_ask: 50,
            highest_bid: 0,
            bid: LinkedList::new(),
            ask: LinkedList::from([PriceLevel::new(Order {
                price: 50,
                size: 100,
                id: 1,
            })]),
        };

        let a = Order {
            id: 2,
            size: 100,
            price: 100,
        };
        let b = Order {
            id: 3,
            size: 200,
            price: 100,
        };

        book.add_ask(a);
        book.add_ask(b);

        assert_eq!(book.lowest_ask, 50);
        assert_eq!(book.ask_price_levels_count(), 2);

        let highest_ask_price_level = book.ask.back().unwrap();
        assert_eq!(highest_ask_price_level.queue.items.len(), 2);
        assert_eq!(
            highest_ask_price_level
                .queue
                .items
                .iter()
                .map(|i| i.size)
                .sum::<u16>(),
            300
        );
    }

    #[test]
    fn match_ask() {
        let mut book = Book {
            last_consumed_orders: vec![],
            lowest_ask: 50,
            highest_bid: 40,
            bid: LinkedList::from([PriceLevel::new(Order {
                price: 40,
                size: 100,
                id: 1,
            })]),
            ask: LinkedList::from([PriceLevel::new(Order {
                price: 50,
                size: 100,
                id: 1,
            })]),
        };

        let a = Order {
            id: 3,
            size: 100,
            price: 40,
        };

        book.add_ask(a);

        assert_eq!(book.highest_bid, 0);
        assert_eq!(book.bid_price_levels_count(), 0);
    }

    #[test]
    fn match_ask_keeping_highest_bid() {
        let mut book = Book {
            last_consumed_orders: vec![],
            lowest_ask: 50,
            highest_bid: 40,
            bid: LinkedList::from([PriceLevel::new(Order {
                price: 40,
                size: 100,
                id: 1,
            })]),
            ask: LinkedList::from([PriceLevel::new(Order {
                price: 50,
                size: 100,
                id: 1,
            })]),
        };

        let a = Order {
            id: 3,
            size: 90,
            price: 40,
        };

        book.add_ask(a);

        assert_eq!(book.highest_bid, 40);
        assert_eq!(book.bid_price_levels_count(), 1);
    }

    #[test]
    fn match_ask_insufficient_volume() {
        let mut book = Book {
            last_consumed_orders: vec![],
            lowest_ask: 50,
            highest_bid: 40,
            bid: LinkedList::from([PriceLevel::new(Order {
                price: 40,
                size: 100,
                id: 1,
            })]),
            ask: LinkedList::from([PriceLevel::new(Order {
                price: 50,
                size: 100,
                id: 1,
            })]),
        };

        let a = Order {
            id: 3,
            size: 110,
            price: 40,
        };

        let result = book.add_ask(a);

        assert!(matches!(result, Err(AddOrderErrors::InsufficientMatch)));
        assert_eq!(book.highest_bid, 40);
    }

    #[test]
    fn match_ask_consume_two_levels() {
        let mut book = Book {
            last_consumed_orders: vec![],
            lowest_ask: 50,
            ask: LinkedList::from([PriceLevel::new(Order {
                price: 50,
                size: 100,
                id: 3,
            })]),
            highest_bid: 40,
            bid: LinkedList::from([
                PriceLevel::new(Order {
                    price: 40,
                    size: 100,
                    id: 1,
                }),
                PriceLevel::new(Order {
                    price: 39,
                    size: 100,
                    id: 2,
                }),
            ]),
        };

        let a = Order {
            id: 4,
            size: 110,
            price: 38,
        };

        book.add_ask(a);

        assert_eq!(book.highest_bid, 39);
        assert_eq!(book.bid_price_levels_count(), 1);
        assert_eq!(book.last_consumed_orders.len(), 2);
        assert_eq!(book.last_consumed_orders.iter().map(|x| x.size).sum::<u16>(), 110);
    }

    #[test]
    fn match_ask_consume_all_levels() {
        let mut book = Book {
            last_consumed_orders: vec![],
            lowest_ask: 50,
            highest_bid: 40,
            bid: LinkedList::from([
                PriceLevel::new(Order {
                    price: 40,
                    size: 100,
                    id: 1,
                }),
                PriceLevel::new(Order {
                    price: 39,
                    size: 100,
                    id: 2,
                }),
            ]),
            ask: LinkedList::from([PriceLevel::new(Order {
                price: 50,
                size: 100,
                id: 3,
            })]),
        };

        let a = Order {
            id: 4,
            size: 200,
            price: 38,
        };

        book.add_ask(a);

        assert_eq!(book.highest_bid, 0);
        assert_eq!(book.bid_price_levels_count(), 0);
        assert_eq!(book.last_consumed_orders.len(), 2);
    }
}
