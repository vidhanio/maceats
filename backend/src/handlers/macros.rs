macro_rules! handlers {
    {
        $($name:ident: ($($arg:ident: $ty:ty),*) => $e:expr),* $(,)?
    } => {
        $(
            pub async fn $name($($arg: $ty),*) -> ::std::result::Result<impl ::warp::Reply, ::warp::Rejection> {
                $e.await
                    .map($crate::SuccessResponse::from)
                    .map_err($crate::ErrorResponse::from)
                    .map_err(::warp::Rejection::from)
            }
        )*
    };
}

pub(super) use handlers;
