import RestaurantList from "../../components/restaurant-list";
import { Suspense } from "react";
import { get, Restaurant } from "../../types";

type Params = {
  params: {
    slug: string;
  };
};

export default async function Page({ params: { slug } }: Params) {
  const promise = get<Restaurant[]>("/restaurants");

  return (
    <div className="flex flex-col items-center justify-center gap-8 p-8 text-center">
      <h1 className="text-4xl font-bold">Restaurants</h1>
      <Suspense fallback={<div>Loading...</div>}>
        <RestaurantList promise={promise} />
      </Suspense>
    </div>
  );
}
