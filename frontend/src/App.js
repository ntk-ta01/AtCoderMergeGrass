import './App.css';
import Graph from "./components/Graph";
import {
  formatMomentDate,
  getNextSunday,
  getToday,
} from "./utils/DateUtil";
import moment from "moment";
import { useLoginState, GetGraphData } from "./api/APIClient";

const WEEKDAY = 7;
const WEEKS = 53;

function App() {
  const userdata = useLoginState().data;

  const GITHUB_LOGIN_LINK = "https://github.com/login/oauth/authorize?client_id=459928d588c951b32207";

  console.log("userdata:", userdata);
  const isLoggedIn = !!userdata && userdata.user_id.length > 0 ? true : false;

  let values = [];
  const dates = [];

  const today = getToday();
  const nextSunday = getNextSunday(today);
  const startDate = nextSunday.date(nextSunday.date() - WEEKS * WEEKDAY);
  for (let i = 0; i < WEEKS * WEEKDAY; i++) {
    const date = formatMomentDate(moment(startDate).add(i, "day"));
    values.push(undefined);
    dates.push(date);
  }

  let data = GetGraphData();
  if (isLoggedIn && !!data.data) {
    for (let i = 0; i < data.data.length; ++i) {
      for (let j = 0; j < data.data[i]['contributionDays'].length; ++j) {
        values[i * WEEKDAY + j] = data.data[i]['contributionDays'][j]['contributionCount'];
      }
    }
  }

  return (
    <div className="App">
      <h1>Tataku GitHub API</h1>
      {isLoggedIn ? (
        <p>Welcome {userdata.user_id}!</p>
      ) : (
        <a href={GITHUB_LOGIN_LINK}>Login</a>
      )}
      <Graph dates={dates} values={values}/>
    </div>
  );
}

export default App;
