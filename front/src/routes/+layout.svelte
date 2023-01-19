<script type="ts">
	import type { CurrentUserFragment, UserLevel } from '$lib/graphql/generated';
	import { browser } from '$app/environment';
	import { client } from '$lib/graphql/client';
	import { onMount } from 'svelte';
	import { session } from '$lib/session';
	import { goto } from '$app/navigation';
	let currentUser: CurrentUserFragment | undefined;

	onMount(() => {
		client.getCurrentUser().then((val) => {
			if (val.currentUser) {
				session.set({
					loggedIn: true,
					user: val.currentUser
				});
			} else {
				session.set({
					loggedIn: false
				});
			}
		});
	});

	const logout = (e: Event) => {
		localStorage.removeItem('token');
		currentUser == undefined;
		session.set({ loggedIn: false });
		goto('/');
	};

	let showMenu = false;

	const hideMenu = () => {
		showMenu = false;
	};
	const menuShow = () => {
		showMenu = true;
	};
</script>

<div class={showMenu ? 'mobile-view active' : 'mobile-view'}>
	<div>
		<button aria-label="Menu" class="btn btn-link mobile-link text-right" on:click={hideMenu}>
			<i class="icon icon-menu" />
		</button>
	</div>
	{#if $session.loggedIn}
		<div>
			<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/dives"> Dives </a>
		</div>
	{/if}
	<div>
		<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/divesites"> Sites </a>
	</div>
	<div>
		<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/photos"> Photos </a>
	</div>
	<div>
		<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/sealife"> Sealife </a>
	</div>

	<div class="divider" />

	{#if $session.loggedIn}
		<div>
			{#if $session?.user?.level === 'ADMIN'}
				<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/admin"> Admin </a>
			{/if}
			<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/feedback">
				Feedback
			</a>
			<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/settings">
				Settings
			</a>
			<button on:click={logout} class="btn btn-link mobile-link text-left"> Logout </button>
		</div>
	{:else}
		<div>
			<a
				class="btn btn-link mobile-link text-left"
				href={`/register?redirect=${browser ? location.pathname : ''}`}
				on:click={hideMenu}
			>
				<button class="btn btn-link mobile-link text-left"> Register </button>
			</a>
		</div>
		<div>
			<a
				class="btn btn-link mobile-link text-left"
				href={`/login?redirect=${browser ? location.pathname : ''}`}
				on:click={hideMenu}
			>
				<button class="btn btn-link mobile-link text-left"> Login </button>
			</a>
		</div>
	{/if}
</div>

<div class="container grid-lg">
	<header class="navbar">
		<section class="navbar-section">
			<button aria-label="Menu" class="btn btn-link mobile-link hide-big" on:click={menuShow}>
				<i class="icon icon-menu" />
			</button>
			{#if $session.loggedIn}
				<a class="btn btn-link hide-small" href="/dives">Dives</a>
			{/if}
			<a class="btn btn-link hide-small" href="/divesites">Sites</a>
			<a class="btn btn-link hide-small" href="/photos">Photos</a>
			<a class="btn btn-link hide-small" href="/sealife">Sealife</a>
		</section>
		<section class="navbar-center">
			<a href="/">
				<img class="logo" width="150" height="30" src={'/logo.svg'} alt="DiveDB Logo" />
			</a>
		</section>
		<section class="navbar-section">
			{#if $session.loggedIn}
				{#if $session?.user?.level === 'ADMIN'}
					<a href="/admin" class="btn btn-link hide-small">Admin </a>
				{/if}

				<a href="/feedback" class="btn btn-link hide-small">Feedback </a>
				<a href="/settings" class="btn btn-link hide-small">Settings</a>
				<button on:click={logout} class="btn btn-link hide-small">Logout</button>
			{:else}
				<a href="/register" class="btn btn-link hide-small">Register</a>
				<a href="/login" class="btn btn-link hide-small">Login</a>
			{/if}
		</section>
	</header>
</div>

<slot />

<style global lang="scss">
	@import '../style/app.scss';
</style>
