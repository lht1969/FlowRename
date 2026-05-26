<!-- AdRename - Root Layout -->
<script lang="ts">
	import '../app.css';
	import { themeStore } from '$lib/stores/theme';

	let { children } = $props();

	$effect(() => {
		themeStore.init();
	});
</script>

{@render children()}
