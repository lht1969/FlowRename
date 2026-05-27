<script lang="ts">
	import type { MoveConfig } from '$lib/types';

	let { config, onChange } = $props<{
		config: MoveConfig;
		onChange: (updated: MoveConfig) => void;
	}>();

	function update(partial: Partial<MoveConfig>) {
		onChange({ ...config, ...partial });
	}
</script>

<div class="space-y-2 text-xs">
	<!-- 起始位置 -->
	<div class="flex items-center gap-2">
		<label for="move-from" class="w-14 text-xs opacity-60 shrink-0">起始</label>
		<input
			id="move-from"
			type="number"
			min="0"
			class="w-20 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50 adr-mono"
			value={config.fromStart}
			oninput={(e) => update({ fromStart: Math.max(0, parseInt((e.target as HTMLInputElement).value) || 0) })}
		/>
		<span class="opacity-50">位置</span>
	</div>

	<!-- 字符数量 -->
	<div class="flex items-center gap-2">
		<label for="move-count" class="w-14 text-xs opacity-60 shrink-0">数量</label>
		<input
			id="move-count"
			type="number"
			min="1"
			class="w-20 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50 adr-mono"
			value={config.count}
			oninput={(e) => update({ count: Math.max(1, parseInt((e.target as HTMLInputElement).value) || 1) })}
		/>
		<span class="opacity-50">个字符</span>
	</div>

	<!-- 目标位置 -->
	<div class="flex items-center gap-2">
		<label for="move-to" class="w-14 text-xs opacity-60 shrink-0">移至</label>
		<input
			id="move-to"
			type="number"
			min="0"
			class="w-20 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
				focus:outline-none focus:border-blue-500/50 adr-mono"
			value={config.toPosition}
			oninput={(e) => update({ toPosition: Math.max(0, parseInt((e.target as HTMLInputElement).value) || 0) })}
		/>
		<span class="opacity-50">位置</span>
	</div>

	<!-- 可视化说明 -->
	<div class="px-2 py-1.5 rounded bg-surface-500/5 border border-surface-500/10">
		<p class="opacity-55 text-[11px]">
			将第 <span class="text-blue-300/80 adr-mono">{config.fromStart}</span>~<span class="text-blue-300/80 adr-mono">{config.fromStart + config.count - 1}</span> 位的
			<span class="text-blue-300/80 adr-mono">{config.count}</span> 个字符移至第 <span class="text-blue-300/80 adr-mono">{config.toPosition}</span> 位
		</p>
	</div>
</div>
