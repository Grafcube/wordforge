<script lang="ts">
  import { Genres, Roles } from "../enums";
  import SearchableListbox from "../SearchableListbox.svelte";
  import { validate } from "../validate";

  const defaultGenre = "Select genre";
  const defaultRole = "Select your role";
  let selectedGenre = defaultGenre;
  let selectedRole = defaultRole;
  let feedback = "";

  const titleAreaInputHandler = (e: any) => {
    e.target.value = e.target.value.replace(/(\r\n|\n|\r)/gm, "");
    e.target.style.height = "auto";
    e.target.style.height = e.target.scrollHeight + "px";
  };

  async function onSubmit(e: any) {
    const newBook = new FormData(e.target);

    if (selectedGenre == defaultGenre) {
      feedback = "Select a genre";
      return;
    } else if (selectedRole == defaultRole) {
      feedback = "Select your role";
      return;
    }

    let data: { [key: string]: any } = {};
    for (const [key, value] of newBook.entries()) {
      data[key] = value.toString().trim();
    }
    data["genre"] = selectedGenre.trim();
    data["role"] = selectedRole.trim();
    data["tags"] = data["tags"].split(",").map((i: string) => i.trim());

    await fetch("/api/v1/novel", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    }).then(async (res) => {
      if (res.ok) {
        const text = await res.text();
        feedback = text;
      } else {
        feedback = `Error: ${res.status}\nMessage: ${
          res.statusText
        }\nPayload: ${await res.text()}`;
      }
    });
  }

  validate("/");
</script>

<body class="main-screen p-2">
  <h1 class="mx-auto text-3xl text-center">Create a new book</h1>
  <div
    class="flex justify-center text-center place-content-center items-center"
  >
    <form
      on:submit|preventDefault={onSubmit}
      on:input={() => (feedback = "")}
      class="space-y-4 p-4 w-1/2 max-w-xl"
    >
      <div>
        <textarea
          class="basic-input max-h-40 overflow-y-auto resize-none"
          placeholder="Title"
          name="title"
          value=""
          rows="1"
          wrap="soft"
          on:input={titleAreaInputHandler}
          on:keydown={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              document.getElementById("summary")?.focus();
            }
          }}
          required
        />
      </div>
      <div>
        <textarea
          class="basic-input"
          placeholder="Summary"
          id="summary"
          name="summary"
          value=""
        />
      </div>
      <div>
        <SearchableListbox items={Genres} bind:selectedItem={selectedGenre} />
      </div>
      <div>
        <SearchableListbox items={Roles} bind:selectedItem={selectedRole} />
      </div>
      <div>
        <input
          class="basic-input"
          type="tags"
          placeholder="Tags"
          name="tags"
          autocomplete="off"
          value=""
        />
      </div>
      <div>
        <button class="button-1" type="submit">Create</button>
      </div>
    </form>
  </div>
  <div class="flex mx-auto text-2xl m-4 justify-center text-center">
    {#if feedback != ""}
      <p class="text-red-900">{feedback}</p>
    {/if}
  </div>
</body>
