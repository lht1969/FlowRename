<!-- AdRename - MethodPanel 组件（左侧方法面板 - 含配置编辑器） -->
<script lang="ts">
	import { methodsStore } from '$lib/stores/app';
	import type { MethodConfig } from '$lib/types';
	import MethodEditor from './method-editors/MethodEditor.svelte';

	/** 可用方法列表 */
	const METHOD_TYPES = [
		{ id: 'Replace', label: '替换', icon: '⟶', desc: '查找并替换文本' },
		{ id: 'Add', label: '添加', icon: '＋', desc: '在指定位置插入文本' },
		{ id: 'Remove', label: '删除', icon: '✕', desc: '移除指定位置的字符' },
		{ id: 'NewCase', label: '大小写', icon: 'Aa', desc: '更改字母大小写' },
		{ id: 'Move', label: '移动', icon: '↔', desc: '移动文件名中的字符' },
		{ id: 'Trim', label: '修剪', icon: '✂', desc: '修剪首尾字符' },
		{ id: 'Renumber', label: '编号', icon: '#', desc: '添加序号编号' },
	] as const;

	const SPECIAL_METHOD_TYPES = [
		{ id: 'NewName', label: '新名称', icon: '✎', desc: '使用模板生成新名称' },
		{ id: 'List', label: '列表', icon: '☰', desc: '从名称列表重命名' },
		{ id: 'Timestamp', label: '时间戳', icon: '🕐', desc: '使用文件时间戳' }
	] as const;

	/** 方法类型 → 左侧色条颜色 */
	const METHOD_COLORS: Record<string, string> = {
		Replace: 'bg-blue-500',
		Add: 'bg-emerald-500',
		Remove: 'bg-red-500',
		NewCase: 'bg-purple-500',
		NewName: 'bg-amber-500',
		List: 'bg-cyan-500',
		Move: 'bg-orange-500',
		Trim: 'bg-pink-500',
		Renumber: 'bg-teal-500',
		Timestamp: 'bg-violet-500',
	};

	let methods = $derived($methodsStore);

	/** 当前展开编辑的方法索引 */
	let expandedIndex = $state<number | null>(null);

	/** 添加方法到 Pipeline */
	function addMethod(typeId: string) {
		const defaultConfig = createDefaultConfig(typeId);
		if (defaultConfig) {
			methodsStore.update((m) => [...m, defaultConfig]);
			// 自动展开新添加的方法
			expandedIndex = methods.length;
		}
	}

	/** 移除方法 */
	function removeMethod(index: number) {
		methodsStore.update((m) => m.filter((_, i) => i !== index));
		if (expandedIndex === index) {
			expandedIndex = null;
		} else if (expandedIndex !== null && expandedIndex > index) {
			expandedIndex--;
		}
	}

	/** 上移方法 */
	function moveUp(index: number) {
		if (index <= 0) return;
		methodsStore.update((m) => {
			const arr = [...m];
			[arr[index - 1], arr[index]] = [arr[index], arr[index - 1]];
			return arr;
		});
		if (expandedIndex === index) expandedIndex = index - 1;
		else if (expandedIndex === index - 1) expandedIndex = index;
	}

	/** 下移方法 */
	function moveDown(index: number) {
		methodsStore.update((m) => {
			if (index >= m.length - 1) return m;
			const arr = [...m];
			[arr[index], arr[index + 1]] = [arr[index + 1], arr[index]];
			return arr;
		});
		if (expandedIndex === index) expandedIndex = index + 1;
		else if (expandedIndex === index + 1) expandedIndex = index;
	}

	/** 切换方法编辑器展开/折叠 */
	function toggleExpand(index: number) {
		expandedIndex = expandedIndex === index ? null : index;
	}

	/** 更新方法配置 */
	function updateMethodConfig(index: number, updated: MethodConfig) {
		methodsStore.update((m) => {
			const arr = [...m];
			arr[index] = updated;
			return arr;
		});
	}

	/** 切换方法启用/禁用状态 */
	function toggleEnabled(index: number, e: Event) {
		const checkbox = e.target as HTMLInputElement;
		methodsStore.update((m) => {
			const arr = [...m];
			const method = arr[index];
			const typeKey = getMethodType(method);
			if (typeKey) {
				const config = (method as any)[typeKey];
				if (config) {
					config.enabled = checkbox.checked;
				}
			}
			return arr;
		});
	}

	/** 获取方法的启用状态 */
	function isMethodEnabled(config: MethodConfig): boolean {
		const typeKey = getMethodType(config);
		if (!typeKey) return true;
		const configData = (config as any)[typeKey];
		return configData?.enabled !== false;
	}

	/** 创建方法的默认配置 */
	function createDefaultConfig(typeId: string): MethodConfig | null {
		switch (typeId) {
			case 'Replace':
			return { Replace: { enabled: true, find: '', replaceWith: '', occurrence: 'All', caseSensitive: false, useRegex: false, applyTo: 'Name' } };
			case 'Add':
			return { Add: { enabled: true, text: '', position: 'Start', customIndex: null, backwards: false, applyTo: 'Name' } };
			case 'Remove':
			return { Remove: { enabled: true, count: 1, position: 'Start', applyTo: 'Name' } };
			case 'NewCase':
			return { NewCase: { enabled: true, newCase: 'Upper', location: 'All', applyTo: 'Name' } };
			case 'NewName':
			return { NewName: { enabled: true, template: '<Name>', applyTo: 'Name' } };
			case 'List':
			return { List: { enabled: true, names: [], overflowBehavior: 'KeepOriginal', applyTo: 'Name' } };
			case 'Move':
			return { Move: { enabled: true, fromStart: 0, count: 1, toPosition: 0, applyTo: 'Name' } };
			case 'Trim':
			return { Trim: { enabled: true, trimStart: '', trimEnd: '', trimWhitespace: false, applyTo: 'Name' } };
			case 'Renumber':
			return { Renumber: { enabled: true, start: 1, step: 1, padding: 3, position: 'Prefix', separator: '_', applyTo: 'Name' } };
			case 'Timestamp':
			return { Timestamp: { enabled: true, source: 'Modified', format: 'YYYY-MM-DD', applyTo: 'Name' } };
			default:
				return null;
		}
	}

	/** 获取方法的显示名称 */
	function getMethodLabel(config: MethodConfig): string {
		if ('Replace' in config) return '替换';
		if ('Add' in config) return '添加';
		if ('Remove' in config) return '删除';
		if ('NewCase' in config) return '大小写';
		if ('NewName' in config) return '新名称';
		if ('List' in config) return '列表';
		if ('Move' in config) return '移动';
		if ('Trim' in config) return '修剪';
		if ('Renumber' in config) return '编号';
		if ('Timestamp' in config) return '时间戳';
		return '未知';
	}

	/** 获取方法的图标 */
	function getMethodIcon(config: MethodConfig): string {
		if ('Replace' in config) return '⟶';
		if ('Add' in config) return '＋';
		if ('Remove' in config) return '✕';
		if ('NewCase' in config) return 'Aa';
		if ('NewName' in config) return '✎';
		if ('List' in config) return '☰';
		if ('Move' in config) return '↔';
		if ('Trim' in config) return '✂';
		if ('Renumber' in config) return '#';
		if ('Timestamp' in config) return '🕐';
		return '?';
	}

	/** 获取方法类型标识 */
	function getMethodType(config: MethodConfig): string {
		if ('Replace' in config) return 'Replace';
		if ('Add' in config) return 'Add';
		if ('Remove' in config) return 'Remove';
		if ('NewCase' in config) return 'NewCase';
		if ('NewName' in config) return 'NewName';
		if ('List' in config) return 'List';
		if ('Move' in config) return 'Move';
		if ('Trim' in config) return 'Trim';
		if ('Renumber' in config) return 'Renumber';
		if ('Timestamp' in config) return 'Timestamp';
		return '';
	}

	/** 获取方法的简要描述 */
	function getMethodSummary(config: MethodConfig): string {
		if ('Replace' in config) {
			const c = config.Replace;
			return c.find ? `"${c.find}" → "${c.replaceWith}"` : '未配置';
		}
		if ('Add' in config) {
			const c = config.Add;
			return c.text ? `"${c.text}" @ ${typeof c.position === 'string' ? c.position : '自定义'}` : '未配置';
		}
		if ('Remove' in config) {
			const c = config.Remove;
			return `${c.count}字符 @ ${c.position === 'Start' ? '开头' : '末尾'}`;
		}
		if ('NewCase' in config) {
			const c = config.NewCase;
			return c.newCase;
		}
		if ('NewName' in config) {
			const c = config.NewName;
			return c.template;
		}
		if ('List' in config) {
			const c = config.List;
			return c.names.length > 0 ? `${c.names.length} 个名称` : '未配置';
		}
		if ('Move' in config) {
			const c = config.Move;
			return `${c.count}字符 → 位置${c.toPosition}`;
		}
		if ('Trim' in config) {
			const c = config.Trim;
			return c.trimWhitespace ? '修剪空白' : '修剪字符';
		}
		if ('Renumber' in config) {
			const c = config.Renumber;
			return `${String(c.start).padStart(c.padding, '0')}+${c.step}`;
		}
		if ('Timestamp' in config) {
			const c = config.Timestamp;
			return c.format;
		}
		return '';
	}
