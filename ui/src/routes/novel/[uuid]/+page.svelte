<script lang="ts">
  import { page } from "$app/stores";

  const uuid = $page.params.uuid;

  async function getNovel() {
    let res = await fetch(`/novel/${uuid}`, {
      headers: { accept: "application/activity+json" },
    });
    if (res.ok) {
      return await res.json();
    } else {
      throw new Error(await res.text());
    }
  }
</script>

<div class="main-screen">
  {#await getNovel()}
    <div>Loading...</div>
  {:then data}
    <ul>
      {#each Object.keys(data) as key}
        <li>{key}: {data[key]}</li>
      {/each}
    </ul>
  {:catch error}
    <div class="text-red-900">{error.message}</div>
  {/await}
</div>
