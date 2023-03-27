<script lang="ts">
  export let items: string[];
  export let selectedItem = "Select item"; // Initial text
  export let placeholder = "Search";

  import {
    Listbox,
    ListboxButton,
    ListboxOption,
    ListboxOptions,
    Transition,
  } from "@rgossiaux/svelte-headlessui";

  const optionStyle = "p-2 cursor-pointer rounded-md active:bg-gray-900";

  let term = "";
  let itemList = items;
  const itemFilter = (term: string) => {
    return term == ""
      ? items
      : items
          .filter((e) => e.toLowerCase().includes(term)) // Filter by search term
          .sort((a, b) => a.indexOf(term) - b.indexOf(term)); // Sort by appearance of search term
  };
</script>

<Listbox
  class="bg-gray-800 rounded-md w-full p-2"
  value={selectedItem}
  on:change={(e) => (selectedItem = e.detail)}
>
  <ListboxButton class="w-full text-left">{selectedItem}</ListboxButton>
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
      class="absolute z-10 text-left dark:bg-gray-700 rounded-xl list-none"
    >
      <input
        class="dark:bg-gray-600 m-2 p-2 rounded-xl text-sl w-fit"
        type="search"
        {placeholder}
        id="search"
        name="search"
        bind:value={term}
        on:input={() => (itemList = itemFilter(term))}
      />
      <div class="overflow-y-auto max-h-80 p-2 text-lg">
        {#each itemList as item}
          <ListboxOption
            class={({ active }) =>
              active ? `${optionStyle} bg-gray-800` : optionStyle}
            value={item}>{item}</ListboxOption
          >
        {/each}
      </div>
    </ListboxOptions>
  </Transition>
</Listbox>
