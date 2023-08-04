<script lang="ts">
	import { fade } from 'svelte/transition';
	import { writeText } from '@tauri-apps/api/clipboard';
	import { clickOutside } from '$lib/utils/clickOutside';
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api';
	import ClientStore from '$lib/stores/ClientStore';

	export let content: Item;
	let showDrop: boolean = false;

	const editItem = async () => {
		goto(`edit/${content.id}`);
	};

	const copyIdentifier = async () => {
		await writeText(content.identifier);
	};

	const copyPassword = async () => {
		await writeText(content.password);
	};

	const toggleDropDown = () => {
		showDrop = !showDrop;
	};

	const deleteItem = async () => {
		let user: Client = JSON.parse(localStorage.getItem('client') || '');
		await invoke('remove_item', {
			username: user.username,
			itemId: content.id
		});

		let items: string = await invoke('fetch_items', { userId: user.id });
		let data: TauriResponse = JSON.parse(items);
		$ClientStore.items = data.data;
		await localStorage.setItem('client', JSON.stringify($ClientStore));
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
			<button on:click={editItem} class="hover:bg-primary-800 p-[7px]">Edit</button>
			<button on:click={copyIdentifier} class="hover:bg-primary-800 p-[7px]"
				>Copy Username/Email</button
			>
			<button on:click={copyPassword} class="hover:bg-primary-800 p-[7px]">Copy Password</button>
			<button on:click={deleteItem} class="hover:bg-primary-800 p-[7px]">Delete</button>
		</div>
	{/if}
</button>
