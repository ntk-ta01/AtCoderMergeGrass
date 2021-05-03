import './App.css';
import Graph from "./components/Graph";
import {
  formatMomentDate,
  getNextSunday,
  getToday,
} from "./utils/DateUtil";
import moment from "moment";
import LoginForm from "./components/LoginForm";
import { useState } from 'react';

const WEEKDAY = 7;
const WEEKS = 53;

function App() {
  const [values, setValues] = useState([]);
  const dates = [];

  const today = getToday();
  const nextSunday = getNextSunday(today);
  const startDate = nextSunday.date(nextSunday.date() - WEEKS * WEEKDAY);
  for (let i = 0; i < WEEKS * WEEKDAY; i++) {
    const date = formatMomentDate(moment(startDate).add(i, "day"));
    if (values.length < WEEKS * WEEKDAY) {
      values.push(undefined);
    }
    if (dates.length < WEEKS * WEEKDAY) {
      dates.push(date);
    }
  }

  return (
    <div className="App">
      <h1>AtCoder Merge Grass</h1>
      <LoginForm values={values} setValues={setValues}/>
      <Graph dates={dates} values={values}/>
    </div>
  );
}

export default App;
