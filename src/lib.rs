pub struct ThreadPool {
    size: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        ThreadPool {size}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}