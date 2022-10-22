macro_rules! handlers {
    {
        $($name:ident: ($($arg:ident: $ty:ty),*) => $e:expr),* $(,)?
    } => {
        $(
            pub async fn $name($($arg: $ty),*) -> ::std::result::Result<impl ::warp::Reply, ::warp::Rejection> {
                $e.await
                    .map(|ref v| ::warp::reply::json(v))
                    .map_err($crate::Error::from)
                    .map_err(::warp::Rejection::from)
            }
        )*
    };
}

pub(super) use handlers;
