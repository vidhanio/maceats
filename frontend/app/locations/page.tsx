import { Suspense } from "react";
import ButtonList from "../../components/button-list";
import { get, Location } from "../../types";

export default async function Page() {
  const promise = get<Location[]>(`/locations`).then((response) => {
    return {
      data: response.data?.map((location) => ({
        text: location.name,
        href: `/locations/${location.slug}`,
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
