<script>
    import { user } from "./../js/store.js";
    import { getSession, postLogin } from "./../js/auth"

    let username, password;
    let errorMessage = "";

    async function handleLogin() {
        let loginResponse = await postLogin(username, password);
        if (loginResponse.result == "error") {
            errorMessage = loginResponse.message;
        } else {
            getSession();
        }
    }
</script>

{#if !$user}
    <div>
        {#if errorMessage}
            <div>
                {errorMessage}
            </div>
        {/if}
        <div>
            <label for="username">Username</label>
                <input
                    class="input"
                    type="username"
                    placeholder="username"
                    bind:value={username}
                />
            <label for="password">Password</label>
                <input
                    class="input"
                    type="password"
                    placeholder="Password"
                    bind:value={password}
                />
        </div>
        <div>
                <button on:click={handleLogin}>
                    Login
                </button>
        </div>
    </div>
{:else}
    <div>
        Logged in as: {$user} <br />
        Now you may access the <strong>secure area </strong>from the Nav above
    </div>
{/if}
