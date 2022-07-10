<script>
  import {onMount} from 'svelte';
  import { user } from "./../js/store.js";
  import {getSecure} from "./../js/fetch.js"

  let response; 

  onMount(async() => {
          response = await getSecure();
  });

</script>

<p>Logged in as {$user}</p>

<pre>
Response:
{response}
</pre>