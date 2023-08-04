<script lang="ts">
	import { fade } from 'svelte/transition';
	import { writeText } from '@tauri-apps/api/clipboard';
	import type { Item } from '$lib/types/types';
	import { clickOutside } from '$lib/utils/clickOutside';

	export let content: Item;
	let showDrop: boolean = false;

	const copyIdentifier = async () => {
		await writeText(content.identifier);
	};

	const copyPassword = async () => {
		await writeText(content.password);
	};

	const toggleDropDown = () => {
		showDrop = !showDrop;
	};

	const closeDropDown = () => {
		showDrop = false;
	};
</script>

<button
	use:clickOutside={closeDropDown}
	on:click={toggleDropDown}
	class="relative active:scale-90 duration-200 m-2"
>
	<slot />
	{#if showDrop}
		<div
			in:fade={{ duration: 200 }}
			class="flex flex-col absolute top-6 left-1/2 transform -translate-x-1/2 bg-primary-900 border border-primary-700 text-xs lg:text-sm z-10"
		>
			<button class="hover:bg-primary-800 p-[7px]">Open</button>
			<button class="hover:bg-primary-800 p-[7px]">Edit</button>
			<button on:click={copyIdentifier} class="hover:bg-primary-800 p-[7px]"
				>Copy Username/Email</button
			>
			<button on:click={copyPassword} class="hover:bg-primary-800 p-[7px]">Copy Password</button>
			<button class="hover:bg-primary-800 p-[7px]">Delete</button>
		</div>
	{/if}
</button>
