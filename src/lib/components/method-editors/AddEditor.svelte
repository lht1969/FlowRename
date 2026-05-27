<!-- AdRename - AddMethodEditor 添加方法配置编辑器 -->
<script lang="ts">
	import type { AddConfig } from '$lib/types';

	let { config, onChange } = $props<{
		config: AddConfig;
		onChange: (updated: AddConfig) => void;
	}>();

	function update(partial: Partial<AddConfig>) {
		onChange({ ...config, ...partial });
	}
</script>

<div class="space-y-2 text-xs">
	<!-- 添加文本 -->
	<div class="flex items-center gap-2">
		<label for="add-text" class="w-14 text-xs opacity-60 shrink-0">文本</label>
		<input
			id="add-text"
			type="text"
			class="flex-1 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 transition-colors"
			placeholder="要添加的文本"
			value={config.text}
			oninput={(e) => update({ text: (e.target as HTMLInputElement).value })}
		/>
	</div>

	<!-- 插入位置 -->
	<div class="flex items-center gap-2">
		<label for="add-position" class="w-14 text-xs opacity-60 shrink-0">位置</label>
		<select
			id="add-position"
			class="flex-1 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20"
			value={typeof config.position === 'string' ? config.position : 'Custom'}
			onchange={(e) => {
				const val = (e.target as HTMLSelectElement).value;
				if (val === 'Custom') {
					update({ position: { Custom: 0 }, customIndex: 0 });
				} else {
					update({ position: val as 'Start' | 'End', customIndex: null });
				}
			}}
		>
			<option value="Start">开头</option>
			<option value="End">末尾</option>
			<option value="Custom">自定义位置</option>
		</select>
	</div>

	<!-- 自定义位置 -->
	{#if typeof config.position === 'object' && 'Custom' in config.position}
		<div class="flex items-center gap-2">
			<label for="add-custom-index" class="w-14 text-xs opacity-60 shrink-0">索引</label>
			<input
				id="add-custom-index"
				type="number"
				min="0"
				class="w-20 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
					focus:outline-none focus:border-blue-500/50"
				value={config.position.Custom}
				oninput={(e) => update({ position: { Custom: Math.max(0, parseInt((e.target as HTMLInputElement).value) || 0) }, customIndex: Math.max(0, parseInt((e.target as HTMLInputElement).value) || 0) })}
			/>
		</div>
	{/if}

	<!-- 选项行 -->
	<div class="flex items-center gap-3 pt-1">
		<label class="flex items-center gap-1 cursor-pointer opacity-60 hover:opacity-80 transition-opacity">
			<input type="checkbox" checked={config.backwards} onchange={() => update({ backwards: !config.backwards })} class="w-3.5 h-3.5 rounded border-surface-500/30 text-blue-500 focus:ring-blue-500/30 focus:ring-offset-0" />
			<span>从末尾计数</span>
		</label>
	</div>

	<!-- 应用目标 -->
	<div class="flex items-center gap-2">
		<span class="w-14 text-xs opacity-60 shrink-0">应用</span>
		<div class="flex gap-1">
			{#each ['Name', 'Extension', 'Both'] as target}
				<button
					class="px-2 py-0.5 rounded text-[11px] transition-colors {config.applyTo === target ? 'bg-blue-500/30 text-blue-300' : 'bg-surface-500/10 opacity-55 hover:opacity-75'}"
					onclick={() => update({ applyTo: target as 'Name' | 'Extension' | 'Both' })}
				>
					{target === 'Name' ? '文件名' : target === 'Extension' ? '扩展名' : '全部'}
				</button>
			{/each}
		</div>
	</div>
</div>
