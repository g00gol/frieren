import axios from "axios";

export default async function fetchApi(api) {
  let data;
  try {
    const res = await axios.get(`http://${api}`);
    data = res.data;
  } catch (e) {
    console.log(e);
    return [];
  }

  return data;
}
