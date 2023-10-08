import { useContext, useEffect, useState } from "react";

import SearchbarContext from "@/context/searchbar";
import fetchApi from "@/utils/fetchApi";
import Card from "./card";

export default function Cards({ children }) {
  const [data, setData] = useState([]);
  const { searchTerm } = useContext(SearchbarContext);

  useEffect(() => {
    if (searchTerm.trim() === "") {
      fetchApi(`104.248.58.127/api/repos`).then((data) => {
        setData(data ?? []);
      });
      return;
    }

    let dataSet = [];
    fetchApi(`104.248.58.127/api/repos?name=${searchTerm}`).then((data) => {
      dataSet.push(...(data ?? []));

      fetchApi(`104.248.58.127/api/repos?technologies=${searchTerm}`).then(
        (data) => {
          dataSet.push(...(data ?? []));

          fetchApi(`104.248.58.127/api/repos?languages=${searchTerm}`).then(
            (data) => {
              dataSet.push(...(data ?? []));

              setData(
                Array.from(
                  new Map(dataSet.map((item) => [item["hash"], item])).values()
                )
              );
            }
          );
        }
      );
    });
  }, [searchTerm]);

  return (
    <>
      {data.map((project) => (
        <Card key={project.hash} project={project} />
      ))}
    </>
  );
}
