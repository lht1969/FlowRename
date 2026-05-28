<!-- FlowRename - NewNameMethodEditor 新名称方法配置编辑器 -->
<script lang="ts">
	import type { NewNameConfig } from '$lib/types';
	import TagPanel from '$lib/components/TagPanel.svelte';

	let { config, onChange } = $props<{
		config: NewNameConfig;
		onChange: (updated: NewNameConfig) => void;
	}>();

	function update(partial: Partial<NewNameConfig>) {
		onChange({ ...config, ...partial });
	}

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
			<p class="opacity-55 mt-1 text-[11px]">使用标签模板生成新文件名，点击下方标签插入</p>
		</div>
	</div>

	<!-- 分类标签面板：确定高度 h-56 确保内部 flexbox 滚动生效 -->
	<div class="border border-surface-500/10 rounded overflow-hidden h-56">
		<TagPanel onInsertTag={insertTag} />
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