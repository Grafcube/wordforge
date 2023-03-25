<script lang="ts">
  export let items: string[];
  export let label: string;
  export let selectedItem = "Select item"; // Initial text
  export let placeholder = "Search";

  import {
    Listbox,
    ListboxButton,
    ListboxLabel,
    ListboxOption,
    ListboxOptions,
  } from "@rgossiaux/svelte-headlessui";

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

<Listbox value={selectedItem} on:change={(e) => (selectedItem = e.detail)}>
  <ListboxLabel>{label}</ListboxLabel>
  <ListboxButton>{selectedItem}</ListboxButton>
  <ListboxOptions as="div" class="absolute z-10">
    <input
      type="search"
      {placeholder}
      id="search"
      name="search"
      bind:value={term}
      on:input={() => (itemList = itemFilter(term))}
    />
    {#each itemList as item}
      <ListboxOption value={item}>{item}</ListboxOption>
    {/each}
  </ListboxOptions>
</Listbox>
