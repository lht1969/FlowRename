<script lang="ts">
	import type { TimestampConfig } from '$lib/types';

	let { config, onChange } = $props<{
		config: TimestampConfig;
		onChange: (updated: TimestampConfig) => void;
	}>();

	function update(partial: Partial<TimestampConfig>) {
		onChange({ ...config, ...partial });
	}

	type SourceOption = { value: TimestampConfig['source']; label: string; group: string };

	/** 时间戳来源选项 - 按媒体类型分组 */
	const SOURCE_OPTIONS: SourceOption[] = [
		{ value: 'Created', label: '创建', group: '文件' },
		{ value: 'Modified', label: '修改', group: '文件' },
		{ value: 'Accessed', label: '访问', group: '文件' },
		{ value: 'ImgDate', label: '拍摄日期', group: '图片' },
		{ value: 'ImgTime', label: '拍摄时间', group: '图片' },
		{ value: 'VidDate', label: '创建日期', group: '视频' },
		{ value: 'VidTime', label: '创建时间', group: '视频' },
		{ value: 'AudDate', label: '录制日期', group: '音频' },
		{ value: 'AudTime', label: '录制时间', group: '音频' },
	];

	/** 按组分类来源选项 */
	const SOURCE_GROUPS = [
		{ id: '文件', label: '文件', options: SOURCE_OPTIONS.filter(o => o.group === '文件') },
		{ id: '图片', label: '图片', options: SOURCE_OPTIONS.filter(o => o.group === '图片') },
		{ id: '视频', label: '视频', options: SOURCE_OPTIONS.filter(o => o.group === '视频') },
		{ id: '音频', label: '音频', options: SOURCE_OPTIONS.filter(o => o.group === '音频') },
	];

	/** 常用格式预设 */
	const FORMAT_PRESETS = [
		{ format: 'YYYY-MM-DD', label: '日期', example: '2024-01-15' },
		{ format: 'YYYY-MM-DD_HH-mm-ss', label: '日期时间', example: '2024-01-15_10-30-00' },
		{ format: 'YYYYMMDD', label: '紧凑日期', example: '20240115' },
		{ format: 'YYYYMMDD_HHmmss', label: '紧凑日期时间', example: '20240115_103000' },
		{ format: 'HH-mm-ss', label: '仅时间', example: '10-30-00' },
	];
</script>

<div class="space-y-2 text-xs">
	<!-- 时间戳来源 -->
	<div class="flex items-center gap-2">
		<span class="w-14 text-xs opacity-60 shrink-0">来源</span>
		<div class="flex flex-col gap-1">
			{#each SOURCE_GROUPS as group}
				<div class="flex items-center gap-1">
					<span class="w-6 text-[10px] opacity-40 shrink-0">{group.label}</span>
					<div class="flex gap-1">
						{#each group.options as opt}
							<button
								class="px-1.5 py-0.5 rounded text-[10px] transition-colors
									{config.source === opt.value ? 'bg-blue-500/30 text-blue-300' : 'bg-surface-500/10 opacity-55 hover:opacity-75'}"
								onclick={() => update({ source: opt.value })}
							>{opt.label}</button>
						{/each}
					</div>
				</div>
			{/each}
		</div>
	</div>

	<!-- 格式字符串 -->
	<div class="flex items-start gap-2">
		<label for="timestamp-format" class="w-12 opacity-50 shrink-0 mt-1">格式</label>
		<div class="flex-1">
			<input
				id="timestamp-format"
				type="text"
				class="w-full bg-surface-500/10 border border-surface-500/20 rounded px-2 py-1 text-xs
					adr-mono focus:outline-none focus:border-blue-500/50 transition-colors"
				placeholder="YYYY-MM-DD_HH-mm-ss"
				value={config.format}
				oninput={(e) => update({ format: (e.target as HTMLInputElement).value })}
			/>
		</div>
	</div>

	<!-- 格式预设快捷按钮 -->
	<div class="flex flex-wrap gap-1">
		{#each FORMAT_PRESETS as preset}
			<button
				class="px-1.5 py-0.5 rounded text-[11px] bg-surface-500/10 hover:bg-surface-500/20
					transition-colors opacity-55 hover:opacity-80 adr-mono
					{config.format === preset.format ? 'ring-1 ring-blue-500/30 opacity-80' : ''}"
				onclick={() => update({ format: preset.format })}
				title={preset.example}
			>
				{preset.label}
			</button>
		{/each}
	</div>

	<!-- 格式说明 -->
	<div class="px-2 py-1.5 rounded bg-surface-500/5 border border-surface-500/10">
		<p class="opacity-50 text-[11px] leading-relaxed">
				YYYY=年 MM=月 DD=日 HH=时 mm=分 ss=秒
		</p>
	</div>
</div>
