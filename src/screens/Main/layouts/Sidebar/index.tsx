import AddDirectories from "@/components/AddDirectories";
import Tree from "@/components/Tree";
import { directories, getAllDirectories } from "@/services/directories";
import { onMount, type Component } from "solid-js";

const Sidebar: Component = () => {
  onMount(() => {
    void (async () => {
      await getAllDirectories();
    })();
  });

  return (
    <>
      <aside
        id="logo-sidebar"
        class="h-screen transition-transform -translate-x-full bg-white border-r border-gray-200 sm:translate-x-0 dark:bg-gray-800 dark:border-gray-700"
        aria-label="Sidebar"
      >
        <div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800">
          <ul class="space-y-2">
            <Tree acc={[]} items={directories} />
            <li>
              <AddDirectories />
            </li>
          </ul>
        </div>
      </aside>
    </>
  );
};

export default Sidebar;
