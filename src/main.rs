use std::collections::{LinkedList, VecDeque};

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

    fn update(&mut self, amount: u16) -> Vec<Order> {
        let mut remaining: u16 = amount;

        let mut result: Vec<Order> = Vec::new();

        while remaining > 0 {
            // dbg!(remaining);

            if remaining == 0 {
                break;
            }

            if let Some(next_item) = self.get_mut() {
                match dbg!(next_item.size <= remaining) {
                    true => {
                        remaining -= next_item.size;

                        if let Some(order) = self.dequeue() {
                            result.push(order);
                        }
                    }
                    false => {
                        // self.subtract_from_first(remaining);

                        next_item.size -= remaining;

                        result.push(Order {
                            id: next_item.id,
                            price: next_item.price,
                            size: remaining,
                        });

                        remaining = 0;
                    }
                }
            }
        }

        result
    }
}

#[derive(Debug)]
struct LinkedItem {
    price: u16,
    queue: Queue,
}

impl LinkedItem {
    fn new(order: Order) -> Self {
        Self {
            price: order.price,
            queue: Queue::new_with_order(order),
        }
    }
}

#[derive(Debug)]
struct Book {
    highest_bid: u16,
    highest_ask: u16,
    bid: LinkedList<LinkedItem>,
    ask: LinkedList<LinkedItem>,
}

impl Book {
    fn new() -> Self {
        Self {
            highest_bid: 0,
            highest_ask: 0,
            bid: LinkedList::new(),
            ask: LinkedList::new(),
        }
    }

    fn add_ask(&mut self, order: Order) {
        if let Some(highest_ask_item) = self.ask.front_mut() {
            match self.highest_ask.cmp(&order.price) {
                std::cmp::Ordering::Less => {
                    self.highest_ask = order.price;
                    self.ask.push_front(LinkedItem::new(order));
                    return;
                }
                std::cmp::Ordering::Equal => {
                    highest_ask_item.queue.add(order);
                    return;
                }
                std::cmp::Ordering::Greater => {
                    // nothing
                }
            };
        }

        dbg!(&order);

        for price_level in self.ask.iter_mut() {
            if price_level.price == order.price {
                price_level.queue.add(order);
                return;
            } else {
                // go to a lower price level
            }
        }

        if self.ask.front().is_none() {
            self.highest_ask = order.price;
        }

        self.ask.push_back(LinkedItem::new(order));
    }

    fn add_bid(order: Order) {}
}

fn main() {
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

    let result = queue.update(375);

    println!("Result: {:?}", result);
    println!("Queue: {:?}", queue);
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

        let result = queue.update(375);
        let remaining_item = queue.get().unwrap();

        assert_eq!(result.len(), 4);
        assert_eq!(result.iter().map(|item| item.size).sum::<u16>(), 375);

        assert_eq!(remaining_item.id, 4);
        assert_eq!(remaining_item.size, 125);
    }

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

        assert_eq!(book.highest_ask, 100);
        assert_eq!(book.ask.len(), 1);

        let price_level = book.ask.front().unwrap();
        assert_eq!(price_level.queue.items.len(), 2);
        assert_eq!(
            price_level.queue.items.iter().map(|i| i.size).sum::<u16>(),
            300
        );
    }

    #[test]
    fn higher_ask_price_level() {
        let mut book = Book {
            highest_ask: 50,
            highest_bid: 0,
            bid: LinkedList::new(),
            ask: LinkedList::from([LinkedItem {
                price: 50,
                queue: Queue {
                    items: VecDeque::from([Order {
                        price: 50,
                        size: 100,
                        id: 1,
                    }]),
                },
            }]),
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

        assert_eq!(book.highest_ask, 100);
        assert_eq!(book.ask.len(), 2);

        let highest_price_level = book.ask.front().unwrap();
        assert_eq!(highest_price_level.queue.items.len(), 2);
        assert_eq!(
            highest_price_level.queue.items.iter().map(|i| i.size).sum::<u16>(),
            300
        );
    }

    #[test]
    fn lower_ask_price_level() {
        let mut book = Book {
            highest_ask: 500,
            highest_bid: 0,
            bid: LinkedList::new(),
            ask: LinkedList::from([LinkedItem {
                price: 500,
                queue: Queue {
                    items: VecDeque::from([Order {
                        price: 500,
                        size: 100,
                        id: 1,
                    }]),
                },
            }]),
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

        assert_eq!(book.highest_ask, 500);
        assert_eq!(book.ask.len(), 2);

        let lowest_price_level = book.ask.back().unwrap();
        assert_eq!(lowest_price_level.queue.items.len(), 2);
        assert_eq!(
            lowest_price_level.queue.items.iter().map(|i| i.size).sum::<u16>(),
            300
        );
    }
}
