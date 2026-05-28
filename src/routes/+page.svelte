<!-- FlowRename - Main Page (Three-Column Layout) -->
<script lang="ts">
	import TitleBar from '$lib/components/TitleBar.svelte';
	import StatusBar from '$lib/components/StatusBar.svelte';
	import MethodPanel from '$lib/components/MethodPanel.svelte';
	import FileList from '$lib/components/FileList.svelte';
	import ControlPanel from '$lib/components/ControlPanel.svelte';
</script>

<div class="adr-app flex flex-col h-screen w-screen overflow-hidden">
	<!-- 顶部标题栏 -->
	<TitleBar />

	<!-- 三栏主内容区 -->
	<div class="adr-main flex flex-1 overflow-hidden">
		<!-- 左栏：方法面板 -->
		<MethodPanel />

		<!-- 中栏：文件列表（弹性宽度） -->
		<div class="flex-1 overflow-hidden">
			<FileList />
		</div>

		<!-- 右栏：控制面板 -->
		<ControlPanel />
	</div>

	<!-- 底部状态栏 -->
	<StatusBar />
</div>

<style>
	.adr-app {
		background: var(--color-surface-200);
	}

	.adr-main {
		border-top: 1px solid var(--color-surface-500/20);
		border-bottom: 1px solid var(--color-surface-500/20);
	}
</style>
