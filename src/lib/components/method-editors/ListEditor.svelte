<script lang="ts">
	import type { ListConfig } from '$lib/types';

	let { config, onChange } = $props<{
		config: ListConfig;
		onChange: (updated: ListConfig) => void;
	}>();

	function update(partial: Partial<ListConfig>) {
		onChange({ ...config, ...partial });
	}

	/** 名称列表文本（每行一个名称） */
	let localNamesText = $state('');

	$effect(() => {
		localNamesText = config.names.join('\n');
	});

	function handleNamesInput(e: Event) {
		const value = (e.target as HTMLTextAreaElement).value;
		localNamesText = value;
		const names = value.split('\n').filter((n) => n.trim() !== '');
		update({ names });
	}
</script>

<div class="space-y-2 text-xs">
	<!-- 名称列表 -->
	<div class="flex items-start gap-2">
		<label for="list-names" class="w-12 opacity-50 shrink-0 mt-1">列表</label>
		<div class="flex-1">
			<textarea
				id="list-names"
				class="w-full bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
					focus:outline-none focus:border-blue-500/50 transition-colors resize-y adr-mono"
				rows="5"
				placeholder="每行一个名称&#10;photo_001&#10;photo_002&#10;photo_003"
				value={localNamesText}
				oninput={handleNamesInput}
			></textarea>
			<p class="opacity-30 mt-1 text-[10px]">每行一个名称，按文件顺序对应</p>
		</div>
	</div>

	<!-- 溢出行为 -->
	<div class="flex items-center gap-2">
		<span class="w-12 opacity-50 shrink-0">溢出</span>
		<div class="flex gap-1">
			<button
				class="px-2 py-0.5 rounded text-[10px] transition-colors
					{config.overflowBehavior === 'KeepOriginal' ? 'bg-blue-500/30 text-blue-300' : 'bg-surface-500/10 opacity-50 hover:opacity-70'}"
				onclick={() => update({ overflowBehavior: 'KeepOriginal' })}
			>保留原名</button>
			<button
				class="px-2 py-0.5 rounded text-[10px] transition-colors
					{config.overflowBehavior === 'Skip' ? 'bg-blue-500/30 text-blue-300' : 'bg-surface-500/10 opacity-50 hover:opacity-70'}"
				onclick={() => update({ overflowBehavior: 'Skip' })}
			>跳过</button>
			<button
				class="px-2 py-0.5 rounded text-[10px] transition-colors
					{config.overflowBehavior === 'Cycle' ? 'bg-blue-500/30 text-blue-300' : 'bg-surface-500/10 opacity-50 hover:opacity-70'}"
				onclick={() => update({ overflowBehavior: 'Cycle' })}
			>循环</button>
		</div>
	</div>
</div>
