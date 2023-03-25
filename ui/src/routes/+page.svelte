<script lang="ts">
  import Login from "./Login.svelte";
  import Register from "./Register.svelte";

  $: feedback = { message: "", color: "black" };
</script>

<body class="main-screen">
  <div class="mx-auto text-7xl text-center">
    <h1>Wordforge</h1>
  </div>

  <div
    class="flex mx-auto text-2xl m-4 justify-center text-center place-content-center items-center"
  >
    <Register bind:feedback />
    <Login bind:feedback />
  </div>
  <div class="flex mx-auto text-2xl m-4 justify-center text-center">
    {#if feedback.message != ""}
      <p class={feedback.color}>{feedback.message}</p>
    {/if}
  </div>

  <a href="/about" class="absolute bottom-0">About</a>
</body>
