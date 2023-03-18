<script lang="ts">
  import { page } from "$app/stores";

  const username = $page.params.username;

  async function getUser() {
    let res = await fetch(`/user/${username}`, {
      headers: { accept: "application/activity+json" },
    });
    if (res.ok) {
      return await res.json();
    } else {
      throw new Error(await res.text());
    }
  }

  let promise = getUser();
</script>

<div>
  {#await promise}
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
