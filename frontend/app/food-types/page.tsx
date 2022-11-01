import { Suspense } from "react";
import ButtonList from "../../components/button-list";
import { FoodType, get, Location } from "../../types";

function toTitleCase(str: string): string {
  return str.replace(/\w\S*/g, function (txt) {
    return txt.charAt(0).toUpperCase() + txt.substring(1).toLowerCase();
  });
}

export default async function Page() {
  const promise = get<FoodType[]>(`/food-types`).then((response) => {
    return {
      data: response.data?.map((tag) => ({
        text: toTitleCase(tag),
        href: `/restaurants/food-type/${tag}`,
      })),
      error: response.error,
    };
  });

  return (
    <div className="flex flex-col items-center justify-center gap-8 p-8 text-center">
      <h1 className="text-4xl font-bold">Locations</h1>
      <Suspense fallback={<div>Loading...</div>}>
        <ButtonList promise={promise} />
      </Suspense>
    </div>
  );
}
