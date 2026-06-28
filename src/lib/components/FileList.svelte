<!-- FlowRename - FileList 组件（中间文件列表 - 增强版） -->
<script lang="ts">
	import { filesStore, previewStore, loadingStore, statusMessageStore } from '$lib/stores/app';
	import { scanDirectory, scanFiles, clearFiles } from '$lib/commands';
	import { currentDirStore, recursiveStore, extensionFilterStore } from '$lib/stores/app';
	import { detectFileCategories } from '$lib/stores/fileCategories';
	import { compareFileName } from '$lib/utils/sort';
	import { onMount } from 'svelte';
	import type { FilePreviewItem } from '$lib/types';

	let files = $derived($filesStore);
	let preview = $derived($previewStore);
	let loading = $derived($loadingStore);
	let currentDir = $derived($currentDirStore);

	/** 排序状态 */
	let sortField = $state<'name' | 'size' | 'modified'>('name');
	let sortAsc = $state(true);

	/** 扩展名过滤输入 */
	let extFilterInput = $state('');
	let showExtFilter = $state(false);

	/** 拖放状态 */
	let isDragOver = $state(false);

	/** 设置 Tauri 拖放事件监听 */
	onMount(async () => {
		try {
			const { getCurrentWindow } = await import('@tauri-apps/api/window');
			const appWindow = getCurrentWindow();
			await appWindow.onDragDropEvent((event) => {
				const ev = event.payload;
				if (ev.type === 'enter' || ev.type === 'over') {
					isDragOver = true;
				} else if (ev.type === 'leave') {
					isDragOver = false;
				} else if (ev.type === 'drop') {
					isDragOver = false;
					if (ev.paths.length > 0) {
						scanSelectedFiles(ev.paths);
					}
				}
			});
		} catch {
			// Tauri 环境未就绪时忽略
		}
	});

	/** 构建预览名称映射 */
	let previewMap = $derived(
		new Map<string, FilePreviewItem>(
			preview.map((p) => [p.originalPath, p])
		)
	);

	/** 排序后的文件列表 */
	let sortedFiles = $derived(() => {
		const arr = [...files];
		const dir = sortAsc ? 1 : -1;
		arr.sort((a, b) => {
			switch (sortField) {
				case 'name':
					return dir * compareFileName(a.originalName, b.originalName);
				case 'size':
					return dir * (a.fileSize - b.fileSize);
				case 'modified':
					return dir * a.modifiedTime.localeCompare(b.modifiedTime);
				default:
					return 0;
			}
		});
		return arr;
	});

	/** 切换排序 */
	function toggleSort(field: 'name' | 'size' | 'modified') {
		if (sortField === field) {
			sortAsc = !sortAsc;
		} else {
			sortField = field;
			sortAsc = true;
		}
	}

	/** 选择目录并扫描 */
	async function handleSelectDirectory() {
		try {
			const { open } = await import('@tauri-apps/plugin-dialog');
			const selected = await open({
				directory: true,
				multiple: false,
				title: '选择要重命名的文件目录'
			});
			if (selected && typeof selected === 'string') {
				currentDirStore.set(selected);
				await scanDir(selected);
			}
		} catch {
			// Tauri 环境未就绪时忽略
		}
	}

	/** 执行目录扫描 */
	async function scanDir(path: string) {
		loadingStore.set(true);
		statusMessageStore.set('正在扫描目录...');
		try {
			const result = await scanDirectory({
				directoryPath: path,
				recursive: $recursiveStore,
				fileExtensions: $extensionFilterStore
			});
			if (result.error) {
				statusMessageStore.set('扫描失败: ' + result.error);
			} else {
				filesStore.set(result.files);
				detectFileCategories(result.files);
				statusMessageStore.set(`已加载 ${result.totalCount} 个文件`);
			}
		} catch (e) {
			statusMessageStore.set('扫描异常: ' + String(e));
		} finally {
			loadingStore.set(false);
		}
	}

	/** 选择多个文件 */
	async function handleSelectFiles() {
		try {
			const { open } = await import('@tauri-apps/plugin-dialog');
			const selected = await open({
				multiple: true,
				title: '选择要重命名的文件'
			});
			if (selected && Array.isArray(selected) && selected.length > 0) {
				currentDirStore.set(null);
				await scanSelectedFiles(selected);
			}
		} catch {
			// Tauri 环境未就绪时忽略
		}
	}

	/** 扫描选中的文件 */
	async function scanSelectedFiles(paths: string[]) {
		loadingStore.set(true);
		statusMessageStore.set('正在加载文件...');
		try {
			const result = await scanFiles({ filePaths: paths });
			if (result.error) {
				statusMessageStore.set('加载失败: ' + result.error);
			} else {
				filesStore.set(result.files);
				detectFileCategories(result.files);
				statusMessageStore.set(`已加载 ${result.totalCount} 个文件`);
			}
		} catch (e) {
			statusMessageStore.set('加载异常: ' + String(e));
		} finally {
			loadingStore.set(false);
		}
	}

	/** 清空文件列表 */
	async function handleClear() {
		await clearFiles();
		filesStore.set([]);
		previewStore.set([]);
		detectFileCategories([]);
		statusMessageStore.set('就绪');
	}

	/** 应用扩展名过滤 */
	function applyExtFilter() {
		const exts = extFilterInput
			.split(/[,;.\s]+/)
			.map((e) => e.trim().replace(/^\./, ''))
			.filter((e) => e.length > 0);
		extensionFilterStore.set(exts);
		if (currentDir) scanDir(currentDir);
	}

	/** 清除扩展名过滤 */
	function clearExtFilter() {
		extFilterInput = '';
		extensionFilterStore.set([]);
		if (currentDir) scanDir(currentDir);
	}

	/** 拖放处理 */
	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		isDragOver = true;
	}

	function handleDragLeave() {
		isDragOver = false;
	}

	async function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragOver = false;
		// Tauri 拖放暂不处理文件路径，使用目录选择器
	}

	/** 格式化文件大小 */
	function formatSize(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	}

	/** 获取排序指示器 */
	function sortIndicator(field: string): string {
		if (sortField !== field) return '';
		return sortAsc ? ' ↑' : ' ↓';
	}

	/** 获取文件行状态样式 */
	function getRowClass(previewItem?: FilePreviewItem): string {
		if (previewItem?.hasConflict) return 'adr-conflict-row';
		return '';
	}
