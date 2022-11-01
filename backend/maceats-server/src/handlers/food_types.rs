use maceats::{Error, FoodType};

super::handlers! {
    all: () => async { Ok::<_, Error>(FoodType::all()) },
}
