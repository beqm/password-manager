<script lang="ts">
	import TableItem from '$lib/components/TableItem.svelte';
	import type { Item } from '$lib/types/types';
	export let resultQuery: string | undefined = undefined;
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

	const data = [sampleWithlink];
</script>

<div class="flex justify-center h-full">
	<div class="w-[90%]">
		<div class="flex mt-20 h-[5%] items-center">
			<h1 class="flex text-3xl font-bold">Results</h1>
		</div>
		<div class="w-full mt-10 drop-shadow-xl h-[70%] rounded-md">
			{#if resultQuery}
				<div
					class="flex items-center rounded-t-md border-b border-primary-800 text-sm lg:text-lg drop-shadow-lg h-[10%] font-bold text-center justify-evenly"
				>
					<div class="w-[20%] p-2">Title</div>
					<div class="w-[20%] p-2">Last Modified</div>
					<div class="w-[20%] p-2">Created</div>
					<div class="w-[15%] p-2" />
				</div>
			{/if}
			<div class="h-[90%] min-h-[200px] overflow-y-scroll">
				{#each data as item}
					{#if resultQuery && item.title.includes(resultQuery)}
						<TableItem data={item} />
					{:else}
						<h1 class="flex text-3xl font-bold w-full justify-center mt-10">No Results</h1>
					{/if}
				{/each}
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
		background-color: #3c3d5d;
		border-radius: 20px; /* roundness of the scroll thumb */
		border: 3px solid #1e1f2f; /* creates padding around scroll thumb */
	}
</style>
