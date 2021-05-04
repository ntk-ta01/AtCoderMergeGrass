import useSWR, { SWRConfiguration } from "swr";

const DATA_URL = `https://atcoder-merge-grass-api.herokuapp.com/data`;
const USER_URL = `https://atcoder-merge-grass-api.herokuapp.com/user`;

const typeCastFetcher = <T>(url: string) =>
  fetch(url, {method: 'GET', mode: 'cors', credentials: 'include'})
    .then((response) => response.json())
    .then((response) => response as T);

interface UserID {
  user_id: string;
}

interface GraphData {
  data: String;
}

const useSWRData = <T>(
  url: string,
  fetcher: (url: string) => Promise<T>,
  config: SWRConfiguration<T> = {}
) => {
  return useSWR(url, fetcher, {
    revalidateOnFocus: false,
    revalidateOnReconnect: false,
    refreshWhenHidden: true,
    ...config,
  });
};

export const useLoginState = () => {
  return useSWRData(USER_URL, (url) => typeCastFetcher<UserID>(url));
};

export const GetGraphData = (source: String) => {
  return useSWRData(`${DATA_URL}/${source}`, (url) => typeCastFetcher<GraphData>(url));
};