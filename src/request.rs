use Method;
use futures::stream::Stream;

pub struct Request<'a> {
    pub method: Method,
    pub path: [&'a str],
    pub get_query: Option<&'a str>,
    pub body: BodyStream,
    #[cfg(target_arch = "wasm32")]
    pub headers: (), // TODO ok?
    #[cfg(not(target_arch = "wasm32"))]
    pub headers: (), // TODO use hyper header
}

// inner always impl's Stream<Item=[&u8], Error=Box<Error + Send + Sync>>
#[cfg(target_arch = "wasm32")]
pub struct BodyFuture {
    inner: futures::future::IntoStream<futures::future::FutureResult<Option<[&u8]>, ()>>,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct BodyFuture {
    inner: hyper::Body,
}

// TODO
impl Stream<Item=Option<[&u8]>, Error=String> for BodyFuture {
}
