import { FC } from "react";
import { test, Output } from "../crates/parser/pkg/parser";

const App: FC = () => {
  const data: Output = test();

  return (
    <div>
      <div>{data.markdown}</div>
      <div>{Array.from(data.header_lines).join(", ")}</div>
    </div>
  );
};

export default App;
