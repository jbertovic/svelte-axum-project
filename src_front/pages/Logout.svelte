<script>
    import {user} from './../js/store.js';
    import {onMount} from 'svelte';
    let errorMessage;

    onMount(async() => {
            const res = await fetch('/auth/logout',{credentials: 'same-origin'});
            let logoutResponse = await res.json();
            if (logoutResponse.result == "error") {
                errorMessage = logoutResponse.message;
            }else {
                user.set('');
            }
    });
    
</script>

<div>
{#if $user}
    You are still logged in as {$user}.
{:else}
    You are now logged out. 
{/if}
</div>