use crate::thread::ThreadPool;
use crate::Result;

/// TODO
pub struct RayonThreadPool {}
impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> Result<RayonThreadPool> {
        unimplemented!();
    }
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unimplemented!();
    }
}
