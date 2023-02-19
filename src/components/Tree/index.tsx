import * as accordion from "@zag-js/accordion";
import { normalizeProps, useMachine } from "@zag-js/solid";
import { createMemo, createUniqueId, For, type Component } from "solid-js";

const Tree: Component<any> = (props) => {
  const [state, send] = useMachine(
    accordion.machine({
      id: createUniqueId(),
      collapsible: true,
      multiple: true,
    })
  );

  const api = createMemo(() => accordion.connect(state, send, normalizeProps));

  return (
    <div {...api().rootProps}>
      <For each={props.items}>
        {(item) => (
          <div {...api().getItemProps({ value: item.name })}>
            <button {...api().getTriggerProps({ value: item.name })}>
              {item.name}
            </button>
            <div class="ml-4" {...api().getContentProps({ value: item.name })}>
              <Tree items={item.children} />
            </div>
          </div>
        )}
      </For>
    </div>
  );
};

export default Tree;
