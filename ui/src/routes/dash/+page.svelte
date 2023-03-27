<script lang="ts">
  import { getName } from "../validate";
</script>

<body class="main-screen">
  <div
    class="flex flex-col mx-auto text-2xl justify-center text-center place-content-center items-center"
  >
    {#await getName()}
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
