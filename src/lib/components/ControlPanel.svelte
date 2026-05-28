<!-- FlowRename - ControlPanel 组件（右侧控制面板 - 含撤销与进度反馈） -->
<script lang="ts">
	import {
		filesStore,
		methodsStore,
		previewStore,
		loadingStore,
		statusMessageStore,
		canExecuteStore,
		statsStore
	} from '$lib/stores/app';
	import { executeRename, undoLastRename, getUndoStatus } from '$lib/commands';
	import type { RenameItem } from '$lib/types';
	import TagPanel from '$lib/components/TagPanel.svelte';
	import { detectFileCategories } from '$lib/stores/fileCategories';
	import { onMount } from 'svelte';
	import AboutDialog from '$lib/components/AboutDialog.svelte';

	let files = $derived($filesStore);
	let methods = $derived($methodsStore);
	let preview = $derived($previewStore);
	let loading = $derived($loadingStore);
	let canExecute = $derived($canExecuteStore);
	let stats = $derived($statsStore);

	/** 撤销历史是否存在 */
	let hasUndoHistory = $state(false);

	/** 关于弹窗显示状态 */
	let showAbout = $state(false);

	/** 组件挂载时检查撤销历史状态（支持持久化撤销） */
	onMount(async () => {
		try {
			const status = await getUndoStatus();
			hasUndoHistory = status.hasHistory;
		} catch {
			// Tauri 环境未就绪时忽略
		}
	});

	/** 进度状态 */
	let progress = $state<{ current: number; total: number; phase: string } | null>(null);

	/** 进度百分比 */

	/** 进度百分比 */
	let progressPercent = $derived(
		progress ? Math.round((progress.current / progress.total) * 100) : 0
	);

	/** 执行重命名操作 */
	async function handleExecute() {
		if (!canExecute) return;

		loadingStore.set(true);
		statusMessageStore.set('正在执行重命名...');
		progress = { current: 0, total: preview.filter((p) => p.isChanged && !p.hasConflict).length, phase: '重命名' };

		try {
			const renameItems: RenameItem[] = preview
				.filter((p) => p.isChanged && !p.hasConflict)
				.map((p) => ({
					currentPath: p.originalPath,
					newName: p.newName
				}));

			const result = await executeRename({
				renameItems,
				createUndoHistory: true
			});

			progress.current = result.successCount;

			if (result.failedCount > 0) {
				statusMessageStore.set(
					`完成: ${result.successCount} 成功, ${result.failedCount} 失败`
				);
			} else {
				statusMessageStore.set(`重命名完成: ${result.successCount} 个文件已重命名`);
				hasUndoHistory = true;
				filesStore.set([]);
				previewStore.set([]);
				detectFileCategories([]);
			}
		} catch (e) {
			statusMessageStore.set('执行异常: ' + String(e));
		} finally {
			loadingStore.set(false);
			setTimeout(() => { progress = null; }, 1200);
		}
	}

	/** 撤销上次重命名 */
	async function handleUndo() {
		if (!hasUndoHistory) return;

		loadingStore.set(true);
		statusMessageStore.set('正在撤销上次重命名...');
		progress = { current: 0, total: 1, phase: '撤销' };

		try {
			const result = await undoLastRename();
			if (result.success) {
				statusMessageStore.set(`撤销成功: ${result.restoredCount} 个文件已恢复`);
				hasUndoHistory = false;
			} else {
				statusMessageStore.set('撤销失败: ' + (result.error || '未知错误'));
			}
		} catch (e) {
			statusMessageStore.set('撤销异常: ' + String(e));
		} finally {
			loadingStore.set(false);
			setTimeout(() => { progress = null; }, 800);
		}
	}
</script>

