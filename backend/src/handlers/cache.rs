use std::collections::{hash_map::Entry, HashMap};

use maceats::{CoffeeBrand, Error, FoodType, Location, Restaurant, Result};

#[derive(Default)]
pub struct Cache {
    restaurants_all: Option<Vec<Restaurant>>,
    restaurants_food_type: HashMap<FoodType, Vec<Restaurant>>,
    restaurants_coffee_brand: HashMap<CoffeeBrand, Vec<Restaurant>>,

    locations_all: Option<Vec<Location>>,
    location_restaurants: HashMap<String, Vec<Restaurant>>,
}

impl Cache {
    #[allow(clippy::missing_const_for_fn)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn invalidate(&mut self) {
        *self = Self::default();
    }

    pub async fn restaurants_all(&mut self) -> Result<Vec<Restaurant>> {
        if self.restaurants_all.is_none() {
            self.restaurants_all = Some(Restaurant::all().await?);
        }

        self.restaurants_all
            .clone()
            .ok_or_else(|| Error::Misc("cache error"))
    }

    pub async fn restaurants_food_type(&mut self, food_type: FoodType) -> Result<Vec<Restaurant>> {
        #[allow(clippy::map_entry)]
        if !self.restaurants_food_type.contains_key(&food_type) {
            let restaurants = self
                .restaurants_all()
                .await?
                .iter()
                .filter(|&r| r.tags.contains(&food_type))
                .cloned()
                .collect();
            self.restaurants_food_type.insert(food_type, restaurants);
        }

        self.restaurants_food_type
            .get(&food_type)
            .cloned()
            .ok_or_else(|| Error::Misc("cache error"))
    }

    pub async fn restaurants_coffee_brand(
        &mut self,
        coffee_brand: CoffeeBrand,
    ) -> Result<Vec<Restaurant>> {
        if let Entry::Vacant(e) = self.restaurants_coffee_brand.entry(coffee_brand) {
            e.insert(coffee_brand.restaurants().await?);
        }

        self.restaurants_coffee_brand
            .get(&coffee_brand)
            .cloned()
            .ok_or_else(|| Error::Misc("cache error"))
    }

    pub async fn locations_all(&mut self) -> Result<Vec<Location>> {
        if self.locations_all.is_none() {
            self.locations_all = Some(Location::all().await?);
        }

        self.locations_all
            .clone()
            .ok_or_else(|| Error::Misc("cache error"))
    }

    pub async fn location_restaurants(&mut self, location: Location) -> Result<Vec<Restaurant>> {
        if !self.location_restaurants.contains_key(&location.slug) {
            let restaurants = self
                .restaurants_all()
                .await?
                .iter()
                .filter(|&r| r.location.slug == location.slug)
                .cloned()
                .collect();
            self.location_restaurants
                .insert(location.slug.clone(), restaurants);
        }

        self.location_restaurants
            .get(&location.slug)
            .cloned()
            .ok_or_else(|| Error::Misc("cache error"))
    }
}
