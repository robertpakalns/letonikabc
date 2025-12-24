import { FC } from "react";
import { test } from "../crates/parser/pkg/parser";

const App: FC = () => {
  return <div>{test()}</div>;
};

export default App;
