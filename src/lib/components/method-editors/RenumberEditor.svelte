<script lang="ts">
	import type { RenumberConfig } from '$lib/types';

	let { config, onChange } = $props<{
		config: RenumberConfig;
		onChange: (updated: RenumberConfig) => void;
	}>();

	function update(partial: Partial<RenumberConfig>) {
		onChange({ ...config, ...partial });
	}

	/** 位置选项 */
	const POSITION_OPTIONS = [
		{ value: 'Prefix' as const, label: '前缀', example: '001_file' },
		{ value: 'Suffix' as const, label: '后缀', example: 'file_001' },
		{ value: 'Replace' as const, label: '替换', example: '001' },
	];

	/** 预览当前配置的序号格式 */
	let previewNumber = $derived(
		String(config.start).padStart(config.padding, '0')
	);
</script>

<div class="space-y-2 text-xs">
	<!-- 起始编号 -->
	<div class="flex items-center gap-2">
		<label for="renumber-start" class="w-14 text-xs opacity-60 shrink-0">起始</label>
		<input
			id="renumber-start"
			type="number"
			min="0"
			class="w-20 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 adr-mono"
			value={config.start}
			oninput={(e) => update({ start: Math.max(0, parseInt((e.target as HTMLInputElement).value) || 0) })}
		/>
	</div>

	<!-- 步长 -->
	<div class="flex items-center gap-2">
		<label for="renumber-step" class="w-14 text-xs opacity-60 shrink-0">步长</label>
		<input
			id="renumber-step"
			type="number"
			min="1"
			class="w-20 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 adr-mono"
			value={config.step}
			oninput={(e) => update({ step: Math.max(1, parseInt((e.target as HTMLInputElement).value) || 1) })}
		/>
	</div>

	<!-- 填充位数 -->
	<div class="flex items-center gap-2">
		<label for="renumber-padding" class="w-14 text-xs opacity-60 shrink-0">位数</label>
		<input
			id="renumber-padding"
			type="number"
			min="1"
			max="10"
			class="w-20 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 adr-mono"
			value={config.padding}
			oninput={(e) => update({ padding: Math.max(1, Math.min(10, parseInt((e.target as HTMLInputElement).value) || 1)) })}
		/>
		<span class="opacity-50">位补零</span>
	</div>

	<!-- 位置 -->
	<div class="flex items-center gap-2">
		<span class="w-14 text-xs opacity-60 shrink-0">位置</span>
		<div class="flex gap-1">
			{#each POSITION_OPTIONS as opt}
				<button
					class="px-2 py-0.5 rounded text-[11px] transition-colors
						{config.position === opt.value ? 'bg-blue-500/30 text-blue-300' : 'bg-surface-500/10 opacity-55 hover:opacity-75'}"
					onclick={() => update({ position: opt.value })}
					title={opt.example}
				>{opt.label}</button>
			{/each}
		</div>
	</div>

	<!-- 分隔符 -->
	<div class="flex items-center gap-2">
		<label for="renumber-separator" class="w-14 text-xs opacity-60 shrink-0">分隔</label>
		<input
			id="renumber-separator"
			type="text"
			maxlength="3"
			class="w-16 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs text-center
				focus:outline-none focus:border-blue-500/50 adr-mono"
			placeholder="_"
			value={config.separator}
			oninput={(e) => update({ separator: (e.target as HTMLInputElement).value })}
		/>
	</div>

	<!-- 预览 -->
	<div class="px-2 py-1.5 rounded bg-surface-500/5 border border-surface-500/10">
		<p class="opacity-55 text-[11px]">
			预览: <span class="text-blue-300/80 adr-mono">{previewNumber}</span>
		</p>
	</div>
</div>
