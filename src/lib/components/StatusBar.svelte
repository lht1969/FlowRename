<!-- FlowRename - StatusBar 组件（底部状态栏） -->
<script lang="ts">
	import { statsStore, statusMessageStore, loadingStore } from '$lib/stores/app';

	let stats = $derived($statsStore);
	let message = $derived($statusMessageStore);
	let loading = $derived($loadingStore);
</script>

<footer
	class="adr-statusbar flex items-center justify-between h-[var(--adr-statusbar-height)] px-3 text-xs select-none"
	style="z-index: var(--adr-z-header);"
>
	<!-- 左侧：状态消息 -->
	<div class="flex items-center gap-2">
		{#if loading}
			<div class="adr-spinner w-3 h-3 border-2 border-blue-400 border-t-transparent rounded-full animate-spin"></div>
		{/if}
		<span class="opacity-65">{message}</span>
	</div>

	<!-- 右侧：文件统计 -->
	<div class="flex items-center gap-3 opacity-55">
		{#if stats.total > 0}
			<span>共 {stats.total} 个文件</span>
			<span class="text-green-600/80 dark:text-green-300/80">√ {stats.changed} 变更</span>
			{#if stats.conflicts > 0}
				<span class="text-amber-600/80 dark:text-amber-300/90">⚠ {stats.conflicts} 冲突</span>
			{/if}
		{/if}
		<span class="border-l border-[var(--color-surface-500)]/30 pl-3 ml-1">lht1969</span>
	</div>
</footer>

<style>
	.adr-statusbar {
		background: var(--color-surface-200);
		border-top: 1px solid var(--color-surface-500/20);
	}

	.adr-spinner {
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>
