import axios from "axios";

export default async function fetchApi(api) {
  let data;
  try {
    console.log(`http://${api}`);
    const res = await axios.get(`http://${api}`);
    data = await res.json();
  } catch (e) {
    console.log(e);
    return [];
  }

  return data;
}
