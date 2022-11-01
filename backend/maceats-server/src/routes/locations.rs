use maceats::Location;

super::routes! {
    locations("locations") {
        all: warp::path!(),
        restaurants: warp::path!(Location)
    }
}
