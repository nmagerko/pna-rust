use crate::thread::ThreadPool;
use crate::Result;

/// TODO
pub struct NaiveThreadPool {}
impl ThreadPool for NaiveThreadPool {
    fn new(threads: u32) -> Result<NaiveThreadPool> {
        unimplemented!();
    }
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unimplemented!();
    }
}
