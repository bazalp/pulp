import { createDirectories } from "@/services/directories";
import { open } from "@tauri-apps/api/dialog";
import { isEmpty } from "lodash-es";
import { Icon } from "solid-heroicons";
import { plusCircle } from "solid-heroicons/outline";
import { createEffect, createSignal, type Component } from "solid-js";

const AddDirectories: Component = () => {
  const [getSelectedDirectories, setSelectedDirectories] = createSignal([]);

  const openSelectedDirectories = async (): Promise<void> => {
    const selected = await open({
      directory: true,
      multiple: true,
    });
    setSelectedDirectories(selected);
  };

  createEffect(() => {
    if (!isEmpty(getSelectedDirectories())) {
      createDirectories(getSelectedDirectories());
      setSelectedDirectories([]);
    }
  });

  return (
    <button
      onClick={openSelectedDirectories}
      type="button"
      class="flex items-center w-full p-2 text-base font-normal text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
    >
      <Icon
        path={plusCircle}
        class="flex-shrink-0 w-6 h-6 mr-3 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white"
      />
      <span class="overflow-hidden overflow-ellipsis whitespace-nowrap">
        Add directory
      </span>
    </button>
  );
};

export default AddDirectories;
