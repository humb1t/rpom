use crossbeam::queue::MsQueue;

pub enum OrderAction {
    New,
    Cancel,
}

pub struct OrderEvent {
    pub action: OrderAction,
    pub quantity: i32,
    pub specification_id: i32,
}

pub struct OrdersChannel(MsQueue<OrderEvent>);

impl OrdersChannel {
    pub fn new(queue: MsQueue<OrderEvent>) -> OrdersChannel
    {
        OrdersChannel(queue)
    }
    pub fn push(&self, event: OrderEvent) {
        &self.0.push(event);
    }
    pub fn pop(&self) -> OrderEvent {
        let queue = &self.0;
        queue.pop()
    }
}

