<!-- FlowRename - RemoveMethodEditor 删除方法配置编辑器 -->
<script lang="ts">
	import type { RemoveConfig } from '$lib/types';

	let { config, onChange } = $props<{
		config: RemoveConfig;
		onChange: (updated: RemoveConfig) => void;
	}>();

	function update(partial: Partial<RemoveConfig>) {
		onChange({ ...config, ...partial });
	}
</script>

<div class="space-y-2 text-xs">
	<!-- 删除数量 -->
	<div class="flex items-center gap-2">
		<label for="remove-count" class="w-14 text-xs opacity-60 shrink-0">数量</label>
		<input
			id="remove-count"
			type="number"
			min="1"
			class="w-20 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20"
			value={config.count}
			oninput={(e) => update({ count: Math.max(1, parseInt((e.target as HTMLInputElement).value) || 1) })}
		/>
		<span class="opacity-50">个字符</span>
	</div>

	<!-- 删除位置 -->
	<div class="flex items-center gap-2">
		<span class="w-14 text-xs opacity-60 shrink-0">位置</span>
		<div class="flex gap-1">
			<button
				class="px-2 py-0.5 rounded text-[11px] transition-colors {config.position === 'Start' ? 'bg-blue-500/30 text-blue-300' : 'bg-surface-500/10 opacity-55 hover:opacity-75'}"
				onclick={() => update({ position: 'Start' })}
			>开头</button>
			<button
				class="px-2 py-0.5 rounded text-[11px] transition-colors {config.position === 'End' ? 'bg-blue-500/30 text-blue-300' : 'bg-surface-500/10 opacity-55 hover:opacity-75'}"
				onclick={() => update({ position: 'End' })}
			>末尾</button>
		</div>
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
