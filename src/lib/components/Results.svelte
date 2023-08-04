<script lang="ts">
	import TableItem from '$lib/components/TableItem.svelte';
	import ClientStore from '$lib/stores/ClientStore';
	import { localToStore } from '$lib/utils';
	import { onMount } from 'svelte';
	export let resultQuery: string | undefined = undefined;

	onMount(async () => {
		await localToStore(ClientStore, 'client', {});
	});
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
			<div class="h-[90%] min-h-[200px] mt-2 overflow-y-scroll">
				{#if $ClientStore}
					{#each $ClientStore.items as item}
						{#if resultQuery && item.title.includes(resultQuery)}
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
		background-color: #3c3d5d;
		border-radius: 20px; /* roundness of the scroll thumb */
		border: 3px solid #1e1f2f; /* creates padding around scroll thumb */
	}
</style>
