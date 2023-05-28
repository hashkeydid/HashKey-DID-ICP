import React from 'react';
import { Principal } from "@dfinity/principal";
import { service } from "../../../../declarations/hello_world_backend";
import Popup from '../../components/Popup';

const Picker = (actor, principalId) => {
  const [canisterId, setCanisterId] = React.useState('');
  const [email, setEmail] = React.useState('');
  const [threshold, setThreshold] = React.useState('');
  const [message, setMessage] = React.useState('');
  const [clickthreshold, setClickthreshold] = React.useState('');
  const [clickthreshold2, setClickthreshold2] = React.useState('');
  const [clickthreshold3, setClickthreshold3] = React.useState('');
  const [buttonPopup, setButtonPopup] = React.useState(false);

  async function createUser() {
    try {
      console.log(canisterId);
      const principal = await window.ic.plug.agent.getPrincipal();
      const controller = await service.getController({ 'canister_id': canisterId });
      let temp = 0;
      for (let i = 0; i < controller.length; i++) {
        if (JSON.stringify(controller[i]) !== JSON.stringify(principal)) {
          temp++;
        }
      }
      if (temp === controller.length) {
        throw (new Error('You are not a controller'));
      }
      const params = {
        to: 'ezw55-al2r4-u5pm6-jaew5-43qve-46acg-ypjdh-caeh4-3iv3o-eh5qw-kae',
        amount: 2_000_000,
        memo: 'test for dddd',
      };
      const result = await window.ic.plug.requestTransfer(params);
      console.log(result);
      const greeting = await service.create({ "threshold": threshold, canister_id: canisterId, "email": email });
      setMessage("succeed");
      setButtonPopup(true);
    } catch (e) {
      if (e.message == 'You are not a controller') {
        setMessage("failed, you are not the controller of this canister");
        setButtonPopup(true);
      } else {
        setMessage("failed, please send ICP to subscribe our service");
        setButtonPopup(true);
      }
      console.log(e.message);
    }
  }

  return (
    <div className='leaderboard-container' style={{ "fontSize": "30px" }}>
      <div align="left">
        <font face="verdana" size="4" color="black">Register</font>
      </div>
      <table>
        <tr>
          <td width="100px"><input
            placeholder="cycle threshold"
            id="threshold"
            value={threshold}
            onChange={(ev) => setThreshold(ev.target.value)}
            onClick={() => setClickthreshold(true)}
            onBlur={() => setClickthreshold(false)}
            size="28"
          ></input></td>
        </tr>
        <tr>
          {
            clickthreshold ?
              <font size="4" color="grey" align="left">an alert will be mailed to you when the threshold is reached</font>
              : null
          }
        </tr>
        <tr>
          <td><input placeholder="email"
            id="email"
            value={email}
            onChange={(ev) => setEmail(ev.target.value)}
            onClick={() => setClickthreshold2(true)}
            onBlur={() => setClickthreshold2(false)}
            size="28"
          ></input></td>
        </tr>
        <tr>
          {
            clickthreshold2 ?
              <font size="4" color="grey" align="right">subscribe email</font>
              : null
          }
        </tr>
        <tr>
          <td> <input placeholder="canister id"
            id="canister_id"
            value={canisterId}
            onChange={(ev) => setCanisterId(Principal.fromText(ev.target.value))}
            onClick={() => setClickthreshold3(true)}
            onBlur={() => setClickthreshold3(false)}
            size="28"
          ></input></td>
        </tr>
        <tr>
          {
            clickthreshold3 ?
              <font size="4" color="grey" align="left">canister id you want to inspect</font>
              : null
          }
        </tr>
      </table>
      <div style={{ margin: "30px" }} align="center">
        <button onClick={() =>{createUser();}} style={{ backgroundColor: "#grey" }}>Subscribe for one month</button>
        <Popup trigger={buttonPopup} setTrigger={setButtonPopup}>
        <font face="verdana" size="4" color="black">
          <span style={{}}>{message}</span>
        </font>
        </Popup>
      </div>
    </div >
  );
};

export default Picker;
