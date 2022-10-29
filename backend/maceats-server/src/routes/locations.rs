use maceats::Location;

super::routes! {
    locations {
        all: warp::path!(),
        restaurants: warp::path!(Location)
    }
}
