<script lang="ts">
	import type { TrimConfig } from '$lib/types';

	let { config, onChange } = $props<{
		config: TrimConfig;
		onChange: (updated: TrimConfig) => void;
	}>();

	function update(partial: Partial<TrimConfig>) {
		onChange({ ...config, ...partial });
	}
</script>

<div class="space-y-2 text-xs">
	<!-- 修剪空白字符开关 -->
	<div class="flex items-center gap-2">
		<span class="w-12 opacity-50 shrink-0">空白</span>
		<button
			class="relative w-8 h-4 rounded-full transition-colors
				{config.trimWhitespace ? 'bg-blue-500/60' : 'bg-surface-500/20'}"
			onclick={() => update({ trimWhitespace: !config.trimWhitespace })}
			aria-label="修剪首尾空白"
			title="修剪首尾空白"
		>
			<span
				class="absolute top-0.5 w-3 h-3 rounded-full bg-white transition-transform
					{config.trimWhitespace ? 'left-4.5' : 'left-0.5'}"
			></span>
		</button>
		<span class="opacity-40">修剪首尾空白</span>
	</div>

	{#if !config.trimWhitespace}
		<!-- 修剪开头字符 -->
		<div class="flex items-center gap-2">
			<label for="trim-start" class="w-12 opacity-50 shrink-0">开头</label>
			<input
				id="trim-start"
				type="text"
				class="flex-1 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
					focus:outline-none focus:border-blue-500/50 adr-mono"
				placeholder="要修剪的字符（如: 0 _-）"
				value={config.trimStart}
				oninput={(e) => update({ trimStart: (e.target as HTMLInputElement).value })}
			/>
		</div>

		<!-- 修剪末尾字符 -->
		<div class="flex items-center gap-2">
			<label for="trim-end" class="w-12 opacity-50 shrink-0">末尾</label>
			<input
				id="trim-end"
				type="text"
				class="flex-1 bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
					focus:outline-none focus:border-blue-500/50 adr-mono"
				placeholder="要修剪的字符（如: 0 _-）"
				value={config.trimEnd}
				oninput={(e) => update({ trimEnd: (e.target as HTMLInputElement).value })}
			/>
		</div>

		<p class="opacity-30 text-[10px] px-2">输入的每个字符都会被从对应方向修剪</p>
	{/if}
</div>