</script>

<aside class="adr-method-panel flex flex-col h-full overflow-hidden">
	<!-- 面板标题 -->
	<div class="flex items-center justify-between px-3 py-2 border-b border-surface-500/20">
		<h2 class="text-sm font-semibold opacity-80">重命名方法</h2>
		<span class="text-xs opacity-55">{methods.length} 个方法</span>
	</div>

	<!-- 方法添加按钮组 -->
	<div class="flex flex-wrap gap-1 px-3 py-2 border-b border-surface-500/10">
		{#each METHOD_TYPES as method}
			<button
				class="adr-method-btn flex items-center gap-1 px-2 py-1 text-xs rounded-md
					hover:bg-surface-500/20 transition-colors border border-surface-500/10"
				onclick={() => addMethod(method.id)}
				title={method.desc}
			>
				<span class="opacity-65">{method.icon}</span>
				<span>{method.label}</span>
			</button>
		{/each}
	</div>
	<!-- 特殊方法一行（新名称、列表、时间戳） -->
	<div class="flex flex-wrap gap-1 px-3 py-2 border-b border-surface-500/10">
		{#each SPECIAL_METHOD_TYPES as method}
			<button
				class="adr-method-btn flex items-center gap-1 px-2 py-1 text-xs rounded-md
					hover:bg-surface-500/20 transition-colors border border-surface-500/10"
				onclick={() => addMethod(method.id)}
				title={method.desc}
			>
				<span class="opacity-65">{method.icon}</span>
				<span>{method.label}</span>
			</button>
		{/each}
	</div>

	<!-- 已添加的方法列表（Pipeline） -->
	<div class="flex-1 overflow-y-auto px-2 py-2">
		{#if methods.length === 0}
			<div class="flex flex-col items-center justify-center h-full opacity-55 text-center px-4">
				<svg class="w-8 h-8 mb-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M12 4v16m8-8H4" />
				</svg>
				<p class="text-xs">点击上方按钮添加重命名方法</p>
			</div>
		{:else}
			{#each methods as method, index (index)}
				<div class="adr-method-item mb-2 rounded-lg border border-surface-500/15 overflow-hidden transition-all duration-[var(--adr-transition-normal)]
					hover:border-surface-500/30
					{expandedIndex === index ? 'ring-1 ring-blue-500/30 border-blue-500/20' : ''}
					{!isMethodEnabled(method) ? 'opacity-50' : ''}">
					<!-- 左侧色条 -->
					<div class="flex">
						<div class="w-[3px] shrink-0 {!isMethodEnabled(method) ? 'bg-surface-500' : (METHOD_COLORS[getMethodType(method)] || 'bg-surface-500')}"></div>
						<div class="flex-1 min-w-0">
							<!-- 方法头部 -->
							<div class="relative">
								<button type="button" class="group flex items-center gap-2.5 px-3 py-2 w-full text-left cursor-pointer hover:bg-surface-500/5"
									onclick={() => toggleExpand(index)}>
									<!-- 展开/折叠箭头 -->
									<svg class="w-3 h-3 opacity-50 transition-transform duration-[var(--adr-transition-fast)] {expandedIndex === index ? 'rotate-90' : ''}"
										viewBox="0 0 20 20" fill="currentColor">
										<path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/>
									</svg>

									<!-- 方法序号 -->
									<span class="text-xs opacity-50 w-4 text-center shrink-0">{index + 1}</span>

									<!-- 方法图标 -->
									<span class="text-sm shrink-0">{getMethodIcon(method)}</span>

									<!-- 方法名称 + 摘要 -->
									<div class="flex-1 min-w-0">
										<span class="text-sm font-medium">{getMethodLabel(method)}</span>
										{#if expandedIndex !== index}
											<span class="text-[11px] opacity-50 ml-1.5 adr-truncate">{getMethodSummary(method)}</span>
										{/if}
									</div>
								</button>

								<!-- 操作按钮 -->
								<div class="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-0.5 transition-opacity" role="group">
									<button
										class="p-1 rounded hover:bg-surface-500/20 opacity-40 hover:opacity-80 transition-all"
										onclick={(e) => { e.stopPropagation(); moveUp(index); }}
										disabled={index === 0}
										title="上移"
									>
										<svg class="w-3 h-3" viewBox="0 0 20 20" fill="currentColor"><path d="M10 4l-6 6h4v6h4v-6h4z"/></svg>
									</button>
									<button
										class="p-1 rounded hover:bg-surface-500/20 opacity-40 hover:opacity-80 transition-all"
										onclick={(e) => { e.stopPropagation(); moveDown(index); }}
										disabled={index === methods.length - 1}
										title="下移"
									>
										<svg class="w-3 h-3" viewBox="0 0 20 20" fill="currentColor"><path d="M10 16l6-6h-4V4H8v6H4z"/></svg>
									</button>
									<input
										type="checkbox"
										class="w-4 h-4 rounded border-surface-500/30 bg-surface-200 text-blue-500 focus:ring-blue-500/20 cursor-pointer"
										checked={isMethodEnabled(method)}
										onchange={(e) => toggleEnabled(index, e)}
										title="启用/禁用此方法"
									/>
									<button
										class="p-1 rounded hover:bg-red-500/20 text-red-500 opacity-40 hover:opacity-100 transition-all"
										onclick={(e) => { e.stopPropagation(); removeMethod(index); }}
										title="移除"
									>
										<svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/></svg>
									</button>
								</div>
							</div>

							<!-- 方法配置编辑器 -->
							{#if expandedIndex === index}
								<div class="px-3 py-2.5 border-t border-surface-500/10 bg-surface-500/5">
									<MethodEditor
										config={method}
										{index}
										onUpdate={updateMethodConfig}
									/>
								</div>
							{/if}
						</div>
					</div>
				</div>
			{/each}
		{/if}
	</div>
</aside>

<style>
	.adr-method-panel {
		background: var(--color-surface-200);
		width: var(--adr-sidebar-width);
		min-width: var(--adr-sidebar-width);
	}

	.adr-method-btn:active {
		transform: scale(0.95);
	}
</style>
