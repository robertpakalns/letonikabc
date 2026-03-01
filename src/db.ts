import { Errors } from "./errorHandler";

const DATABASE = "letonikabc_db";
const VERSION = 2;
const MARKDOWN_STORE = "md";
const METADATA_STORE = "metadata";
const HEADINGS_STORE = "headings";

export interface MDRecord {
  hash: string;
  value: string;
}

export interface MetadataRecord {
  hash: string;
  title: string;
  author: string;
  size_before: number;
  size_after: number;
  created_at: string; // ISO 8601
  edited_at: string; //ISO 8601
  last_position: number;
}

export interface HeadingsRecord {
  hash: string;
  headings: any[];
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

      if (!db.objectStoreNames.contains(METADATA_STORE)) {
        db.createObjectStore(METADATA_STORE, {
          keyPath: "hash",
        });
      }

      if (!db.objectStoreNames.contains(HEADINGS_STORE)) {
        db.createObjectStore(HEADINGS_STORE, {
          keyPath: "hash",
        });
      }
    };

    request.onsuccess = () => res(request.result);
    request.onerror = () => rej(request.error);
  });
};

// MARKDOWN_STORE
export const getMarkdown = async (hash: string): Promise<MDRecord | null> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readonly");
    const store = tx.objectStore(MARKDOWN_STORE);

    const request = store.get(hash);
    request.onsuccess = () => res(request.result as MDRecord | null);
    request.onerror = () => rej(request.error);
  });
};

export const getAllMarkdowns = async (): Promise<MDRecord[]> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readonly");
    const store = tx.objectStore(MARKDOWN_STORE);

    const request = store.getAll();
    request.onsuccess = () => res(request.result as MDRecord[]);
    request.onerror = () => rej(request.error);
  });
};

export const addMarkdown = async (
  record: MDRecord,
): Promise<{ hash: string; error?: string }> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readwrite");
    const store = tx.objectStore(MARKDOWN_STORE);

    const { hash } = record;

    const request = store.add(record);

    request.onsuccess = () => res({ hash });
    request.onerror = () => {
      if (request.error?.name === "ConstraintError") {
        // Return hash to the existing entry with an error text
        res({ hash, error: Errors.DuplicateMarkdown });
      } else {
        rej(request.error);
      }
    };
  });
};

export const updateMarkdown = async (record: MDRecord): Promise<string> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readwrite");
    const store = tx.objectStore(MARKDOWN_STORE);

    const request = store.put(record);
    request.onsuccess = () => res(record.hash);
    request.onerror = () => rej(request.error);
  });
};

export const deleteMarkdown = async (hash: string): Promise<void> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(MARKDOWN_STORE, "readwrite");
    const store = tx.objectStore(MARKDOWN_STORE);

    const request = store.delete(hash);
    request.onsuccess = () => res();
    request.onerror = () => rej(request.error);
  });
};

// METADATA_STORE
export const getMetadata = async (
  hash: string,
): Promise<MetadataRecord | null> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(METADATA_STORE, "readonly");
    const store = tx.objectStore(METADATA_STORE);

    const request = store.get(hash);
    request.onsuccess = () => res(request.result as MetadataRecord | null);
    request.onerror = () => rej(request.error);
  });
};

export const getAllMetadata = async (): Promise<MetadataRecord[]> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(METADATA_STORE, "readonly");
    const store = tx.objectStore(METADATA_STORE);

    const request = store.getAll();
    request.onsuccess = () => res(request.result as MetadataRecord[]);
    request.onerror = () => rej(request.error);
  });
};

export const addMetadata = async (
  record: MetadataRecord,
): Promise<{ hash: string; error?: string }> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(METADATA_STORE, "readwrite");
    const store = tx.objectStore(METADATA_STORE);

    const request = store.add(record);

    const { hash } = record;

    request.onsuccess = () => res({ hash });
    request.onerror = () => rej(request.error);
  });
};

export const updateMetadata = async (
  record: MetadataRecord,
): Promise<string> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(METADATA_STORE, "readwrite");
    const store = tx.objectStore(METADATA_STORE);

    const { hash } = record;

    const request = store.put(record);
    request.onsuccess = () => res(hash);
    request.onerror = () => rej(request.error);
  });
};

export const deleteMetadata = async (hash: string): Promise<void> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(METADATA_STORE, "readwrite");
    const store = tx.objectStore(METADATA_STORE);

    const request = store.delete(hash);
    request.onsuccess = () => res();
    request.onerror = () => rej(request.error);
  });
};

// HEADING_STORE
export const getHeadings = async (
  hash: string,
): Promise<HeadingsRecord | null> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(HEADINGS_STORE, "readonly");
    const store = tx.objectStore(HEADINGS_STORE);

    const request = store.get(hash);
    request.onsuccess = () => res(request.result as HeadingsRecord | null);
    request.onerror = () => rej(request.error);
  });
};

export const getAllHeadings = async (): Promise<HeadingsRecord[]> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(HEADINGS_STORE, "readonly");
    const store = tx.objectStore(HEADINGS_STORE);

    const request = store.getAll();
    request.onsuccess = () => res(request.result as HeadingsRecord[]);
    request.onerror = () => rej(request.error);
  });
};

export const addHeadings = async (
  record: HeadingsRecord,
): Promise<{ hash: string; error?: string }> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(HEADINGS_STORE, "readwrite");
    const store = tx.objectStore(HEADINGS_STORE);

    const request = store.add(record);

    const { hash } = record;

    request.onsuccess = () => res({ hash });
    request.onerror = () => rej(request.error);
  });
};

export const updateHeadings = async (
  record: HeadingsRecord,
): Promise<string> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(HEADINGS_STORE, "readwrite");
    const store = tx.objectStore(HEADINGS_STORE);

    const { hash } = record;

    const request = store.put(record);
    request.onsuccess = () => res(hash);
    request.onerror = () => rej(request.error);
  });
};

export const deleteHeadings = async (hash: string): Promise<void> => {
  const db = await openDB();
  return new Promise((res, rej) => {
    const tx = db.transaction(HEADINGS_STORE, "readwrite");
    const store = tx.objectStore(HEADINGS_STORE);

    const request = store.delete(hash);
    request.onsuccess = () => res();
    request.onerror = () => rej(request.error);
  });
};
