<script lang="ts">
	import TableItem from '$lib/components/TableItem.svelte';
	import type { Item } from '$lib/types/types';
	import { fade } from 'svelte/transition';

	const sampleWithlink: Item = {
		title: 'google',
		content: 'password123',
		link: 'https://www.google.com/',
		lastUsed: '02/01/2023',
		lastModified: '01/01/2023'
	};

	const sampleWithoutlink: Item = {
		title: 'notes',
		content: 'password123',
		lastUsed: '02/01/2023',
		lastModified: '01/01/2023'
	};

	let showItemMenu: boolean;
	const toggleItemOptions = () => {
		showItemMenu = !showItemMenu;
	};

	const data = [
		sampleWithlink,
		sampleWithlink,
		sampleWithlink,
		sampleWithoutlink,
		sampleWithoutlink
	];
</script>

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
				<div class="w-[20%] p-2">Last Used</div>
				<div class="w-[15%] p-2" />
			</div>
			<div class="h-[90%] min-h-[200px] mt-2 overflow-y-scroll">
				{#each data as item}
					<TableItem data={item} />
				{/each}
			</div>
		</div>
	</div>
</div>

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
