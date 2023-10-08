import React, { createContext, useState } from "react";

const SearchbarContext = createContext();

export function SearchbarProvider({ children }) {
  const [searchTerm, setSearchTerm] = useState("");

  return (
    <SearchbarContext.Provider value={{ searchTerm, setSearchTerm }}>
      {children}
    </SearchbarContext.Provider>
  );
}

export default SearchbarContext;
