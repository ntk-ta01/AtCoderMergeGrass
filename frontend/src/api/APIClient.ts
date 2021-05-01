import useSWR, { SWRConfiguration } from "swr";

const DATA_URL = `http://localhost:8080/data`;
const USER_URL = `http://localhost:8080/user`;

const typeCastFetcher = <T>(url: string) =>
  fetch(url, {mode: 'cors', credentials: 'include'})
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

export const GetGraphData = () => {
  return useSWRData(DATA_URL, (url) => typeCastFetcher<GraphData>(url));
};