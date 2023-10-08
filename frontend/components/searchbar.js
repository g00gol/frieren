import { useContext } from "react";
import SearchbarContext from "@/context/searchbar";

export default function Searchbar() {
  const { searchTerm, setSearchTerm } = useContext(SearchbarContext);

  function handleSearchbarChange(e) {
    setSearchTerm(e.target.value);
  }

  return (
    <>
      <label htmlFor="searchbar" />
      <input
        id="searchbar"
        placeholder="search by project name, language, or technologies"
        className="my-12 w-full py-4 px-2 border-2 border-sky-blue rounded-lg bg-deep-blue font-quicksand text-mage-silver outline-none"
        onChange={handleSearchbarChange}
      />
    </>
  );
}
