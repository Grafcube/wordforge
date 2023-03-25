<script lang="ts">
  import { Genres, Roles } from "../enums";
  import SearchableListbox from "../SearchableListbox.svelte";

  let selectedGenre = "Select genre";
  let selectedRole = "Select your role";

  function onSubmit(_e: any) {}
</script>

<body class="main-screen">
  <form on:submit|preventDefault={onSubmit} class="space-y-4 p-4">
    <div>
      <input type="text" placeholder="Title" name="title" value="" required />
    </div>
    <div>
      <textarea placeholder="Summary" id="summary" name="summary" value="" />
    </div>
    <div>
      <SearchableListbox
        label="Genre"
        items={Genres}
        bind:selectedItem={selectedGenre}
      />
    </div>
    <div>
      <SearchableListbox
        label="Your role"
        items={Roles}
        bind:selectedItem={selectedRole}
      />
    </div>
    <div>
      <input type="tags" placeholder="Tags" name="tags" value="" />
    </div>
  </form>
</body>
