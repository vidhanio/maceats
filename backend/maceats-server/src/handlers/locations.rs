use maceats::Location;

super::handlers! {
    all: () => Location::all(),
    restaurants: (loc: Location) => loc.restaurants(),
}
