<script lang="ts">
  export let feedback = { color: "black", message: "" };

  function resetFeedback(_: any) {
    feedback = { color: "black", message: "" };
  }

  async function onSubmit(e: any) {
    const newAccount = new FormData(e.target);

    function isFormValid(data: { [fieldName: string]: string }): boolean {
      return (
        Object.values(data).every((value) => value != null && value !== "") &&
        data["password"].length >= 8
      );
    }

    const data: { [key: string]: any } = {};
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
          const payload = JSON.stringify({
            email: data.email,
            password: data.password,
            client_app: "Web",
          });
          await fetch("/api/v1/login", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: payload,
          });
          location.href = "/dash";
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
      name="display_name"
      value=""
      required
    />
  </div>
  <div>
    <input
      type="text"
      placeholder="Username"
      name="username"
      value=""
      required
    />
  </div>
  <div>
    <input type="email" placeholder="Email" name="email" value="" required />
  </div>
  <div>
    <input
      type="password"
      placeholder="Password (Minimum 8 characters)"
      name="password"
      value=""
      required
    />
  </div>
  <div>
    <button class="button-1" type="submit">Create</button>
  </div>
</form>
