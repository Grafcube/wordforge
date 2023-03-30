<script lang="ts">
  import {
    Switch,
    SwitchGroup,
    SwitchLabel,
  } from "@rgossiaux/svelte-headlessui";

  export let enabled: boolean = false;
  export let label: string;
</script>

<SwitchGroup class=" items-start text-left text-xl align-middle space-x-2">
  <Switch
    checked={enabled}
    class={enabled
      ? "basic-switch h-8 w-14 dark:bg-purple-600"
      : "basic-switch h-8 w-14 dark:bg-gray-600"}
    on:change={(e) => (enabled = e.detail)}
    ><span
      class={"inline-block ml-0.5 w-7 h-7 rounded-full" +
        (enabled
          ? " translate-x-6 ease-in-out duration-100 dark:bg-purple-900"
          : " ease-in-out duration-100 dark:bg-gray-800")}
    />
  </Switch>
  <SwitchLabel class="align-middle mt-0.5 absolute">{label}</SwitchLabel>
</SwitchGroup>
