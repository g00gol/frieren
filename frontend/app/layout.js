import { Playfair_Display, Quicksand } from "next/font/google";

import "./globals.css";
import Nav from "@/components/nav";
import ContextProvider from "@/context/context";

const playfairDisplay = Playfair_Display({
  subsets: ["latin-ext"],
  weight: "700",
  variable: "--font-playfair-display",
});
const quicksand = Quicksand({
  subsets: ["latin-ext"],
  weight: "400",
  variable: "--font-quicksand",
});

export const metadata = {
  title: "frieren.playground",
  description:
    "celebrating open source projects. discover projects that interest you! making open source projects more discoverable. we set the standard for collaboration, so you can focus on the projects you love",
};

export default function RootLayout({ children }) {
  return (
    <html className="bg-night-blue text-mage-silver" lang="en">
      <body
        className={`${playfairDisplay.variable} ${quicksand.variable} w-3/6 mx-auto`}
      >
        <ContextProvider>
          <Nav />
          {children}
        </ContextProvider>
      </body>
    </html>
  );
}
