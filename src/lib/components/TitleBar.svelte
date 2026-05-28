<script lang="ts">
	import { themeStore, AVAILABLE_THEMES } from '$lib/stores/theme';
	import type { ThemeId } from '$lib/stores/theme';
	import { onMount } from 'svelte';

	let showThemeMenu = $state(false);
	let isMaximized = $state(false);
	let appWindow: any = null;

	onMount(async () => {
		const { getCurrentWindow } = await import('@tauri-apps/api/window');
		appWindow = getCurrentWindow();
		await checkMaximized();
	});

	async function checkMaximized() {
		if (!appWindow) return;
		isMaximized = await appWindow.isMaximized();
	}

	function toggleThemeMenu() {
		showThemeMenu = !showThemeMenu;
	}

	function selectTheme(id: ThemeId) {
		themeStore.set(id);
		showThemeMenu = false;
	}

	function handleClickOutside(event: MouseEvent) {
		if (showThemeMenu) {
			const target = event.target as HTMLElement;
			if (!target.closest('.theme-menu-container')) {
				showThemeMenu = false;
			}
		}
	}

	function handleMinimize() {
		appWindow?.minimize();
	}

	async function handleToggleMaximize() {
		await appWindow?.toggleMaximize();
	}

	function handleClose() {
		appWindow?.close();
	}
</script>

<svelte:window onclick={handleClickOutside} />

<header
	class="adr-titlebar flex items-center justify-between h-[var(--adr-header-height)] px-3 select-none"
	style="z-index: var(--adr-z-header);"
	data-tauri-drag-region
>
	<div class="flex items-center gap-2">
		<div class="adr-brand-icon w-5 h-5 rounded bg-blue-500 flex items-center justify-center">
			<svg class="w-3 h-3 text-white" viewBox="0 0 16 16" fill="currentColor">
				<path d="M2 4h12v2H2V4zm0 3h8v2H2V7zm0 3h10v2H2v-2z" />
			</svg>
		</div>
		<span class="text-sm font-semibold tracking-wide opacity-80">FlowRename</span>
		<span class="text-xs opacity-55 ml-1">v1.0</span>
	</div>

	<div class="flex items-center gap-1">
		<div class="theme-menu-container relative">
			<button
				class="p-1.5 rounded-md hover:bg-surface-500/20 transition-colors"
				onclick={toggleThemeMenu}
				title="切换主题"
			>
				<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<circle cx="12" cy="12" r="5" />
					<path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42" />
				</svg>
			</button>

			{#if showThemeMenu}
				<div
					class="absolute right-0 top-full mt-1 w-40 rounded-lg border border-surface-500/30 bg-surface-100 shadow-xl overflow-hidden"
					style="z-index: var(--adr-z-modal);"
				>
					{#each AVAILABLE_THEMES as theme}
						<button
							class="w-full text-left px-3 py-2 text-sm hover:bg-surface-500/20 transition-colors flex items-center gap-2 {$themeStore === theme.id ? 'active-theme-item' : ''}"
							onclick={() => selectTheme(theme.id)}
						>
							<span class="w-3 h-3 rounded-full" style="background: var(--color-surface-500);"></span>
							<span>{theme.label}</span>
							{#if $themeStore === theme.id}
								<svg class="w-3.5 h-3.5 ml-auto" viewBox="0 0 20 20" fill="currentColor"
									style="color: {theme.isDark ? 'rgb(96, 165, 250)' : 'rgb(37, 99, 235)'};">
									<path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
								</svg>
							{/if}
						</button>
					{/each}
				</div>
			{/if}
		</div>

		<button
			class="p-1.5 rounded-md hover:bg-surface-500/20 transition-colors"
			onclick={handleMinimize}
			title="最小化"
		>
			<svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M5 12h14" />
			</svg>
		</button>

		<button
			class="p-1.5 rounded-md hover:bg-surface-500/20 transition-colors"
			onclick={handleToggleMaximize}
			title={isMaximized ? '还原' : '最大化'}
		>
			{#if isMaximized}
				<svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M4 14h6v6M20 10h-6V4M14 10l7-7M10 14l-7 7" />
				</svg>
			{:else}
				<svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M4 4h16v16H4z" />
				</svg>
			{/if}
		</button>

		<button
			class="p-1.5 rounded-md hover:bg-red-500/60 transition-colors"
			onclick={handleClose}
			title="关闭"
		>
			<svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M6 6l12 12M18 6L6 18" />
			</svg>
		</button>
	</div>
</header>

<style>
	.adr-titlebar {
		background: var(--color-surface-200);
		border-bottom: 1px solid var(--color-surface-500/20);
	}

	:global([data-tauri-drag-region]) {
		-webkit-app-region: drag;
	}

	.adr-titlebar button,
	.adr-titlebar .theme-menu-container {
		-webkit-app-region: no-drag;
	}

	.active-theme-item {
		background: rgba(128, 128, 128, 0.15);
	}
</style>