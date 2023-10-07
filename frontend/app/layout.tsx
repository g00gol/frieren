import { Theme, ThemePanel } from "@radix-ui/themes";
import type { Metadata } from "next";
import { Inter } from "next/font/google";

import "./globals.css";
import "@radix-ui/themes/styles.css";
import theme from "@/constants/theme";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Create Next App",
  description: "Generated by create next app",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Theme {...theme}>{children}</Theme>
      </body>
    </html>
  );
}
