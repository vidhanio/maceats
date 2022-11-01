pub(super) fn reject_all<R: Reply>() -> impl Filter<Extract = (R,), Error = Rejection> + Clone {
    warp::any().and_then(|| async { Err(warp::reject::not_found()) })
}

macro_rules! modules {
    {
        $($module:ident),* $(,)?
    } => {
        $(
            mod $module;
        )*

        pub fn filter() -> impl ::warp::Filter<Extract = (impl ::warp::Reply,), Error = warp::Rejection> + ::std::clone::Clone {
            use ::warp::Filter;

            $crate::routes::macros::reject_all::<::warp::reply::Json>()
                $(.or($module::filter()))*
        }
    };
}

pub(super) use modules;

macro_rules! routes {
    {
        $module:ident ($route:literal) {
            $($filter:ident: $path:expr),* $(,)?
        }
    } => {
        pub fn filter() -> impl ::warp::Filter<Extract = (impl ::warp::Reply,), Error = ::warp::Rejection> + ::std::clone::Clone {
            use ::warp::Filter;

            ::warp::path($route)
                .and($crate::routes::macros::reject_all::<::warp::reply::Json>()
                        $(.or($filter()))*
                )
        }

        $(
            pub fn $filter() -> impl ::warp::Filter<Extract = (impl ::warp::Reply,), Error = ::warp::Rejection> + ::std::clone::Clone {
                use ::warp::Filter;

                $path.and_then($crate::handlers::$module::$filter)
            }
        )*
    };
}

pub(super) use routes;
use warp::{Filter, Rejection, Reply};
