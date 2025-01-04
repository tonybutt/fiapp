import { useEffect, useState } from "react";
import { usePlaidLink } from "react-plaid-link";
import "./App.css";
import React from "react";

const App = () => {
  const [linkToken, setLinkToken] = useState(null);
  const generateToken = async () => {
    const response = await fetch("/api/create_link_token", {
      method: "POST",
    });
    const data = await response.json();
    console.log("Data", data);
    setLinkToken(data.link_token);
  };

  useEffect(() => {
    generateToken();
  }, []);

  return linkToken != null ? <Link linkToken={linkToken} /> : <></>;
};

interface LinkProps {
  linkToken: string | null;
}

const Link: React.FC<LinkProps> = (props: LinkProps) => {
  const callback = async (public_token: any, metadata: any) => {
    const response = await fetch("/api/set_access_token", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ public_token }),
    });
    const data = await response.json();
    console.log(data);
  };
  const onSuccess = React.useCallback(callback, []);

  const config: Parameters<typeof usePlaidLink>[0] = {
    token: props.linkToken!,
    onSuccess,
  };

  const { open, ready } = usePlaidLink(config);

  return (
    <button onClick={() => open()} disabled={!ready}>
      Link Account
    </button>
  );
};
export default App;
