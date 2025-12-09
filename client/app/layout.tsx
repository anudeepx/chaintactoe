import type { Metadata } from "next";
import { Montserrat } from "next/font/google";
// You may need to find a different host or local file for "Bitcount Grid Double"
// as it is not a standard Google Font, or import it in globals.css via URL.
// For now, we will use Montserrat.
import "./globals.css";
import { AppProviders } from "@/components/providers/AppProviders";

const montserrat = Montserrat({
  subsets: ["latin"],
  variable: "--font-montserrat",
});

export const metadata: Metadata = {
  title: "ChainTacToe",
  description: "Tic-Tac-Toe on Solana",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="dark">
      {/* If you really need the Bitcount font, add the link tag here or in globals.css */}
      <head>
        <link
          href="https://fonts.googleapis.com/css2?family=Bitcount+Grid+Double+Ink:wght@100..900&display=swap"
          rel="stylesheet"
        />
      </head>
      <body className={`${montserrat.className} font-sans`}>
        <AppProviders>{children}</AppProviders>
      </body>
    </html>
  );
}