</script>

<main
	class="adr-file-list flex flex-col h-full overflow-hidden {isDragOver ? 'adr-drop-zone active' : ''}"
	ondragover={handleDragOver}
	ondragleave={handleDragLeave}
	ondrop={handleDrop}
>
	<!-- 工具栏 -->
	<div class="flex items-center gap-2 px-3 py-2 border-b border-surface-500/20">
		<!-- 选择文件按钮 -->
		<button
			class="adr-btn-primary flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-md
				bg-blue-500/90 hover:bg-blue-500 text-white transition-colors"
			onclick={handleSelectFiles}
			disabled={loading}
		>
			<svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor">
				<path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z" clip-rule="evenodd"/>
			</svg>
			选择文件
		</button>

		<!-- 选择目录按钮 -->
		<button
			class="adr-btn-primary flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-md
				bg-blue-500/90 hover:bg-blue-500 text-white transition-colors"
			onclick={handleSelectDirectory}
			disabled={loading}
		>
			<svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor">
				<path fill-rule="evenodd" d="M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" clip-rule="evenodd"/>
			</svg>
			选择目录
		</button>

		<!-- 当前目录路径 -->
		{#if currentDir}
			<span class="text-xs opacity-55 adr-truncate flex-1" title={currentDir}>
				{currentDir}
			</span>
		{/if}

		<!-- 递归选项 -->
		<label class="flex items-center gap-1 text-xs opacity-65 cursor-pointer">
			<input type="checkbox" bind:checked={$recursiveStore} class="w-3 h-3" />
			递归
		</label>

		<!-- 扩展名过滤 -->
		<button
			class="p-1 rounded hover:bg-surface-500/20 transition-colors opacity-60 hover:opacity-80"
			onclick={() => showExtFilter = !showExtFilter}
			title="扩展名过滤"
		>
			<svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor">
				<path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd"/>
			</svg>
		</button>

		<!-- 清空按钮 -->
		{#if files.length > 0}
			<button
				class="p-1 rounded hover:bg-surface-500/20 transition-colors opacity-50 hover:opacity-75"
				onclick={handleClear}
				title="清空列表"
			>
				<svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor">
					<path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd"/>
				</svg>
			</button>
		{/if}
	</div>

	<!-- 扩展名过滤条 -->
	{#if showExtFilter}
		<div class="flex items-center gap-2 px-3 py-1.5 border-b border-surface-500/10 bg-surface-500/5">
			<span class="text-xs opacity-50">扩展名:</span>
			<input
				type="text"
				class="flex-1 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-0.5 text-xs
					focus:outline-none focus:border-blue-500/50"
				placeholder="jpg, png, mp3（逗号分隔）"
				value={extFilterInput}
				oninput={(e) => extFilterInput = (e.target as HTMLInputElement).value}
				onkeydown={(e) => { if (e.key === 'Enter') applyExtFilter(); }}
			/>
			<button class="text-xs text-blue-300/80 hover:text-blue-300" onclick={applyExtFilter}>应用</button>
			<button class="text-xs opacity-50 hover:opacity-70" onclick={clearExtFilter}>清除</button>
		</div>
	{/if}

	<!-- 文件列表 -->
	{#if files.length === 0}
		<div class="flex-1 flex flex-col items-center justify-center opacity-55 text-center px-8">
			<svg class="w-12 h-12 mb-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
				<path d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
			</svg>
			<p class="text-sm mb-1">暂无文件</p>
			<p class="text-xs">点击"选择目录"或拖放文件夹到此处</p>
		</div>
	{:else}
		<!-- 固定表头（在滚动区域外） -->
		<div class="adr-table-header grid grid-cols-[32px_1fr_70px_1fr] gap-2 px-3 py-1.5
			text-xs font-medium opacity-60 border-b border-surface-500/20 bg-surface-300/50 shrink-0">
			<span class="text-center">#</span>
			<button class="text-left hover:opacity-100 transition-opacity" onclick={() => toggleSort('name')}>
				原始名称{sortIndicator('name')}
			</button>
			<button class="text-right hover:opacity-100 transition-opacity" onclick={() => toggleSort('size')}>
				大小{sortIndicator('size')}
			</button>
			<span>预览名称</span>
		</div>

		<!-- 可滚动文件行 -->
		<div class="flex-1 overflow-y-auto">
			{#each sortedFiles() as file, index (file.id)}
				{@const previewItem = previewMap.get(file.originalPath)}
				<div
					class="adr-file-row grid grid-cols-[32px_1fr_70px_1fr] gap-2 px-3 py-1.5 text-xs
						border-b border-surface-500/5 transition-all duration-300 ease-out
						hover:bg-surface-500/10 hover:pl-[11px] hover:border-l-[3px] hover:border-l-blue-500/40
						{index % 2 === 1 ? 'bg-surface-500/[0.03]' : ''}
						{getRowClass(previewItem)}"
					data-preview-updated={previewItem?.isChanged ? 'true' : 'false'}
				>
					<!-- 行号 -->
					<span class="text-center opacity-30 text-[11px] select-none">{index + 1}</span>

					<!-- 原始名称 -->
					<span class="adr-truncate opacity-80" title={file.originalName + file.originalExt}>
						{file.originalName}<span class="opacity-50">{file.originalExt}</span>
					</span>

					<!-- 文件大小 -->
					<span class="text-right opacity-50">{formatSize(file.fileSize)}</span>

					<!-- 预览名称 -->
					{#if previewItem?.hasConflict}
						<span class="adr-truncate font-bold text-red-600 dark:text-red-400" title="冲突: {previewItem.newName}">
							🚫 {previewItem.newName}
						</span>
					{:else if previewItem?.isChanged}
						<span class="adr-truncate adr-preview-changed" title={previewItem.newName}>
							{previewItem.newName}
						</span>
					{:else}
						<span class="adr-truncate opacity-30">—</span>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</main>

<style>
	.adr-file-list {
		background: var(--color-surface-50);
	}

	.adr-btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.adr-file-row {
		animation: adr-fade-in 150ms ease forwards;
	}

	/* 预览更新时的平滑过渡高亮 */
	.adr-file-row[data-preview-updated="true"] {
		transition: background-color 0.3s ease-out, color 0.3s ease-out;
	}

	/* 冲突文件行 - 明显红色背景标记 */
	.adr-conflict-row {
		background-color: rgba(239, 68, 68, 0.15) !important;
		border-left: 3px solid rgba(239, 68, 68, 0.8) !important;
		animation: adr-conflict-pulse 1.5s ease-in-out infinite;
	}

	.adr-conflict-row:hover {
		background-color: rgba(239, 68, 68, 0.25) !important;
	}

	@keyframes adr-conflict-pulse {
		0%, 100% { background-color: rgba(239, 68, 68, 0.15); }
		50% { background-color: rgba(239, 68, 68, 0.25); }
	}

	/* Obsidian 主题预览名称使用更亮的绿色 */
	:global([data-theme="obsidian"]) .adr-preview-changed {
		color: #4ade80 !important;
	}

	/* 其他浅色主题默认使用深绿色 */
	.adr-preview-changed {
		color: #0a5f56;
	}
</style>
