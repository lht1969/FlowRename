<!-- AdRename - ControlPanel 组件（右侧控制面板 - 含撤销与进度反馈） -->
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
	import { previewRename, executeRename, undoLastRename, getUndoStatus } from '$lib/commands';
	import type { RenameItem } from '$lib/types';
	import { onMount } from 'svelte';

	let files = $derived($filesStore);
	let methods = $derived($methodsStore);
	let preview = $derived($previewStore);
	let loading = $derived($loadingStore);
	let canExecute = $derived($canExecuteStore);
	let stats = $derived($statsStore);

	/** 撤销历史是否存在 */
	let hasUndoHistory = $state(false);

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

	/** 执行按钮的动态 class */
	let executeBtnClass = $derived(
		canExecute
			? 'adr-execute-active flex items-center justify-center gap-2 w-full px-4 py-2.5 text-sm font-medium rounded-lg transition-all disabled:opacity-30 disabled:cursor-not-allowed'
			: 'adr-execute-inactive flex items-center justify-center gap-2 w-full px-4 py-2.5 text-sm font-medium rounded-lg transition-all disabled:opacity-30 disabled:cursor-not-allowed'
	);

	/** 进度百分比 */
	let progressPercent = $derived(
		progress ? Math.round((progress.current / progress.total) * 100) : 0
	);

	/** 预览重命名结果 */
	async function handlePreview() {
		if (methods.length === 0 || files.length === 0) return;

		loadingStore.set(true);
		statusMessageStore.set('正在生成预览...');
		progress = { current: 0, total: files.length, phase: '预览' };

		try {
			const result = await previewRename({ methods });
			previewStore.set(result.files);

			if (result.error) {
				statusMessageStore.set('预览失败: ' + result.error);
			} else {
				progress.current = files.length;
				statusMessageStore.set(
					`预览完成: ${result.changedCount} 个文件将重命名` +
					(result.conflictCount > 0 ? `，${result.conflictCount} 个冲突` : '')
				);
			}
		} catch (e) {
			statusMessageStore.set('预览异常: ' + String(e));
		} finally {
			loadingStore.set(false);
			setTimeout(() => { progress = null; }, 800);
		}
	}

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
			<span class="text-[10px] opacity-40 bg-green-500/10 text-green-400/60 px-1.5 py-0.5 rounded">可撤销</span>
		{/if}
	</div>

	<!-- 进度条 -->
	{#if progress}
		<div class="px-3 py-2 border-b border-surface-500/10">
			<div class="flex items-center justify-between text-xs mb-1">
				<span class="opacity-50">{progress.phase}中...</span>
				<span class="opacity-40">{progress.current}/{progress.total}</span>
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
		<!-- 预览按钮 -->
		<button
			class="adr-action-btn flex items-center justify-center gap-2 w-full px-4 py-2.5 text-sm font-medium rounded-lg
				bg-blue-500/90 hover:bg-blue-500 text-white transition-all
				disabled:opacity-30 disabled:cursor-not-allowed"
			onclick={handlePreview}
			disabled={loading || methods.length === 0 || files.length === 0}
		>
			<svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
				<path d="M10 12a2 2 0 100-4 2 2 0 000 4z"/>
				<path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"/>
			</svg>
			预览重命名
		</button>

		<!-- 执行按钮 -->
		<button
			class={executeBtnClass}
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
			class="adr-undo-btn flex items-center justify-center gap-2 w-full px-4 py-2 text-xs font-medium rounded-lg
				transition-all disabled:opacity-20 disabled:cursor-not-allowed
				{hasUndoHistory ? 'bg-amber-500/15 text-amber-400/80 hover:bg-amber-500/25 border border-amber-500/20' : 'bg-surface-500/5 opacity-30'}"
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
				<div class="flex justify-between text-green-400/80">
					<span>将重命名</span>
					<span>{stats.changed}</span>
				</div>
				{#if stats.conflicts > 0}
					<div class="flex justify-between text-amber-400/80">
						<span>冲突</span>
						<span>{stats.conflicts}</span>
					</div>
				{/if}
				<div class="flex justify-between opacity-40">
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

	<!-- 标签参考 -->
	<div class="flex-1 overflow-y-auto px-3 py-2 border-t border-surface-500/20">
		<h3 class="text-xs font-medium opacity-50 mb-2">标签参考</h3>
		<div class="space-y-1 text-xs">
			{#each [
				{ tag: '<Name>', desc: '原始文件名' },
				{ tag: '<Ext>', desc: '文件扩展名' },
				{ tag: '<Index>', desc: '文件序号' },
				{ tag: '<Date:YYYY>', desc: '当前年份' },
				{ tag: '<Date:YYYYMMDD>', desc: '当前日期' },
				{ tag: '<Time:HHmmss>', desc: '当前时间' },
				{ tag: '<Inc:3>', desc: '递增序号(3位)' },
				{ tag: '<Inc:3:100>', desc: '从100开始(3位)' },
				{ tag: '<Cnt:3>', desc: '总文件数(3位)' },
			] as ref}
				<div class="flex items-center gap-2">
					<code class="adr-mono text-blue-400/70 bg-surface-500/10 px-1 py-0.5 rounded text-[10px]">{ref.tag}</code>
					<span class="opacity-40">{ref.desc}</span>
				</div>
			{/each}
		</div>
	</div>
</aside>

<style>
	.adr-control-panel {
		background: var(--color-surface-200);
		width: var(--adr-panel-width);
		min-width: var(--adr-panel-width);
	}

	.adr-action-btn:active:not(:disabled) {
		transform: scale(0.98);
	}

	.adr-execute-active {
		background: rgba(16, 185, 129, 0.9);
		color: white;
	}

	.adr-execute-active:hover {
		background: rgb(16, 185, 129);
	}

	.adr-execute-inactive {
		background: rgba(128, 128, 128, 0.15);
		color: rgba(128, 128, 128, 0.5);
	}

	.adr-undo-btn:active:not(:disabled) {
		transform: scale(0.98);
	}
</style>
