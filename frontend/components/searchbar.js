import { useContext, useRef } from "react";
import { AiOutlineSearch } from "react-icons/ai";

import SearchbarContext from "@/context/searchbar";

export default function Searchbar() {
  const inputRef = useRef();

  const { searchTerm, setSearchTerm } = useContext(SearchbarContext);

  function handleSearchbarSubmit(e) {
    e.preventDefault();
    setSearchTerm(inputRef.current.value);
  }

  return (
    <div className="join mt-12 mb-4 w-full h-full">
      <span className="join-item w-full">
        <label htmlFor="searchbar" />
        <input
          id="searchbar"
          placeholder={
            searchTerm
              ? searchTerm
              : "search by project name, language, or technologies"
          }
          className="w-full py-4 px-2 border-2 border-r-0 rounded-e-none border-sky-blue rounded-lg bg-deep-blue flex items-center font-quicksand text-mage-silver outline-none"
          type="text"
          ref={inputRef}
        />
      </span>
      <button
        onClick={handleSearchbarSubmit}
        className="join-item bg-night-blue border-2 border-l-0 px-4 border-sky-blue text-mage-silver"
      >
        <AiOutlineSearch />
      </button>
    </div>
  );
}
