import React, { useState, useEffect } from 'react';
import { render } from 'react-dom';
import {
  HashRouter as Router,
  Switch,
  Route,
  Redirect,
} from "react-router-dom";

import Background from '../assets/background.jpg';
import TitleImg from '../assets/logo.png';

import '../assets/main.css';

import { idlFactory } from "../../declarations/hello_world_backend";

import { ConnectionBadge } from './components';
import { Picker, Connect, Info } from './views';

const App = () => {
  const [connected, setConnected] = useState(false);
  const [principalId, setPrincipalId] = useState('');
  const [actor, setActor] = useState(false);

  const handleConnect = async () => {
    setConnected(true);

    if (!window.ic.plug.agent) {
      const whitelist = [process.env.SERVICE_CANISTER_ID];
      await window.ic?.plug?.createAgent(whitelist);
    }

    // Create an actor to interact with the NNS Canister
    // we pass the NNS Canister id and the interface factory
    const NNSUiActor = await window.ic.plug.createActor({
      canisterId: process.env.SERVICE_CANISTER_ID,
      interfaceFactory: idlFactory,
    });

    setActor(NNSUiActor);
  }

  useEffect(async () => {
    if (!window.ic?.plug?.agent) {
      setActor(false);
      setConnected(false);
      window.location.hash = '/connect';
    }
  }, []);

  useEffect(() => {
    (async function getId() {
      if (connected) {
        const principal = await window.ic.plug.agent.getPrincipal();

        if (principal) {
          setPrincipalId(principal.toText());
        }
      } else {
        window.location.hash = '/connect';
      }
    })();
  }, [connected]);


  return (
    <div className='app'>
      <img className="background" src={Background} />
      <div className="content">
        <img className='title-image' src={TitleImg} height="400" width="500" />
        <Router>
          <ConnectionBadge principalId={principalId} />
          {
            connected
              ? <Redirect to="/pick" />
              : <Redirect to="/connect" />
          }
          <Route path="/connect">
            <Connect handleConnect={handleConnect} />
          </Route>
          <Route path="/info">
            <Info />
          </Route>
          <Route path="/pick">
            <Picker
              actor={actor}
              principalId={principalId}
            />
          </Route>
        </Router>
      </div>
    </div>
  );
};


render(<App />, document.getElementById("app"));

