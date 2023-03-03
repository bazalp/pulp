import {
  loadChildrenDirectory,
  setDirectories,
  type Directory,
} from "@/services/directories";
import { isArray, isEmpty } from "lodash-es";
import { Icon } from "solid-heroicons";
import { folder, folderMinus, folderPlus } from "solid-heroicons/outline";
import {
  createEffect,
  For,
  Match,
  Show,
  splitProps,
  Switch,
  type Component,
} from "solid-js";
import ActionsDirectory from "../ActionsDirectory";

// TODO : Clean Directory[] type for use in Tree component with isRoot is Directory[] and children is FileEntry[]

const Tree: Component<{
  class?: string;
  items?: Directory[];
  acc: any[];
}> = (props) => {
  const [localProps, othersProps] = splitProps(props, ["items"]);
  const isRoot = (): boolean => isEmpty(props.acc);

  return (
    <ul {...othersProps}>
      <For each={localProps.items}>
        {(item, index) => {
          createEffect(() => {
            void (async () => {
              if (item.collapsed === true) {
                await loadChildrenDirectory(item.path, [...props.acc, index()]);
              }
            })();
          }, item.collapsed);
          return (
            <Show when={isArray(item.children) || isRoot()}>
              <li>
                <button
                  onClick={() => {
                    setDirectories(
                      ...(props.acc as []),
                      index(),
                      "collapsed",
                      item.collapsed !== true
                    );
                  }}
                  type="button"
                  class="flex items-center w-full p-2 text-base font-normal text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
                >
                  <Switch
                    fallback={
                      <Icon
                        path={folder}
                        class="flex-shrink-0 w-6 h-6 mr-3 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white"
                      />
                    }
                  >
                    <Match when={item.collapsed === true}>
                      <Icon
                        path={folderMinus}
                        class="flex-shrink-0 w-6 h-6 mr-3 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white"
                      />
                    </Match>

                    <Match when={item.collapsed !== true}>
                      <Icon
                        path={folderPlus}
                        class="flex-shrink-0 w-6 h-6 mr-3 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white"
                      />
                    </Match>
                  </Switch>

                  <span class="overflow-hidden overflow-ellipsis whitespace-nowrap">
                    {item.name}
                  </span>
                  <Show when={isRoot()}>
                    <div class="flex items-center ml-auto">
                      <ActionsDirectory directory={item} />
                    </div>
                  </Show>
                </button>
                <Show when={item.collapsed === true}>
                  <Tree
                    acc={[...props.acc, index(), "children"]}
                    items={item.children as Directory[]}
                    class="ml-4"
                  />
                </Show>
              </li>
            </Show>
          );
        }}
      </For>
    </ul>
  );
};

export default Tree;
