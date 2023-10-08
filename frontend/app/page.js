"use client";
import React from "react";

import Searchbar from "@/components/searchbar";

export default function Home() {
  return (
    <main className="">
      <h2 className="">
        celebrating open source projects. discover projects that interest you!
      </h2>
      <h3>
        making open source projects more discoverable. we set the standard for
        collaboration, so you can focus on the projects you love
      </h3>
      <Searchbar />
    </main>
  );
}
