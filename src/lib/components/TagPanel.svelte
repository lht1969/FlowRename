<!-- FlowRename - TagPanel 标签面板组件
  可复用的标签面板，支持 Tab 页签切换四类标签：基本/图片/视频/音频
  视频/音频标签仅当文件列表中存在对应文件类型时显示 -->
<script lang="ts">
	import { hasImageFiles, hasVideoFiles, hasAudioFiles } from '$lib/stores/fileCategories';

	/** 标签类别定义 */
	const TAG_CATEGORIES = [
		{
			id: 'basic',
			label: '基本',
			tags: [
				{ tag: '<Name>', label: '文件名', desc: '原始文件名（不含扩展名）' },
				{ tag: '<Ext>', label: '扩展名', desc: '带点的文件扩展名' },
				{ tag: '<Index>', label: '序号', desc: '1-based 文件序号' },
				{ tag: '<Index:0>', label: '序号0', desc: '0-based 文件序号' },
			],
		},
		{
			id: 'sequence',
			label: '序号',
			tags: [
				{ tag: '<Inc:3>', label: '递增3位', desc: '三位补齐递增序号' },
				{ tag: '<Inc:3:100>', label: '从100起', desc: '从100开始的递增序号' },
				{ tag: '<Inc:5>', label: '递增5位', desc: '五位补齐递增序号' },
				{ tag: '<Cnt>', label: '总数', desc: '文件总数' },
				{ tag: '<Cnt:3>', label: '总数3位', desc: '三位补齐的文件总数' },
			],
		},
		{
			id: 'timestamp',
			label: '日期',
			tags: [
				{ tag: '<Date:YYYYMMDD>', label: '日期', desc: 'YYYYMMDD 格式当前日期' },
				{ tag: '<Date:YYYY>', label: '年份', desc: '四位年份' },
				{ tag: '<Date:YYYYMM>', label: '年月', desc: 'YYYYMM 年月' },
				{ tag: '<Time:HHmmss>', label: '时间', desc: 'HHmmss 格式当前时间' },
				{ tag: '<Time:HHmm>', label: '时分', desc: 'HHmm 时分' },
				{ tag: '<DateTime:YYYYMMDD_HHmmss>', label: '日期时间', desc: '完整日期时间' },
			],
		},
		{
			id: 'image',
			label: '图片',
			alwaysShow: true,
			tags: [
				{ tag: '<ImgWidth>', label: '宽度', desc: '图片像素宽度' },
				{ tag: '<ImgHeight>', label: '高度', desc: '图片像素高度' },
				{ tag: '<ImgMake>', label: '相机制造商', desc: '相机品牌' },
				{ tag: '<ImgModel>', label: '相机型号', desc: '相机型号' },
				{ tag: '<ImgDate:YYYYMMDD>', label: '拍摄日期', desc: 'EXIF 原始拍摄日期' },
				{ tag: '<ImgTime:HHmmss>', label: '拍摄时间', desc: 'EXIF 原始拍摄时间' },
				{ tag: '<ImgISO>', label: 'ISO', desc: 'ISO 感光度' },
				{ tag: '<ImgAperture>', label: '光圈', desc: '光圈 f 值' },
				{ tag: '<ImgFocal>', label: '焦距', desc: '镜头焦段(mm)' },
				{ tag: '<ImgExposure>', label: '曝光', desc: '曝光时间(秒)' },
			],
		},
		{
			id: 'video',
			label: '视频',
			showStore: hasVideoFiles,
			tags: [
				{ tag: '<VidWidth>', label: '视频宽度', desc: '视频像素宽度' },
				{ tag: '<VidHeight>', label: '视频高度', desc: '视频像素高度' },
				{ tag: '<VidFrameRate>', label: '帧率', desc: '视频帧率(fps)' },
				{ tag: '<VidDuration>', label: '时长', desc: 'HH-MM-SS 格式时长' },
				{ tag: '<VidDurationSec>', label: '时长秒', desc: '时长总秒数' },
				{ tag: '<VidTitle>', label: '标题', desc: '视频容器标题' },
				{ tag: '<VidGenre>', label: '流派', desc: '视频流派' },
				{ tag: '<VidDate:YYYYMMDD>', label: '创建日期', desc: '视频创建日期' },
				{ tag: '<VidTime:HHmmss>', label: '创建时间', desc: '视频创建时间' },
				{ tag: '<VidCodec>', label: '编码', desc: '视频编码格式' },
				{ tag: '<VidBitRate>', label: '比特率', desc: '视频比特率(kbps)' },
			],
		},
		{
			id: 'audio',
			label: '音频',
			showStore: hasAudioFiles,
			tags: [
				{ tag: '<AudTitle>', label: '标题', desc: '音频曲目标题' },
				{ tag: '<AudArtist>', label: '艺术家', desc: '艺术家/演唱者' },
				{ tag: '<AudAlbum>', label: '专辑', desc: '所属专辑名' },
				{ tag: '<AudYear>', label: '年份', desc: '发行年份' },
				{ tag: '<AudTrack:2>', label: '音轨号', desc: '两位补齐的曲目序号' },
				{ tag: '<AudGenre>', label: '流派', desc: '音乐流派' },
				{ tag: '<AudDuration>', label: '时长', desc: 'MM-SS 格式时长' },
				{ tag: '<AudDurationSec>', label: '时长秒', desc: '时长总秒数' },
				{ tag: '<AudBitRate>', label: '比特率', desc: '音频比特率(kbps)' },
				{ tag: '<AudDisc:2>', label: '碟片号', desc: '碟片序号' },
				{ tag: '<AudDate:YYYYMMDD>', label: '录制日期', desc: 'ID3 录制日期' },
				{ tag: '<AudTime:HHmmss>', label: '录制时间', desc: 'ID3 录制时间' },
			],
		},
	];

	/** 当前选中的 Tab */
	let activeTab = $state('basic');

	/** 插入标签回调 */
	let { onInsertTag } = $props<{
		onInsertTag: (tag: string) => void;
	}>();

	/** 可见的标签类别（根据文件类型动态过滤） */
	const visibleCategories = $derived(
		TAG_CATEGORIES.filter((cat) => {
			if (cat.alwaysShow) return true;
			if (cat.showStore) {
				let visible = false;
				const unsub = cat.showStore.subscribe((v) => (visible = v));
				unsub();
				return visible;
			}
			return true;
		})
	);

	/** 当前 Tab 的标签列表 */
	let activeTags = $derived(
		visibleCategories.find((c) => c.id === activeTab)?.tags ?? []
	);

	function insertTag(tag: string) {
		onInsertTag(tag);
	}
