const DATABASE = "letonikabc_db";
const VERSION = 1;
const MARKDOWN_STORE = "md";

export interface MDRecord {
  value: string;
}

const openDB = async (): Promise<IDBDatabase> => {
  return new Promise((res, rej) => {
    const request = indexedDB.open(DATABASE, VERSION);

    request.onupgradeneeded = (event) => {
      const db = (event.target as IDBOpenDBRequest).result;
      if (!db.objectStoreNames.contains(MARKDOWN_STORE)) {
        db.createObjectStore(MARKDOWN_STORE, {
          keyPath: "id",
          autoIncrement: true,
        });
      }
    };

    request.onsuccess = () => res(request.result);
    request.onerror = () => rej(request.error);
  });
};

export const getRecord = async (id: number): Promise<MDRecord | undefined> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readonly");
    const store = tx.objectStore(MARKDOWN_STORE);

    const request = store.get(id);
    request.onsuccess = () => res(request.result as MDRecord | undefined);
    request.onerror = () => rej(request.error);
  });
};

export const getAll = async (): Promise<MDRecord[]> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readonly");
    const store = tx.objectStore(MARKDOWN_STORE);

    const request = store.getAll();
    request.onsuccess = () => res(request.result as MDRecord[]);
    request.onerror = () => rej(request.error);
  });
};

export const addRecord = async (valueString: string): Promise<number> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readwrite");
    const store = tx.objectStore(MARKDOWN_STORE);

    const record: MDRecord = { value: valueString };
    const request = store.add(record);

    request.onsuccess = () => res(request.result as number);
    request.onerror = () => rej(request.error);
  });
};

export const updateRecord = async (
  id: number,
  newValue: string,
): Promise<number> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readwrite");
    const store = tx.objectStore(MARKDOWN_STORE);

    const request = store.put({ id, value: newValue });
    request.onsuccess = () => res(request.result as number);
    request.onerror = () => rej(request.error);
  });
};

export const deleteRecord = async (id: number): Promise<void> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readwrite");
    const store = tx.objectStore(MARKDOWN_STORE);

    const request = store.delete(id);
    request.onsuccess = () => res();
    request.onerror = () => rej(request.error);
  });
};
