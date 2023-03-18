<script lang="ts">
  async function getName() {
    const res = await fetch("/api/v1/validate");
    const text = await res.text();
    if (res.ok) {
      return text;
    } else {
      throw new Error(text);
    }
  }

  let promise = getName();
</script>

<h1
  class="flex mx-auto text-2xl m-4 justify-center text-center place-content-center items-center"
>
  {#await promise}
    Loading...
  {:then name}
    Welcome {name}!
  {:catch error}
    <div class="text-red-900">{error.message}</div>
  {/await}
</h1>
