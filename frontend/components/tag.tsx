import Link from "next/link";

type Props = {
  tag: string;
  selected: boolean;
};

export default function Tag({ tag, selected }: Props) {
  return (
    <Link
      href={`/restaurants/food-type/${tag}`}
      className={`px-2 py-1 text-xs rounded-md ${
        selected
          ? "text-gray-300 dark:text-gray-700 bg-gray-700 dark:bg-gray-300"
          : "text-gray-700 dark:text-gray-300 bg-gray-300 dark:bg-gray-700"
      }`}
    >
      {tag}
    </Link>
  );
}
