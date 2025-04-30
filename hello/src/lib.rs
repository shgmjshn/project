pub struct ThreadPool;

impl ThreadPool {
    pub fn new<F>(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {
            
        }
}