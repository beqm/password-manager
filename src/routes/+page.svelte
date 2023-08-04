<script lang="ts">
	import TableItem from '$lib/components/TableItem.svelte';
	import ClientStore from '$lib/stores/ClientStore';
	import { localToStore } from '$lib/utils';
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';

	let showItemMenu: boolean;
	const toggleItemOptions = () => {
		showItemMenu = !showItemMenu;
	};

	onMount(async () => {
		await localToStore(ClientStore, 'client', {});
		let items: string = await invoke('fetch_items', { userId: $ClientStore.id });
		let data: TauriResponse = JSON.parse(items);
		$ClientStore.items = data.data;
		await localStorage.setItem('client', JSON.stringify($ClientStore));
	});
</script>

{#if $ClientStore}
	<div class="flex justify-center h-full">
		<div class="w-[90%]">
			<div class="flex mt-20 h-[5%] items-center">
				<h1 class="flex text-3xl font-bold">All items</h1>
				<div class="ml-auto relative">
					<button
						on:click={toggleItemOptions}
						class="hover:bg-primary-700 border duration-200 w-[10%] min-w-[100px] border-primary-600 rounded-md p-2 active:scale-90 flex justify-center items-center bg-primary-600"
						>Add Item</button
					>
					{#if showItemMenu}
						<div
							transition:fade={{ duration: 100 }}
							class="flex flex-col absolute bg-primary-900 border border-primary-700 mt-1 right-0 w-[200px] z-10"
						>
							<a href="new_password" class="hover:bg-primary-700 p-2">New Password</a>
							<a href="new_note" class="hover:bg-primary-700 p-2">New Note</a>
						</div>
					{/if}
				</div>
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
							<TableItem data={item} />
						{/each}
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	div::-webkit-scrollbar {
		width: 12px;
	}

	div::-webkit-scrollbar-track {
		background: none;
	}

	div::-webkit-scrollbar-thumb {
		background-color: #28293e;
		border-radius: 20px;
		border: 3px solid #0f0f18;
	}
</style>
