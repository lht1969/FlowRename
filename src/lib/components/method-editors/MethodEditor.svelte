<script lang="ts">
	import type {
		MethodConfig, ReplaceConfig, AddConfig, RemoveConfig,
		NewCaseConfig, NewNameConfig, ListConfig, MoveConfig,
		TrimConfig, RenumberConfig, TimestampConfig
	} from '$lib/types';
	import ReplaceEditor from './ReplaceEditor.svelte';
	import AddEditor from './AddEditor.svelte';
	import RemoveEditor from './RemoveEditor.svelte';
	import NewCaseEditor from './NewCaseEditor.svelte';
	import NewNameEditor from './NewNameEditor.svelte';
	import ListEditor from './ListEditor.svelte';
	import MoveEditor from './MoveEditor.svelte';
	import TrimEditor from './TrimEditor.svelte';
	import RenumberEditor from './RenumberEditor.svelte';
	import TimestampEditor from './TimestampEditor.svelte';

	let { config, index, onUpdate } = $props<{
		config: MethodConfig;
		index: number;
		onUpdate: (index: number, updated: MethodConfig) => void;
	}>();

	function handleReplaceChange(updated: ReplaceConfig) {
		onUpdate(index, { Replace: updated });
	}

	function handleAddChange(updated: AddConfig) {
		onUpdate(index, { Add: updated });
	}

	function handleRemoveChange(updated: RemoveConfig) {
		onUpdate(index, { Remove: updated });
	}

	function handleNewCaseChange(updated: NewCaseConfig) {
		onUpdate(index, { NewCase: updated });
	}

	function handleNewNameChange(updated: NewNameConfig) {
		onUpdate(index, { NewName: updated });
	}

	function handleListChange(updated: ListConfig) {
		onUpdate(index, { List: updated });
	}

	function handleMoveChange(updated: MoveConfig) {
		onUpdate(index, { Move: updated });
	}

	function handleTrimChange(updated: TrimConfig) {
		onUpdate(index, { Trim: updated });
	}

	function handleRenumberChange(updated: RenumberConfig) {
		onUpdate(index, { Renumber: updated });
	}

	function handleTimestampChange(updated: TimestampConfig) {
		onUpdate(index, { Timestamp: updated });
	}
</script>

<div class="adr-method-editor">
	{#if 'Replace' in config}
		<ReplaceEditor config={config.Replace} onChange={handleReplaceChange} />
	{:else if 'Add' in config}
		<AddEditor config={config.Add} onChange={handleAddChange} />
	{:else if 'Remove' in config}
		<RemoveEditor config={config.Remove} onChange={handleRemoveChange} />
	{:else if 'NewCase' in config}
		<NewCaseEditor config={config.NewCase} onChange={handleNewCaseChange} />
	{:else if 'NewName' in config}
		<NewNameEditor config={config.NewName} onChange={handleNewNameChange} />
	{:else if 'List' in config}
		<ListEditor config={config.List} onChange={handleListChange} />
	{:else if 'Move' in config}
		<MoveEditor config={config.Move} onChange={handleMoveChange} />
	{:else if 'Trim' in config}
		<TrimEditor config={config.Trim} onChange={handleTrimChange} />
	{:else if 'Renumber' in config}
		<RenumberEditor config={config.Renumber} onChange={handleRenumberChange} />
	{:else if 'Timestamp' in config}
		<TimestampEditor config={config.Timestamp} onChange={handleTimestampChange} />
	{/if}
</div>
