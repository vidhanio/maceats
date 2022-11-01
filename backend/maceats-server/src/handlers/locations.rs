use maceats::Location;

use super::CACHE;

super::handlers! {
    all: () => CACHE.lock().await.locations_all(),
    restaurants: (loc: Location) => CACHE.lock().await.location_restaurants(loc),
}
