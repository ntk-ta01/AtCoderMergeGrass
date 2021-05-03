import React, { useState } from 'react';
import { Button, ButtonGroup } from "reactstrap";
import { useLoginState, GetGraphData } from "../api/APIClient";

export default function LoginForm(props) {
  const userdata = useLoginState().data;
  const GITHUB_LOGIN_LINK = "https://github.com/login/oauth/authorize?client_id=459928d588c951b32207";
  const isLoggedIn = !!userdata && userdata.user_id.length > 0 ? true : false;
  const [AtCoderUserName, setAtCoderUserName] = useState("");

  const [inputUserName, setInputUserName] = useState("");
  const [showMode, setShowMode] = useState("Submissions");

  let data_g = GetGraphData("github");
  let data_a = GetGraphData("atcoderproblems?uid=" + AtCoderUserName + "&show_mode=" + showMode);

  const updateGraph = () => {
    if (isLoggedIn && !!data_g.data && !!data_a.data) {
      let newvalues = [];
      for (let i = 0; i < data_g.data.length; ++i) {
        for (let j = 0; j < data_g.data[i]['contributionDays'].length; ++j) {
          newvalues.push(data_g.data[i]['contributionDays'][j]['contributionCount']);
        }
      }
      for (let i = 0; i < data_a.data.length; ++i) {
        newvalues[i] += data_a.data[i];
      }
      props.setValues([]);
      props.setValues([...newvalues]);
    }
  };

  // create:filter
  // filterボタンを作る
  // data_a.dataが変わる

  return (
    <form onSubmit={(e) => {e.preventDefault(); setAtCoderUserName(inputUserName); updateGraph();}}>
      {isLoggedIn ? (
        <p>Welcome {userdata.user_id}!</p>
      ) : (
        <p><a href={GITHUB_LOGIN_LINK}>Login</a></p>
      )}
      <label>
        <p>
          AtCoder UserName:
          <input type="text" onChange={(e) => {setInputUserName(e.target.value)}} />
        </p>
      </label>
      <p>showMode:{showMode}</p>
      <p>
        <ButtonGroup>
          <Button
            onClick={() => setShowMode("Submissions")}
          >
            All Submissions
          </Button>
          <Button
            onClick={() => setShowMode("AC")}
          >
            All AC
          </Button>
          <Button
            onClick={() => setShowMode("UniqueAC")}
          >
            Unique AC
          </Button>
        </ButtonGroup>
      </p>
      <p><input type="submit" value="Merge" /></p>
    </form>
  );
}
