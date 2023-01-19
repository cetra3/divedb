<script lang="ts">
	import { goto } from '$app/navigation';
	import { session } from '$lib/session';

	session.subscribe((val) => {
		if (val.loggedIn === false) {
			let loginPath = `/login?redirect=${location.pathname}`;
			goto(loginPath);
		}
	});
</script>
