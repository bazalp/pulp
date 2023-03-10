import { directories, getCollapsedDirectories } from "@/services/directories";
import { invoke } from "@tauri-apps/api";
import {
  createEffect,
  createMemo,
  createResource,
  createSignal,
  For,
  type Component,
} from "solid-js";

const fetchDirectoryFiles = async (pathsDir) => {
  return await invoke("get_directory_files", {
    pathsDir,
  });
};

const ViewFiles: Component = () => {
  const collapsedDirectories = createMemo(() => getCollapsedDirectories());

  const [pathDir, setPathDir] = createSignal([
    {
      path_dir: "/Users/vincentsourice/Music/GarageBand",
      files_starts_with: ["/plouf", "/Users"],
    },
  ]);

  const getValue = createMemo(() => {
    return directories.find((item) => item.collapsed);
  });

  console.log(getValue());

  const [files, { mutate, refetch }] = createResource(
    pathDir,
    fetchDirectoryFiles
  );

  createEffect(() => {
    setPathDir(
      Object.keys(collapsedDirectories()).map((key) => ({
        path_dir: key,
        files_starts_with: collapsedDirectories()[key],
      }))
    );
  });
  return (
    <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
      <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
        <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
          <tr>
            <th scope="col" class="px-6 py-3">
              Name
            </th>
            <th scope="col" class="px-6 py-3">
              <span class="sr-only">Edit</span>
            </th>
          </tr>
        </thead>
        <tbody>
          <For each={files()} fallback={<div>No files</div>}>
            {(file) => {
              return (
                <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                  <th
                    scope="row"
                    class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
                  >
                    {file.name}
                  </th>
                  <td class="px-6 py-4 text-right">
                    <a
                      href="#"
                      class="font-medium text-blue-600 dark:text-blue-500 hover:underline"
                    >
                      Edit
                    </a>
                  </td>
                </tr>
              );
            }}
          </For>
        </tbody>
      </table>
    </div>
  );
};

export default ViewFiles;
