import React from "react";
import Link from "next/link";

export default function NotFound() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-100 dark:bg-slate-900">
      <div className="text-center">
        <h1 className="mb-4 text-4xl font-bold dark:text-white">404</h1>
        <p className="mb-4 text-xl text-gray-600 dark:text-gray-300">
          Oops! Page not found
        </p>
        <Link href="/" className="text-blue-500 underline hover:text-blue-700">
          Return to Home
        </Link>
      </div>
    </div>
  );
}
