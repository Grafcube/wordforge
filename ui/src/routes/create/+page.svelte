<script lang="ts">
  import { getAll639_1, getEnglishName } from "all-iso-language-codes";
  import SearchableListbox from "../lib/SearchableListbox.svelte";
  import ToggleSwitch from "../lib/ToggleSwitch.svelte";
  import { Genres, Roles } from "../enums";
  import { validate } from "../validate";

  const defaultGenre = "Select genre";
  const defaultRole = "Select your role";
  const defaultLang = "English";
  let selectedGenre = defaultGenre;
  let selectedRole = defaultRole;
  let selectedLang = defaultLang;
  let hasContentWarning = false;
  let feedback = "";

  const titleAreaInputHandler = (e: any) => {
    e.target.value = e.target.value.replace(/(\r\n|\n|\r)/gm, "");
    e.target.style.height = "auto";
    e.target.style.height = e.target.scrollHeight + "px";
  };

  const langList = () =>
    getAll639_1()
      .map(getEnglishName)
      .filter((l) => l != null) as string[];

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
    data["lang"] = selectedLang.trim();
    data["cw"] = hasContentWarning;
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
      style="width: 36rem;"
      class="space-y-4 p-4 max-w-xl"
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
        <SearchableListbox
          items={langList()}
          bind:selectedItem={selectedLang}
        />
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
        <ToggleSwitch
          bind:enabled={hasContentWarning}
          label="Content warning"
        />
      </div>
      <div>
        <button class="button-1" id="submit" type="submit">Create</button>
      </div>
    </form>
  </div>
  <div class="flex mx-auto text-2xl m-4 justify-center text-center">
    {#if feedback != ""}
      <p class="text-red-900">{feedback}</p>
    {/if}
  </div>
</body>
