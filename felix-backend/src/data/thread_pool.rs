#[derive(Debug)]
pub(crate) struct ThreadPool {
    thread_pool: rayon::ThreadPool,
}

impl ThreadPool {
    pub fn new() -> Self {
        ThreadPool {
            thread_pool: rayon::ThreadPoolBuilder::new()
                .num_threads((num_cpus::get() - 1).max(1))
                .build()
                .expect("Could not initialize rayon ThreadPool"),
        }
    }

    /// Transparent method which calls rayon::ThreadPool::install.
    pub fn install<OP, R>(&self, op: OP) -> R
    where
        OP: FnOnce() -> R + Send,
        R: Send,
    {
        self.thread_pool.install(op)
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        Self::new()
    }
}