<aside class="adr-control-panel flex flex-col h-full overflow-hidden">
	<!-- 面板标题 -->
	<div class="flex items-center justify-between px-3 py-2 border-b border-surface-500/20">
		<h2 class="text-sm font-semibold opacity-80">操作</h2>
		{#if hasUndoHistory}
			<span class="text-[11px] opacity-55 bg-green-500/10 text-green-300/80 px-1.5 py-0.5 rounded">可撤销</span>
		{/if}
	</div>

	<!-- 进度条 -->
	{#if progress}
		<div class="px-3 py-2 border-b border-surface-500/10">
			<div class="flex items-center justify-between text-xs mb-1">
				<span class="opacity-55">{progress.phase}中...</span>
				<span class="opacity-50">{progress.current}/{progress.total}</span>
			</div>
			<div class="w-full h-1.5 bg-surface-500/10 rounded-full overflow-hidden">
				<div
					class="h-full bg-blue-500/70 rounded-full transition-all duration-300 ease-out"
					style="width: {progressPercent}%"
				></div>
			</div>
		</div>
	{/if}

	<!-- 操作按钮 -->
	<div class="flex flex-col gap-2 px-3 py-3">
		<!-- 执行按钮 -->
		<button
			class="flex items-center justify-center gap-2 w-full px-4 py-2.5 text-sm font-medium rounded-lg transition-all
				disabled:opacity-30 disabled:cursor-not-allowed active:scale-[0.98]
				{canExecute
					? 'bg-emerald-500/90 hover:bg-emerald-500 text-white'
					: 'bg-surface-500/10 opacity-40'}"
			onclick={handleExecute}
			disabled={!canExecute || loading}
		>
			<svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
				<path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
			</svg>
			执行重命名
		</button>

		<!-- 撤销按钮 -->
		<button
			class="flex items-center justify-center gap-2 w-full px-4 py-2 text-sm font-medium rounded-lg
				transition-all disabled:opacity-30 disabled:cursor-not-allowed active:scale-[0.98]
				{hasUndoHistory
					? 'bg-amber-500/15 text-amber-600 dark:text-amber-400/80 hover:bg-amber-500/25 border border-amber-500/20'
					: 'bg-surface-500/5 opacity-30'}"
			onclick={handleUndo}
			disabled={!hasUndoHistory || loading}
		>
			<svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor">
				<path fill-rule="evenodd" d="M9.707 16.707a1 1 0 01-1.414 0l-7-7a1 1 0 010-1.414l7-7a1 1 0 011.414 1.414L3.414 9H17a1 1 0 110 2H3.414l6.293 6.293a1 1 0 010 1.414z" clip-rule="evenodd"/>
			</svg>
			撤销上次重命名
		</button>
	</div>

	<!-- 统计信息 -->
	{#if preview.length > 0}
		<div class="px-3 py-2 border-t border-surface-500/20">
			<h3 class="text-xs font-medium opacity-50 mb-2">统计</h3>
			<div class="space-y-1.5 text-xs">
				<div class="flex justify-between">
					<span class="opacity-50">总文件数</span>
					<span>{stats.total}</span>
				</div>
				<div class="flex justify-between text-green-600/80 dark:text-green-400/80">
					<span>将重命名</span>
					<span>{stats.changed}</span>
				</div>
				{#if stats.conflicts > 0}
					<div class="flex justify-between text-amber-600/80 dark:text-amber-400/80">
						<span>冲突</span>
						<span>{stats.conflicts}</span>
					</div>
				{/if}
				<div class="flex justify-between opacity-50">
					<span>不变</span>
					<span>{stats.unchanged}</span>
				</div>
			</div>

			<!-- 可视化进度条 -->
			<div class="mt-2 w-full h-2 bg-surface-500/10 rounded-full overflow-hidden flex">
				<div class="h-full bg-green-500/60 transition-all" style="width: {stats.total > 0 ? (stats.changed / stats.total) * 100 : 0}%"></div>
				<div class="h-full bg-amber-500/60 transition-all" style="width: {stats.total > 0 ? (stats.conflicts / stats.total) * 100 : 0}%"></div>
			</div>
		</div>
	{/if}

	<!-- 标签参考面板 -->
	<div class="flex-1 overflow-hidden px-3 py-2 border-t border-surface-500/20 flex flex-col">
		<h3 class="text-xs font-medium opacity-50 mb-1 shrink-0">标签参考</h3>
		<div class="flex-1 overflow-hidden">
			<TagPanel onInsertTag={() => {}} />
		</div>
	</div>

	<!-- 关于按钮 -->
	<div class="px-3 py-2 border-t border-surface-500/20">
		<button
			class="flex items-center justify-center gap-2 w-full px-4 py-2 text-xs font-medium rounded-lg
				transition-all active:scale-[0.98] opacity-50 hover:opacity-80
				bg-surface-500/5 hover:bg-surface-500/10"
			onclick={() => showAbout = true}
		>
			<svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor">
				<path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
			</svg>
			关于...
		</button>
	</div>
</aside>

<!-- 关于弹窗 -->
<AboutDialog bind:show={showAbout} />

<style>
	.adr-control-panel {
		background: var(--color-surface-200);
		width: var(--adr-panel-width);
		min-width: var(--adr-panel-width);
	}


</style>
