import Image from "next/image";
import Link from "next/link";

type Props = {
  text: string;
  href: string;
};

export default function Button({ text, href }: Props) {
  return (
    <Link
      href={href}
      className="flex flex-col items-center justify-center w-full gap-4 p-4 text-center text-gray-800 bg-gray-200 shadow-xl sm:w-96 rounded-xl dark:bg-gray-800 dark:text-gray-200"
    >
      <h1 className="text-2xl font-bold">{text}</h1>
    </Link>
  );
}
