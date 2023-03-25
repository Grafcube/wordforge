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

<body class="main-screen">
  <div
    class="flex flex-col mx-auto text-2xl justify-center text-center place-content-center items-center"
  >
    {#await promise}
      Loading...
    {:then name}
      Welcome {name}!

      <button class="button-1" on:click={() => (location.href = "/create")}
        >New book</button
      >
    {:catch error}
      <div class="text-red-900">{error.message}</div>
    {/await}
  </div>
</body>
