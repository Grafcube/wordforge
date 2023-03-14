<script lang="ts">
  import Login from "./Login.svelte";
  import Register from "./Register.svelte";
</script>

<main>
  <div class="mx-auto text-7xl text-center">
    <h1>Wordforge</h1>
  </div>

  <div
    class="flex mx-auto text-2xl m-4 justify-center text-center place-content-center items-center"
  >
    <Register />
    <Login />
  </div>

  <a href="/about">About</a>
</main>
