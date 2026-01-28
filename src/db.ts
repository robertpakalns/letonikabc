const DATABASE = "letonikabc_db";
const VERSION = 2;
const MARKDOWN_STORE = "md";

export interface MDRecord {
  hash: string;
  value: string;
}

const openDB = async (): Promise<IDBDatabase> => {
  return new Promise((res, rej) => {
    const request = indexedDB.open(DATABASE, VERSION);

    request.onupgradeneeded = (event) => {
      const db = (event.target as IDBOpenDBRequest).result;
      if (!db.objectStoreNames.contains(MARKDOWN_STORE)) {
        db.createObjectStore(MARKDOWN_STORE, {
          keyPath: "hash",
        });
      }
    };

    request.onsuccess = () => res(request.result);
    request.onerror = () => rej(request.error);
  });
};

export const getRecord = async (hash: string): Promise<MDRecord | null> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readonly");
    const store = tx.objectStore(MARKDOWN_STORE);

    const request = store.get(hash);
    request.onsuccess = () => res(request.result as MDRecord | null);
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

export const addRecord = async (
  hash: string,
  value: string,
): Promise<string> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readwrite");
    const store = tx.objectStore(MARKDOWN_STORE);

    const record: MDRecord = { hash, value };
    const request = store.add(record);

    request.onsuccess = () => res(hash);
    request.onerror = () => {
      if (request.error?.name === "ConstraintError") {
        // It should throw an error -> display it visually
        console.warn(
          "Duplicate entry detected; using the existing one instead",
        );
        res(hash);
      } else {
        rej(request.error);
      }
    };
  });
};

export const updateRecord = async (
  hash: string,
  value: string,
): Promise<string> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readwrite");
    const store = tx.objectStore(MARKDOWN_STORE);

    const request = store.put({ hash, value });
    request.onsuccess = () => res(hash);
    request.onerror = () => rej(request.error);
  });
};

export const deleteRecord = async (hash: string): Promise<void> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readwrite");
    const store = tx.objectStore(MARKDOWN_STORE);

    const request = store.delete(hash);
    request.onsuccess = () => res();
    request.onerror = () => rej(request.error);
  });
};
