<script lang="ts">
  export let items: string[];
  export let selectedItem = "Select item"; // Initial text
  export let placeholder = "Filter";

  import {
    Listbox,
    ListboxButton,
    ListboxOption,
    ListboxOptions,
    Transition,
  } from "@rgossiaux/svelte-headlessui";
  import { ChevronDownIcon } from "@rgossiaux/svelte-heroicons/outline";

  let term = "";
  let itemList = items;
  const itemFilter = (term: string) => {
    term = term.toLowerCase();
    return term == ""
      ? items
      : items
          .filter((e) => e.toLowerCase().includes(term)) // Filter by search term
          .sort(
            // Sort by appearance of search term
            (a, b) =>
              a.toLowerCase().indexOf(term) - b.toLowerCase().indexOf(term)
          );
  };
</script>

<Listbox
  class="dark:bg-gray-800 rounded-md w-full p-2"
  value={selectedItem}
  on:change={(e) => (selectedItem = e.detail)}
>
  <div class="flex mx-auto align-middle justify-between">
    <ListboxButton class="w-full text-left">{selectedItem}</ListboxButton>
    <ChevronDownIcon class="my-auto h-4 w-4" />
  </div>
  <Transition
    enter="transition duration-100 ease-out"
    enterFrom="transform scale-95 opacity-0"
    enterTo="transform scale-100 opacity-100"
    leave="transition duration-75 ease-out"
    leaveFrom="transform scale-100 opacity-100"
    leaveTo="transform scale-95 opacity-0"
  >
    <ListboxOptions
      as="div"
      class="absolute z-10 text-left mt-4 dark:bg-gray-700 rounded-xl list-none"
    >
      <input
        class="dark:bg-gray-600 m-2 p-2 rounded-xl text-sl w-fit"
        type="search"
        {placeholder}
        name="filter"
        autocomplete="off"
        bind:value={term}
        on:input={() => (itemList = itemFilter(term))}
      />
      <div class="overflow-y-auto max-h-80 text-lg">
        {#each itemList as item}
          <ListboxOption
            class={({ active, selected }) => {
              const optionStyle =
                "p-2 m-1 cursor-pointer rounded-md active:dark:bg-gray-900";
              if (active) {
                return `${optionStyle} dark:bg-gray-800`;
              } else if (selected) {
                return `${optionStyle} dark:bg-gray-900`;
              } else return optionStyle;
            }}
            value={item}>{item}</ListboxOption
          >
        {/each}
      </div>
    </ListboxOptions>
  </Transition>
</Listbox>
