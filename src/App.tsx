import { parse_html_to_markdown, ParseOutput } from "../crates/app/pkg/app";
import { FC, useState, ChangeEvent } from "react";

const App: FC = () => {
  const [data, setData] = useState<ParseOutput | null>(null);

  const handleFileChange = async (
    e: ChangeEvent<HTMLInputElement>,
  ): Promise<void> => {
    const file = e.target.files?.[0];
    if (!file) return;

    const text = await file.text();

    const parsedData: ParseOutput = parse_html_to_markdown(text);
    setData(parsedData);
  };

  return (
    <>
      <input type="file" accept=".html" onChange={handleFileChange} />

      {data && (
        <div>
          <div>{data.markdown}</div>
          <div>{Array.from(data.header_lines).join(", ")}</div>
        </div>
      )}
    </>
  );
};

export default App;
