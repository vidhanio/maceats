import { FoodType, get, Response, Restaurant } from "../../../../types";
import RestaurantList from "../../../../components/restaurant-list";
import { Suspense } from "react";

type Params = {
  params: {
    tag: FoodType;
  };
};

function toTitleCase(str: string): string {
  return str.replace(/\w\S*/g, function (txt) {
    return txt.charAt(0).toUpperCase() + txt.substring(1).toLowerCase();
  });
}

export default async function Page({ params: { tag } }: Params) {
  const restaurants = get<Restaurant[]>(`/restaurants/food-type/${tag}`);

  return (
    <div className="flex flex-col items-center justify-center gap-8 p-8 text-center">
      <h1 className="text-4xl font-bold">
        Restaurants with {toTitleCase(tag)}
      </h1>
      <Suspense fallback={<div>Loading...</div>}>
        <RestaurantList promise={restaurants} tag={tag} />
      </Suspense>
    </div>
  );
}
