<!-- FlowRename - NewCaseMethodEditor 大小写方法配置编辑器 -->
<script lang="ts">
	import type { NewCaseConfig } from '$lib/types';

	let { config, onChange } = $props<{
		config: NewCaseConfig;
		onChange: (updated: NewCaseConfig) => void;
	}>();

	function update(partial: Partial<NewCaseConfig>) {
		onChange({ ...config, ...partial });
	}

	const CASE_OPTIONS = [
		{ value: 'Upper', label: '全部大写', example: 'PHOTO' },
		{ value: 'Lower', label: '全部小写', example: 'photo' },
		{ value: 'Title', label: '首字母大写', example: 'Photo' },
		{ value: 'Sentence', label: '句首大写', example: 'Photo album' },
		{ value: 'Inverted', label: '反转大小写', example: 'pHOTO' }
	] as const;
</script>

<div class="space-y-2 text-xs">
	<!-- 大小写模式 -->
	<div class="space-y-1">
		{#each CASE_OPTIONS as opt}
			<button
				class="w-full flex items-center gap-2 px-2 py-1 rounded transition-colors
					{config.newCase === opt.value ? 'bg-blue-500/20 text-blue-300' : 'bg-surface-500/5 opacity-60 hover:opacity-80 hover:bg-surface-500/10'}"
				onclick={() => update({ newCase: opt.value as NewCaseConfig['newCase'] })}
			>
				<span class="w-3 h-3 rounded-full border {config.newCase === opt.value ? 'border-blue-400 bg-blue-400' : 'border-surface-500/40'}"></span>
				<span>{opt.label}</span>
				<code class="ml-auto adr-mono text-[11px] opacity-55">{opt.example}</code>
			</button>
		{/each}
	</div>

	<!-- 应用位置 -->
	<div class="flex items-center gap-2 pt-1">
		<span class="w-14 text-xs opacity-60 shrink-0">位置</span>
		<div class="flex gap-1">
			<button
				class="px-2 py-0.5 rounded text-[11px] transition-colors {config.location === 'All' ? 'bg-blue-500/30 text-blue-300' : 'bg-surface-500/10 opacity-55 hover:opacity-75'}"
				onclick={() => update({ location: 'All' })}
			>所有单词</button>
			<button
				class="px-2 py-0.5 rounded text-[11px] transition-colors {config.location === 'First' ? 'bg-blue-500/30 text-blue-300' : 'bg-surface-500/10 opacity-55 hover:opacity-75'}"
				onclick={() => update({ location: 'First' })}
			>仅首个</button>
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
