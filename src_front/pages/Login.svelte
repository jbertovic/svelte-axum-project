<script>
    import { user } from "./../js/store.js";

    let username, password;
    let errorMessage = "";

    async function handleLogin() {
        const res = await fetch("/auth/login", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ username: username, password: password }),
        });
        let loginResponse = await res.json();
        if (loginResponse.result == "error") {
            errorMessage = loginResponse.message;
        } else {
            const res = await fetch("/session", {
                credentials: "same-origin",
            });
            let sessionResponse = await res.json();
            console.log(sessionResponse);
            if (sessionResponse.user_id !== "") {
                user.set(sessionResponse.user_id);
            } else {
                user.set("");
            }
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
