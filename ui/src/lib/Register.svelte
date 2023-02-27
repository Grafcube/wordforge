<script lang="ts">
  let feedback = { color: "black", message: "" };

  function resetFeedback(_: any) {
    feedback = { color: "black", message: "" };
  }

  async function onSubmit(e: any) {
    const newAccount = new FormData(e.target);

    function isFormValid(data: { [fieldName: string]: any }): boolean {
      return Object.values(data).every(
        (value) => value != null && value !== ""
      );
    }

    const data = {};
    for (const [key, value] of newAccount.entries()) {
      data[key] = value;
    }
    const payload = JSON.stringify(data);

    if (isFormValid(data)) {
      await fetch("/api/v1/accounts", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: payload,
      }).then(async (response) => {
        if (response.ok) {
          const data = JSON.stringify(await response.json());
          feedback = {
            color: "text-green-900",
            message: `Account created\n${data}`,
          };
        } else {
          feedback = {
            color: "text-red-900",
            message: `Error: ${response.status}\nMessage: ${
              response.statusText
            }\nPayload: ${await response.text()}`,
          };
        }
      });
    } else {
      feedback = { color: "text-red-900", message: "Invalid input" };
    }
  }
</script>

<form
  on:submit|preventDefault={onSubmit}
  on:input={resetFeedback}
  class="space-y-4 p-4"
>
  <div>
    <input
      type="text"
      placeholder="Display name"
      id="display_name"
      name="display_name"
      value=""
    />
  </div>
  <div>
    <input
      type="text"
      placeholder="Username"
      id="username"
      name="username"
      value=""
    />
  </div>
  <div>
    <input type="email" placeholder="Email" id="email" name="email" value="" />
  </div>
  <div>
    <button type="submit">Create</button>
  </div>
  {#if feedback.message != ""}
    <p class={feedback.color}>{feedback.message}</p>
  {/if}
</form>
