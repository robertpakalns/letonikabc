const BASE_URL = "https://letonika.lv/literatura";

const extractBookId = (url = window.location.href) => {
  const id = new URL(url).searchParams.get("r");
  if (!id) throw new Error("Failed to extract book ID");
  return id;
};

const getBookData = async (id) => {
  try {
    const response = await fetch(`${BASE_URL}/Reader.aspx?r=${id}`, {
      credentials: "include",
    });
    return await response.text();
  } catch (err) {
    console.error(`Failed to fetch book data: ${err}`);
    return null;
  }
};

const extractComponentIds = (html) => {
  const match = html.match(
    /getComponents:\s*function\s*\(\)\s*{\s*return\s*\[([\s\S]*?)\];/,
  );

  if (!match) return [];

  return match[1]
    .split(",")
    .map((s) => s.trim().replace(/['"]/g, ""))
    .filter(Boolean)
    .map((item) => item.split("/")[0]);
};

const getEntryData = async (id) => {
  try {
    const response = await fetch(`${BASE_URL}/GetBookEntry.aspx?id=${id}`, {
      credentials: "include",
    });
    return await response.text();
  } catch (err) {
    console.error(`Failed to fetch entry data: ${err}`);
    return null;
  }
};

const downloadText = (text, id = "unknown") => {
  const blob = new Blob([text], { type: "text/html;charset=utf-8" });
  const url = URL.createObjectURL(blob);

  const a = document.createElement("a");
  a.href = url;
  a.download = `book-${id}.html`;

  document.body.appendChild(a);
  a.click();

  a.remove();
  URL.revokeObjectURL(url);
};

const sleep = (ms) => new Promise((res) => setTimeout(res, ms));

const main = async () => {
  const bookId = extractBookId();

  console.log(`Loading book data`);

  const bookData = await getBookData(bookId);
  if (!bookData) return;
  const bookComponents = extractComponentIds(bookData);

  let text = "";

  for (const [i, id] of bookComponents.entries()) {
    console.log(`Loading ${i + 1}/${bookComponents.length}`);

    const data = await getEntryData(id);
    if (!data) {
      await sleep(500);
      continue;
    }

    text += data;

    await sleep(500);
  }

  downloadText(text, bookId);
};

main();
