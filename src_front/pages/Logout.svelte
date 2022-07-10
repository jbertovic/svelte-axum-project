<script>
    import {user} from './../js/store.js';
    import {getLogout} from './../js/auth.js'
    import {onMount} from 'svelte';

    $user

    onMount(getLogout);
    
</script>

<div>
{#if $user}
    You are still logged in as {$user}.
{:else}
    You are now logged out. 
{/if}
</div>