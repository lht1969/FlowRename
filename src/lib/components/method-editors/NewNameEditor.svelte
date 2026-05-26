<!-- AdRename - NewNameMethodEditor 新名称方法配置编辑器 -->
<script lang="ts">
	import type { NewNameConfig } from '$lib/types';

	let { config, onChange } = $props<{
		config: NewNameConfig;
		onChange: (updated: NewNameConfig) => void;
	}>();

	function update(partial: Partial<NewNameConfig>) {
		onChange({ ...config, ...partial });
	}

	/** 常用标签快捷插入 */
	const QUICK_TAGS = [
		{ tag: '<Name>', label: '文件名' },
		{ tag: '<Ext>', label: '扩展名' },
		{ tag: '<Index>', label: '序号' },
		{ tag: '<Inc:3>', label: '递增' },
		{ tag: '<Date:YYYYMMDD>', label: '日期' },
		{ tag: '<Time:HHmmss>', label: '时间' },
	];

	function insertTag(tag: string) {
		update({ template: config.template + tag });
	}
</script>

<div class="space-y-2 text-xs">
	<!-- 模板输入 -->
	<div class="flex items-start gap-2">
		<label for="newname-template" class="w-12 opacity-50 shrink-0 mt-1">模板</label>
		<div class="flex-1">
			<input
				id="newname-template"
				type="text"
				class="w-full bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
					adr-mono focus:outline-none focus:border-blue-500/50 transition-colors"
				placeholder="<Name>_<Inc:3>"
				value={config.template}
				oninput={(e) => update({ template: (e.target as HTMLInputElement).value })}
			/>
			<p class="opacity-30 mt-1 text-[10px]">使用标签模板生成新文件名</p>
		</div>
	</div>

	<!-- 快捷标签插入 -->
	<div class="flex flex-wrap gap-1">
		{#each QUICK_TAGS as qt}
			<button
				class="px-1.5 py-0.5 rounded text-[10px] bg-surface-500/10 hover:bg-surface-500/20
					transition-colors opacity-50 hover:opacity-80 adr-mono"
				onclick={() => insertTag(qt.tag)}
				title="插入 {qt.tag}"
			>
				{qt.label}
			</button>
		{/each}
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
