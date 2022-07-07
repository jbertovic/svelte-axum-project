<script>
  import {onMount} from 'svelte';
  import { user } from "./../js/store.js";

  let errormessage;
  let response; 

  onMount(async() => {
          const res = await fetch('/secure',{credentials: 'same-origin'});
          let secureResponse = await res.json();
          if (secureResponse.result == "unauthorized") {
              errorMessage = secureResponse.message;
          } else {
              response = JSON.stringify(secureResponse);
          }
  });

</script>

<p>Logged in as {$user}</p>

<pre>
Response:
{response}
</pre>