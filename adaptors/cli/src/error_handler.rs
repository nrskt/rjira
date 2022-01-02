use std::fmt::Debug;
use std::future::Future;

pub async fn error_handler<F, T, Fut>(f: F)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = eyre::Result<T>>,
    T: Debug,
{
    match f().await {
        Err(e) => eprintln!("{:?}", e),
        Ok(r) => println!("{:?}", r),
    }
}
