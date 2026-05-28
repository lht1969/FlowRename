<!-- FlowRename - AboutDialog 组件（关于弹窗） -->
<script lang="ts">
	import { open } from '@tauri-apps/plugin-shell';

	/** 弹窗显示状态 */
	let { show = $bindable(false) } = $props();

	/** GitHub 仓库地址 */
	const GITHUB_URL = 'https://github.com/lht1969/FlowRename';

	/** 打开 GitHub 链接 */
	async function openGitHub() {
		try {
			await open(GITHUB_URL);
		} catch (e) {
			console.error('打开链接失败:', e);
		}
	}

	/** 关闭弹窗 */
	function closeDialog() {
		show = false;
	}

	/** 点击遮罩关闭 */
	function handleBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			closeDialog();
		}
	}

	/** ESC 键关闭 */
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			closeDialog();
		}
	}
</script>

{#if show}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="adr-about-overlay fixed inset-0 z-[9999] flex items-center justify-center bg-black/40 backdrop-blur-sm"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
	>
		<div
			class="adr-about-dialog relative w-full max-w-md mx-4 rounded-2xl shadow-2xl border border-[var(--color-surface-500)]/20 overflow-hidden"
			role="dialog"
			aria-modal="true"
			aria-labelledby="about-title"
		>
			<!-- 头部装饰条 -->
			<div class="h-1.5 bg-gradient-to-r from-emerald-500/60 via-blue-500/60 to-purple-500/60"></div>

			<!-- 关闭按钮 -->
			<button
				class="absolute top-3 right-3 p-1.5 rounded-lg opacity-40 hover:opacity-80 hover:bg-[var(--color-surface-500)]/10 transition-all"
				onclick={closeDialog}
				aria-label="关闭"
			>
				<svg class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
					<path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
				</svg>
			</button>

			<!-- 内容区域 -->
			<div class="px-6 py-6 text-center">
				<!-- 应用图标占位 -->
				<div class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-gradient-to-br from-emerald-500/20 to-blue-500/20 flex items-center justify-center border border-[var(--color-surface-500)]/10">
					<svg class="w-8 h-8 text-emerald-500/80" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
						<path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
					</svg>
				</div>

				<!-- 应用名称 -->
				<h2 id="about-title" class="text-xl font-bold mb-1 opacity-90">FlowRename 1.0</h2>

				<!-- 副标题 -->
				<p class="text-sm opacity-50 mb-6">批量文件重命名工具</p>

				<!-- 分隔线 -->
				<div class="w-full h-px bg-[var(--color-surface-500)]/15 mb-5"></div>

				<!-- 版权信息 -->
				<p class="text-xs opacity-55 mb-4">
					Copyright © lht1969 2025-2026
				</p>

				<!-- GitHub 链接 -->
				<button
					class="inline-flex items-center gap-2 px-4 py-2 text-sm rounded-lg bg-[var(--color-surface-500)]/10 hover:bg-[var(--color-surface-500)]/20 transition-all opacity-70 hover:opacity-100"
					onclick={openGitHub}
				>
					<svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
						<path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
					</svg>
					<span>GitHub 仓库</span>
				</button>
			</div>

			<!-- 底部装饰 -->
			<div class="px-6 py-3 bg-[var(--color-surface-500)]/5 text-center">
				<p class="text-[11px] opacity-35">使用 Tauri + SvelteKit 构建</p>
			</div>
		</div>
	</div>
{/if}

<style>
	.adr-about-dialog {
		background: var(--color-surface-100);
	}

	@media (max-width: 480px) {
		.adr-about-dialog {
			margin: 1rem;
		}
	}
</style>
