pub trait DelayStrategy<D> {
    type Out;

    fn delay(&self, by: D) -> Self::Out;
}

#[derive(Debug, Clone, Copy)]
pub struct ThreadSleep {}

impl<D: Into<std::time::Duration>> DelayStrategy<D> for ThreadSleep {
    type Out = ();

    fn delay(&self, delay: D) -> Self::Out {
        std::thread::sleep(delay.into())
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg(feature = "async-tokio")]
pub struct TokioSleep {}

#[cfg(feature = "async-tokio")]
impl<D: Into<std::time::Duration>> DelayStrategy<D> for TokioSleep {
    type Out = tokio::time::Sleep;

    fn delay(&self, delay: D) -> Self::Out {
        tokio::time::sleep(delay.into())
    }
}

#[cfg(feature = "async-web")]
impl<D: Into<std::time::Duration>> DelayStrategy<D> for web_sys::Window {
    type Out = Box<dyn std::future::Future<Output = ()> + Unpin>;

    fn delay(&self, by: D) -> Self::Out {
        use wasm_bindgen_futures::{js_sys::Promise, JsFuture};

        let window = self.clone();
        let delay = by.into().as_millis() as i32;
        Box::new(Box::pin(async move {
            JsFuture::from(Promise::new(&mut |resolve, _reject| {
                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, delay)
                    .unwrap();
            }))
            .await
            .unwrap();
        }))
    }
}
