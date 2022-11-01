import Link from "next/link";
import { Location } from "../types";

type Props = {
  location: Location;
};

export default function LocationCard({ location }: Props) {
  return (
    <Link
      href={`/locations/${location.slug}`}
      className="flex flex-col items-center justify-center w-64 h-64 p-4 text-center text-gray-800 bg-gray-200 rounded-lg shadow-lg dark:bg-gray-800 dark:text-gray-200"
    >
      {location.name}
    </Link>
  );
}
