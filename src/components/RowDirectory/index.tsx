import { deleteDirectories, scanDirectory } from "@/services/directories";
import { type Directory } from "@prisma/client";
import { readDir, type FileEntry } from "@tauri-apps/api/fs";
import { Icon } from "solid-heroicons";
import { trash } from "solid-heroicons/outline";
import { createSignal, onMount, type Component } from "solid-js";
import Tree from "../Tree";

const filterChildRecursive = (arr: FileEntry[]): FileEntry[] => {
  return arr.reduce((acc: FileEntry[], curr) => {
    console.log(curr);
    // eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
    if (curr.children) {
      return [
        ...acc,
        { ...curr, children: filterChildRecursive(curr.children) },
      ];
    }
    return acc;
  }, []);
};

const RowDirectory: Component<{
  directory: Directory;
}> = (props) => {
  const selectedDirectory = (): void => {
    console.log("selectedDir");
  };
  const [items, setItems] = createSignal<FileEntry[]>([]);

  onMount(() => {
    void (async () => {
      const entries = await readDir(props.directory.path, {
        recursive: true,
      });

      console.log(filterChildRecursive(entries));
      setItems(filterChildRecursive(entries));
    })();
  });

  return (
    <div class="mb-4">
      <Tree items={[{ name: props.directory.name, children: items() }]} />

      <button
        onClick={() => {
          void (async () => {
            await deleteDirectories([props.directory.path]);
          })();
        }}
        title={`Remove ${props.directory.name}`}
      >
        <Icon path={trash} class="text-gray-900 h-6" />
      </button>

      <button
        onClick={() => {
          void (async () => {
            await scanDirectory(props.directory.path);
          })();
        }}
      >
        scan
      </button>
    </div>
  );
};

export default RowDirectory;
