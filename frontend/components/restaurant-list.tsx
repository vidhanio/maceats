import { FoodType, Response, Restaurant } from "../types";
import RestaurantCard from "./restaurant-card";

type Props = {
  promise: Promise<Response<Restaurant[]>>;
  tag?: FoodType;
};

export default async function RestaurantList({ promise, tag }: Props) {
  const restaurants: Response<Restaurant[]> = await promise;

  if (restaurants.error) throw new Error(restaurants.error);

  return (
    <div className="flex flex-row flex-wrap items-center justify-center gap-8 p-8">
      {restaurants.data?.map((restaurant) => (
        <RestaurantCard
          key={restaurant.name}
          restaurant={restaurant}
          tag={tag}
        />
      ))}
    </div>
  );
}
