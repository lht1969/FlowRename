<!-- AdRename - ReplaceMethodEditor 替换方法配置编辑器 -->
<script lang="ts">
	import type { ReplaceConfig } from '$lib/types';

	let { config, onChange } = $props<{
		config: ReplaceConfig;
		onChange: (updated: ReplaceConfig) => void;
	}>();

	function update(partial: Partial<ReplaceConfig>) {
		onChange({ ...config, ...partial });
	}
</script>

<div class="space-y-2 text-xs">
	<!-- 查找文本 -->
	<div class="flex items-center gap-2">
		<label for="replace-find" class="w-12 opacity-50 shrink-0">查找</label>
		<input
			id="replace-find"
			type="text"
			class="flex-1 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50 transition-colors"
			placeholder="输入要查找的文本"
			value={config.find}
			oninput={(e) => update({ find: (e.target as HTMLInputElement).value })}
		/>
	</div>

	<!-- 替换文本 -->
	<div class="flex items-center gap-2">
		<label for="replace-with" class="w-12 opacity-50 shrink-0">替换为</label>
		<input
			id="replace-with"
			type="text"
			class="flex-1 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50 transition-colors"
			placeholder="替换后的文本"
			value={config.replaceWith}
			oninput={(e) => update({ replaceWith: (e.target as HTMLInputElement).value })}
		/>
	</div>

	<!-- 匹配范围 -->
	<div class="flex items-center gap-2">
		<label for="replace-occurrence" class="w-12 opacity-50 shrink-0">范围</label>
		<select
			id="replace-occurrence"
			class="flex-1 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50"
			value={typeof config.occurrence === 'string' ? config.occurrence : 'Custom'}
			onchange={(e) => {
				const val = (e.target as HTMLSelectElement).value;
				if (val === 'Custom') {
					update({ occurrence: { Custom: 1 } });
				} else {
					update({ occurrence: val as 'All' | 'First' | 'Last' });
				}
			}}
		>
			<option value="All">全部匹配</option>
			<option value="First">第一个</option>
			<option value="Last">最后一个</option>
			<option value="Custom">自定义位置</option>
		</select>
	</div>

	<!-- 自定义位置（仅当选择 Custom 时显示） -->
	{#if typeof config.occurrence === 'object' && 'Custom' in config.occurrence}
		<div class="flex items-center gap-2">
			<label for="replace-custom-pos" class="w-12 opacity-50 shrink-0">位置</label>
			<input
				id="replace-custom-pos"
				type="number"
				min="1"
				class="w-20 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
					focus:outline-none focus:border-blue-500/50"
				value={config.occurrence.Custom}
				oninput={(e) => update({ occurrence: { Custom: Math.max(1, parseInt((e.target as HTMLInputElement).value) || 1) } })}
			/>
		</div>
	{/if}

	<!-- 选项行 -->
	<div class="flex items-center gap-3 pt-1">
		<label class="flex items-center gap-1 cursor-pointer opacity-60 hover:opacity-80 transition-opacity">
			<input type="checkbox" checked={config.caseSensitive} onchange={() => update({ caseSensitive: !config.caseSensitive })} class="w-3 h-3" />
			<span>区分大小写</span>
		</label>
		<label class="flex items-center gap-1 cursor-pointer opacity-60 hover:opacity-80 transition-opacity">
			<input type="checkbox" checked={config.useRegex} onchange={() => update({ useRegex: !config.useRegex })} class="w-3 h-3" />
			<span>正则表达式</span>
		</label>
	</div>

	<!-- 应用目标 -->
	<div class="flex items-center gap-2">
		<span class="w-12 opacity-50 shrink-0">应用</span>
		<div class="flex gap-1">
			{#each ['Name', 'Extension', 'Both'] as target}
				<button
					class="px-2 py-0.5 rounded text-[10px] transition-colors {config.applyTo === target ? 'bg-blue-500/30 text-blue-300' : 'bg-surface-500/10 opacity-50 hover:opacity-70'}"
					onclick={() => update({ applyTo: target as 'Name' | 'Extension' | 'Both' })}
				>
					{target === 'Name' ? '文件名' : target === 'Extension' ? '扩展名' : '全部'}
				</button>
			{/each}
		</div>
	</div>
</div>
