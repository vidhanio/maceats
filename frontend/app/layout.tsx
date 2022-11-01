import Link from "next/link";
import "./globals.css";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="text-gray-900 bg-gray-100 dark:bg-gray-900 dark:text-gray-100">
        <nav className="flex flex-row items-center justify-center w-full gap-4 p-4 bg-gray-200 dark:bg-gray-800">
          <Link
            className="px-4 py-2 bg-gray-300 rounded-md dark:bg-gray-700"
            href="/"
          >
            Home
          </Link>
          <Link
            className="px-4 py-2 bg-gray-300 rounded-md dark:bg-gray-700"
            href="/locations"
          >
            Locations
          </Link>
          <Link
            className="px-4 py-2 bg-gray-300 rounded-md dark:bg-gray-700"
            href="/restaurants"
          >
            Restaurants
          </Link>
          <Link
            className="px-4 py-2 bg-gray-300 rounded-md dark:bg-gray-700"
            href="/restaurants/food-types"
          >
            Food Types
          </Link>
        </nav>
        <main className="flex flex-col items-center justify-center p-8 text-center">
          {children}
        </main>
      </body>
    </html>
  );
}
