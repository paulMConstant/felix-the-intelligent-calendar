/// Simple wrapper around a rayon ThreadPool.
/// Has juste one method and is initialized with the number of threads on the machine.
#[derive(Debug)]
pub(crate) struct FelixThreadPool {
    thread_pool: rayon::ThreadPool,
}

impl FelixThreadPool {
    pub fn new() -> Self {
        FelixThreadPool {
            thread_pool: rayon::ThreadPoolBuilder::new()
                .num_threads(num_cpus::get())
                .build()
                .expect("Could not initialize rayon ThreadPool"),
        }
    }

    /// Transparent method which calls rayon::ThreadPool::spawn.
    pub fn spawn<OP>(&self, op: OP)
    where
        OP: FnOnce() + Send + 'static,
    {
        self.thread_pool.spawn(op)
    }
}

impl Default for FelixThreadPool {
    fn default() -> Self {
        Self::new()
    }
}
