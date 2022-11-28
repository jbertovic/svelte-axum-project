<script>
  import { onMount } from "svelte";
  import { user } from "./../js/store.js";
  import { getSecure } from "./../js/fetch.js";

  let response;

  onMount(async () => {
    response = await getSecure();
  });
</script>

<div>
  <container class='wider'>
    <p>Logged in as {$user}</p>
    <p class='mono'>Response: {response}</p>
  </container>
</div>

<style>
  div {
    margin: 25px;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>