</script>

<div class="tag-panel flex flex-col h-full">
	<!-- Tab 页签导航 -->
	<div class="flex flex-wrap gap-0.5 border-b border-surface-500/10 pb-1">
		{#each visibleCategories as cat}
			<button
				class="px-2.5 py-1 rounded-t text-[11px] transition-colors
					{activeTab === cat.id
						? 'bg-blue-500/15 text-blue-600 dark:text-blue-300 border-b border-blue-500/50 font-medium'
						: 'bg-surface-500/5 opacity-55 hover:opacity-80 hover:bg-surface-500/10'}"
				onclick={() => (activeTab = cat.id)}
			>
				{cat.label}
			</button>
		{/each}
	</div>

	<!-- 标签内容区 -->
	<div class="flex-1 overflow-y-auto p-1 space-y-0.5">
		{#each activeTags as tag}
			<button
				class="w-full text-left px-2 py-1.5 rounded text-[11px]
					bg-surface-500/5 hover:bg-surface-500/15 hover:pl-3 transition-all duration-[var(--adr-transition-fast)]
					group flex items-center justify-between"
				onclick={() => insertTag(tag.tag)}
				title={tag.desc}
			>
				<code class="adr-mono text-[11px] font-medium"
					style="color: {activeTab === 'basic' ? 'var(--color-primary-500)' : 'inherit'}">
					{tag.tag}
				</code>
				<span class="opacity-50 group-hover:opacity-70 transition-opacity text-[11px] truncate ml-2">
					{tag.label}
				</span>
			</button>
		{/each}
	</div>
</div>