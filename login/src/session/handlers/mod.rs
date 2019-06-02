mod identification;
mod selection;

fn compat_poll_fn<T, E, F>(f: F) -> impl std::future::Future<Output = Result<T, E>>
where
    F: FnMut() -> futures01::Poll<T, E>,
{
    futures::compat::Compat01As03::new(futures01::future::poll_fn(f))
}
