<script lang="ts">
	import TableItem from '$lib/components/TableItem.svelte';
	import ClientStore from '$lib/stores/ClientStore';
	import type { TauriResponse } from '$lib/types/types';
	import { localToStore } from '$lib/utils';
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';

	onMount(async () => {
		await localToStore(ClientStore, 'client', {});
		let items: string = await invoke('fetch_items', { userId: $ClientStore.id });
		let data: TauriResponse = JSON.parse(items);
		$ClientStore.items = data.data;
		await localStorage.setItem('client', JSON.stringify($ClientStore));
	});
</script>

<div class="flex justify-center h-full">
	<div class="w-[90%]">
		<div class="flex mt-20 h-[5%] items-center">
			<h1 class="flex text-3xl font-bold">Notes</h1>
			<a
				href="/new_note"
				class="hover:bg-primary-700 border duration-200 w-[10%] min-w-[100px] ml-auto border-primary-600 rounded-md p-2 active:scale-90 flex justify-center items-center bg-primary-600"
				>Add Note</a
			>
		</div>
		<div class="w-full mt-10 drop-shadow-xl h-[70%] rounded-md">
			<div
				class="flex items-center border-b border-primary-800 rounded-t-md text-sm lg:text-lg drop-shadow-lg h-[10%] font-bold text-center justify-evenly"
			>
				<div class="w-[20%] p-2">Title</div>
				<div class="w-[20%] p-2">Last Modified</div>
				<div class="w-[20%] p-2">Created</div>
				<div class="w-[15%] p-2" />
			</div>
			<div class="h-[90%] min-h-[200px] mt-2 overflow-y-scroll">
				{#if $ClientStore}
					{#each $ClientStore.items as item}
						{#if item.type_ == 'note'}
							<TableItem data={item} />
						{/if}
					{/each}
				{/if}
			</div>
		</div>
	</div>
</div>

<style>
	div::-webkit-scrollbar {
		width: 12px; /* width of the entire scrollbar */
	}

	div::-webkit-scrollbar-track {
		background: none; /* color of the tracking area */
	}

	div::-webkit-scrollbar-thumb {
		background-color: #28293e;
		border-radius: 20px;
		border: 3px solid #0f0f18;
	}
</style>
