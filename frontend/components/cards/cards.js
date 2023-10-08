import { useContext, useEffect, useState } from "react";

import SearchbarContext from "@/context/searchbar";
import fetchApi from "@/utils/fetchApi";
import Card from "./card";

export default function Cards({ children }) {
  const [data, setData] = useState([]);
  const { searchTerm } = useContext(SearchbarContext);

  useEffect(() => {
    fetchApi("104.248.58.127/api/repos").then((data) => {
      console.log(data);
      setData(data);
    });
  }, [searchTerm]);

  return (
    <>
      {data.map((project) => (
        <Card key={project.id} project={project} />
      ))}
    </>
  );
}
