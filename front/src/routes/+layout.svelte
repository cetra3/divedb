<script lang="ts">
	import type { CurrentUserFragment } from '$lib/graphql/generated';
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
		<div class="divider" data-content="DIVES" />
		<div>
			<a
				on:click={hideMenu}
				class="btn btn-link mobile-link text-left"
				href="/users/{$session.user?.username}/dives"
			>
				Your Dives
			</a>
			<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/dives">
				All Dives
			</a>
		</div>
	{:else}
		<div>
			<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/dives"> Dives </a>
		</div>
	{/if}

	<div class="divider" data-content="SITES" />
	<div>
		<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/divesites">
			View List
		</a>
		<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/divesites/map">
			View Map
		</a>
	</div>
	{#if $session.loggedIn}
		<div class="divider" data-content="PHOTOS" />
		<div>
			<a
				on:click={hideMenu}
				class="btn btn-link mobile-link text-left"
				href="/users/{$session.user?.username}/photos"
			>
				Your Photos
			</a>
			<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/photos">
				All Photos
			</a>
		</div>
		<div class="divider" />
	{:else}
		<div class="divider" />
		<div>
			<a on:click={hideMenu} class="btn btn-link mobile-link text-left" href="/photos"> Photos </a>
		</div>
	{/if}
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
				Register
			</a>
		</div>
		<div>
			<a
				class="btn btn-link mobile-link text-left"
				href={`/login?redirect=${browser ? location.pathname : ''}`}
				on:click={hideMenu}
			>
				Login
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
			{#if $session.user}
				<div class="dropdown hide-small">
					<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
					<!-- svelte-ignore a11y-missing-attribute -->
					<a class="btn btn-link dropdown-toggle" tabindex="0"
						>Dives <i class="icon icon-caret" /></a
					>
					<ul class="menu menu-list">
						<li class="menu-item"><a href="/users/{$session.user?.username}/dives">Your Dives</a></li>
						<li class="menu-item"><a href="/dives">All Dives</a></li>
					</ul>
				</div>
			{:else}
				<a class="btn btn-link hide-small" href="/dives">Dives</a>
			{/if}
			<div class="dropdown hide-small">
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<!-- svelte-ignore a11y-missing-attribute -->
				<a class="btn btn-link dropdown-toggle" tabindex="0">Sites <i class="icon icon-caret" /></a>
				<ul class="menu menu-list">
					<li class="menu-item"><a href="/divesites">View List</a></li>
					<li class="menu-item"><a href="/divesites/map">View Map</a></li>
				</ul>
			</div>
			{#if $session.loggedIn}
				<div class="dropdown hide-small">
					<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
					<!-- svelte-ignore a11y-missing-attribute -->
					<a class="btn btn-link dropdown-toggle" tabindex="0"
						>Photos <i class="icon icon-caret" /></a
					>
					<ul class="menu menu-list">
						<li class="menu-item"><a href="/users/{$session.user?.username}/photos">Your Photos</a></li>
						<li class="menu-item"><a href="/photos">All Photos</a></li>
					</ul>
				</div>
			{:else}
				<a class="btn btn-link hide-small" href="/photos">Photos</a>
			{/if}
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
<footer class="container grid-lg">
	<div class="columns">
		<div class="column column-lg">
			<div class="float-right">
				<p>
					<small>
						DiveDB is an <a href="https://github.com/cetra3/divedb">open source project</a>, made
						with ❤️ by <a href="https://cetra3.github.io/">cetra3</a>
					</small>
				</p>
			</div>
		</div>
	</div>
</footer>

<style global lang="scss">
	@import '../style/app.scss';
</style>
