<script>
	import {user} from './js/store.js';
	import {getSession} from './js/auth.js'
	import NavBar from "./component/Navbar.svelte";
	import LogIn from "./pages/Login.svelte";
	import LogOut from "./pages/Logout.svelte";
	import Secure from "./pages/Secure.svelte";
	import Apicheck from "./pages/Apicheck.svelte"
	import {onMount} from 'svelte';

	let menu;
	$: loggedin = $user !== '';

    // check if logged in
    onMount(getSession);

	const set_menu_items = (loggedin) => {
		if (loggedin) {
			return [
				{"label": "About", "id": 1},
				{"label": "Secure", "id": 3},
				{"label": "API Check", "id": 5},
				{"label": "Logout", "id": 4},
			];
		} else {
			return [
				{"label": "About", "id": 1},
				{"label": "API Check", "id": 5},
				{"label": "Login", "id": 2},
			];
		}
	}

</script>


<!-- MENNU BAR ON TOP -->
<NavBar navItems={set_menu_items(loggedin)} bind:menu={menu}></NavBar>


<!-- PAGE LOADING -->
{#if menu === 1}
	{#if !loggedin}
		<h4>Requires Login</h4>
	{:else}
		<h4>Logged In as {$user}</h4>
	{/if}
	<p>ABOUT</p>
{:else if menu === 2}
	<LogIn />
{:else if menu === 3}
	<Secure />
{:else if menu === 4}
	<LogOut />
{:else if menu === 5}
	<Apicheck />
{:else}
<h2>
	Page Not Found or Completed Yet
</h2>
{/if}