"use client";
import React from "react";

import Searchbar from "@/components/searchbar";
import Cards from "@/components/cards/cards";

export default function Home() {
  return (
    <main>
      <h2>
        celebrating open source projects. discover projects that interest you!
      </h2>
      <h3>
        making open source projects more discoverable. we set the standard for
        collaboration, so you can focus on the projects you love
      </h3>
      <Searchbar />

      <Cards />
    </main>
  );
}
