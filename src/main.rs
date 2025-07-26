use std::collections::VecDeque;

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

struct LinkedItem {
    next: Box<LinkedItem>,
    queue: Box<LinkedItem>,
}

struct Book {
    bid: LinkedItem,
    ask: LinkedItem,
}

// struct Ask { }
//
// impl Order for Ask {
//
// }

impl Book {
    fn addAsk(order: Order) {}

    fn addBid(order: Order) {}
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
}
