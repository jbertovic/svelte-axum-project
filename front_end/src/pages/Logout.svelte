<script>
    import { user } from "./../js/store.js";
    import { getLogout } from "./../js/auth.js";
    import { onMount } from "svelte";

    $user;

    onMount(getLogout);
</script>

<div>
    <container>
        {#if $user}
            You are still logged in as {$user}.
        {:else}
            You are now logged out.
        {/if}
    </container>
</div>

<style>
    div {
        margin: 25px;
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
    }
</style>
