import React from 'react';
import PlugConnect from '@psychedelic/plug-connect';

const Connect = ({ handleConnect }) => {
  const network = "https://mainnet.dfinity.network/";
  const whitelist = [process.env.SERVICE_CANISTER_ID || 'renrk-eyaaa-aaaaa-aaada-cai'];

  return (
    <>
      <h1 style={{ color: "black" }}>Connect to Subscribe</h1>

      <PlugConnect
        host={network}
        whitelist={whitelist}
        dark
        onConnectCallback={handleConnect}
      />
    </>
  );
};

export default Connect;
