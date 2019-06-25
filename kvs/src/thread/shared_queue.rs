use crate::thread::ThreadPool;
use crate::Result;

/// TODO
pub struct SharedQueueThreadPool {}
impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<SharedQueueThreadPool> {
        unimplemented!();
    }
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unimplemented!();
    }
}
