use std::{marker::PhantomData, sync::Arc, thread::{self, sleep}, time::Duration};

struct ThreadBoundedShare<T> {
    data: Arc<T>,
    _limit: PhantomData<*const ()>,
}

impl<T> ThreadBoundedShare<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(data),
            _limit: PhantomData,
        }
    }

    pub fn get(&self) -> &T {
        &self.data
    }
}

unsafe impl<T> Sync for ThreadBoundedShare<T> {}

fn main() {
    let synconly_data = ThreadBoundedShare::new(100);
    let ref_synconly_data = &synconly_data;

    thread::scope(|s| {
        s.spawn(move || {
            sleep(Duration::from_secs(1));
            println!("synconly_data: {}", ref_synconly_data.get());
        });
        s.spawn(|| {
            println!("synconly_data1: {}", ref_synconly_data.get());
        });
    });
    println!("hello");

    println!("synconly_data: {}", synconly_data.get());
}
