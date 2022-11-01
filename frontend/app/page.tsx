import { Suspense } from "react";
import RestaurantList from "../components/restaurant-list";
import { get, Restaurant } from "../types";

export default async function Page() {
  const promise = get<Restaurant[]>("/restaurants/open-now");

  return (
    <div className="flex flex-col items-center justify-center gap-8 p-8 text-center">
      <h1 className="text-4xl font-bold">Restaurants Open Now</h1>
      <Suspense fallback={<div>Loading...</div>}>
        <RestaurantList promise={promise} />
      </Suspense>
    </div>
  );
}
