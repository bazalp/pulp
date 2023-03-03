import { deleteDirectories, scanDirectory } from "@/services/directories";
import { type Directory } from "@prisma/client";
import { confirm } from "@tauri-apps/api/dialog";
import { Icon } from "solid-heroicons";
import { documentMagnifyingGlass, trash } from "solid-heroicons/outline";
import { type Component } from "solid-js";

const ActionsDirectory: Component<{
  directory: Directory;
}> = (props) => {
  return (
    <>
      <button
        class="text-gray-500 hover:text-gray-900"
        onClick={(event) => {
          event.preventDefault();
          event.stopPropagation();
          void (async () => {
            const response = await confirm(
              `Are you sure you want to delete ${props.directory.name} ?\n\n(This action does not delete the original)`,
              {
                title: "Delete directory",
                type: "error",
              }
            );
            if (response) {
              await deleteDirectories([props.directory.path]);
            }
          })();
        }}
        title={`Remove ${props.directory.name}`}
      >
        <Icon path={trash} class="w-4" />
      </button>
      <button
        class="text-gray-500 hover:text-gray-900"
        onClick={(event) => {
          event.preventDefault();
          event.stopPropagation();
          void (async () => {
            const response = await confirm(
              `Scan the directory ${props.directory.name} to get all samples`,
              {
                title: "Scan the directory",
                type: "warning",
              }
            );
            if (response) {
              await scanDirectory(props.directory.path);
            }
          })();
        }}
        title={`Scanning ${props.directory.name}`}
      >
        <Icon path={documentMagnifyingGlass} class="w-4" />
      </button>
    </>
  );
};

export default ActionsDirectory;
