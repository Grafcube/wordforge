<script lang="ts">
  import { page } from "$app/stores";
  import { getEnglishName } from "all-iso-language-codes";

  const uuid = $page.params.uuid;

  async function getNovel() {
    let res = await fetch(`/novel/${uuid}`, {
      headers: { accept: "application/activity+json" },
    });
    if (res.ok) {
      const info = await res.json();
      return info;
    } else {
      throw new Error(await res.text());
    }
  }

  async function getUser(apub_id: string): Promise<string> {
    const url = new URL(`http://${apub_id}`); // TODO: Fix this mess
    let res = await fetch(url.href, {
      headers: { accept: "application/activity+json" },
    });
    if (res.ok) {
      const info = await res.json();
      const username = info.preferredUsername;
      if ($page.url.host == url.host) {
        return `@${username}`;
      } else {
        return `@${username}@{domain}`;
      }
    } else {
      throw new Error(await res.text());
    }
  }
</script>

<div class="main-screen">
  {#await getNovel()}
    <span>Loading...</span>
  {:then data}
    <h1 class="text-xl m-2">{data.name}</h1>
    <p class="p-2">{data.summary}</p>
    <div>
      <h2 class="text-md">Authors</h2>
      <ul>
        {#each data.authors as author}
          {#await getUser(author.id)}
            <span>Loading...</span>
          {:then username}
            <li><a href={author.id}>{username}</a></li>
          {:catch error}
            <div class="text-red-900">{error.message}</div>
          {/await}
        {/each}
      </ul>
    </div>
    <span>{data.genre}</span>
    <div class="m-2">
      {#each data.tags as tag}
        <span class="dark:bg-purple-900 px-2 m-1 rounded-full">{tag}</span>
      {/each}
    </div>
    <span>{getEnglishName(data.language)}</span>
    <div>
      <h2 class="text-md">Chapters</h2>
      {#each data.history as chapter}
        <span>{chapter}</span>
      {/each}
    </div>
  {:catch error}
    <div class="text-red-900">{error.message}</div>
  {/await}
</div>
