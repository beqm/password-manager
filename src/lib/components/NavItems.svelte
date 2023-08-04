<script lang="ts">
	import { slide } from 'svelte/transition';
	import { page } from '$app/stores';
	let showItems: boolean;
	let showTools: boolean;

	let cHome = 'flex p-4 items-center bg-primary-600';
	let cPasswords = 'flex p-4 items-center hover:bg-primary-800';
	let cNotes = 'flex p-4 items-center hover:bg-primary-800';
	let cGenerator = 'flex p-4 items-center hover:bg-primary-800';

	$: {
		if ($page.url.pathname == '/') {
			cHome = 'flex p-4 items-center bg-primary-600';
			cPasswords = 'flex p-4 items-center hover:bg-primary-800';
			cNotes = 'flex p-4 items-center hover:bg-primary-800';
			cGenerator = 'flex p-4 items-center hover:bg-primary-800';
		} else if ($page.url.pathname == '/passwords') {
			cHome = 'flex p-4 items-center hover:bg-primary-800';
			cPasswords = 'flex p-4 items-center bg-primary-600';
			cNotes = 'flex p-4 items-center hover:bg-primary-800';
			cGenerator = 'flex p-4 items-center hover:bg-primary-800';
		} else if ($page.url.pathname == '/notes') {
			cHome = 'flex p-4 items-center hover:bg-primary-800';
			cPasswords = 'flex p-4 items-center hover:bg-primary-800';
			cNotes = 'flex p-4 items-center bg-primary-600';
			cGenerator = 'flex p-4 items-center hover:bg-primary-800';
		} else if ($page.url.pathname == '/generator') {
			cHome = 'flex p-4 items-center hover:bg-primary-800';
			cPasswords = 'flex p-4 items-center hover:bg-primary-800';
			cNotes = 'flex p-4 items-center hover:bg-primary-800';
			cGenerator = 'flex p-4 items-center bg-primary-600';
		} else {
			cHome = 'flex p-4 items-center hover:bg-primary-800';
			cPasswords = 'flex p-4 items-center hover:bg-primary-800';
			cNotes = 'flex p-4 items-center hover:bg-primary-800';
			cGenerator = 'flex p-4 items-center hover:bg-primary-800';
		}
	}

	const toggleItemDropdown = () => {
		showItems = !showItems;
	};

	const toggleToolsDropdown = () => {
		showTools = !showTools;
	};
</script>

<div class="flex flex-col w-full border-b border-primary-800">
	<button on:click={toggleItemDropdown} class="flex w-full p-2">
		<span class="ml-2 flex text-sm xl:text-lg w-[95%]">Items</span>
		{#if showItems}
			<svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 512 512"
				><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
					d="M233.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L256 338.7 86.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"
				/></svg
			>
		{:else}
			<svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 512 512"
				><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
					d="M233.4 105.4c12.5-12.5 32.8-12.5 45.3 0l192 192c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L256 173.3 86.6 342.6c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l192-192z"
				/></svg
			>
		{/if}
	</button>
	{#if showItems}
		<div transition:slide class="w-full">
			<a class={cHome} href="/">
				<svg
					class="w-[10%]"
					fill="currentColor"
					xmlns="http://www.w3.org/2000/svg"
					height="1.2em"
					viewBox="0 0 576 512"
					><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
						d="M64 0C28.7 0 0 28.7 0 64V416c0 35.3 28.7 64 64 64H80l16 32h64l16-32H400l16 32h64l16-32h16c35.3 0 64-28.7 64-64V64c0-35.3-28.7-64-64-64H64zM224 320a80 80 0 1 0 0-160 80 80 0 1 0 0 160zm0-240a160 160 0 1 1 0 320 160 160 0 1 1 0-320zM480 221.3V336c0 8.8-7.2 16-16 16s-16-7.2-16-16V221.3c-18.6-6.6-32-24.4-32-45.3c0-26.5 21.5-48 48-48s48 21.5 48 48c0 20.9-13.4 38.7-32 45.3z"
					/></svg
				>
				<span class="ml-2 flex">All Items</span>
			</a>
			<a class={cPasswords} href="/passwords">
				<svg
					class="w-[10%]"
					fill="currentColor"
					xmlns="http://www.w3.org/2000/svg"
					height="1.2rem"
					viewBox="0 0 512 512"
					><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
						d="M336 352c97.2 0 176-78.8 176-176S433.2 0 336 0S160 78.8 160 176c0 18.7 2.9 36.8 8.3 53.7L7 391c-4.5 4.5-7 10.6-7 17v80c0 13.3 10.7 24 24 24h80c13.3 0 24-10.7 24-24V448h40c13.3 0 24-10.7 24-24V384h40c6.4 0 12.5-2.5 17-7l33.3-33.3c16.9 5.4 35 8.3 53.7 8.3zM376 96a40 40 0 1 1 0 80 40 40 0 1 1 0-80z"
					/></svg
				>
				<span class="ml-2 flex">Passwords</span>
			</a>
			<a class={cNotes} href="/notes">
				<svg
					class="w-[10%]"
					fill="currentColor"
					xmlns="http://www.w3.org/2000/svg"
					height="1.2em"
					viewBox="0 0 384 512"
					><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
						d="M64 0C28.7 0 0 28.7 0 64V448c0 35.3 28.7 64 64 64H320c35.3 0 64-28.7 64-64V160H256c-17.7 0-32-14.3-32-32V0H64zM256 0V128H384L256 0zM112 256H272c8.8 0 16 7.2 16 16s-7.2 16-16 16H112c-8.8 0-16-7.2-16-16s7.2-16 16-16zm0 64H272c8.8 0 16 7.2 16 16s-7.2 16-16 16H112c-8.8 0-16-7.2-16-16s7.2-16 16-16zm0 64H272c8.8 0 16 7.2 16 16s-7.2 16-16 16H112c-8.8 0-16-7.2-16-16s7.2-16 16-16z"
					/></svg
				>
				<span class="ml-2 flex">Notes</span>
			</a>
		</div>
	{/if}
</div>

<div class="flex flex-col w-full border-b border-primary-800">
	<button on:click={toggleToolsDropdown} class="flex w-full p-2">
		<span class="ml-2 flex text-sm xl:text-lg w-[95%]">Tools</span>
		{#if showTools}
			<svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 512 512"
				><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
					d="M233.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L256 338.7 86.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"
				/></svg
			>
		{:else}
			<svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 512 512"
				><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
					d="M233.4 105.4c12.5-12.5 32.8-12.5 45.3 0l192 192c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L256 173.3 86.6 342.6c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l192-192z"
				/></svg
			>
		{/if}
	</button>
	{#if showTools}
		<div transition:slide class="w-full">
			<a class={cGenerator} href="/generator">
				<svg
					class="w-[10%]"
					fill="currentColor"
					xmlns="http://www.w3.org/2000/svg"
					height="1.2em"
					viewBox="0 0 512 512"
					><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
						d="M386.3 160H336c-17.7 0-32 14.3-32 32s14.3 32 32 32H464c17.7 0 32-14.3 32-32V64c0-17.7-14.3-32-32-32s-32 14.3-32 32v51.2L414.4 97.6c-87.5-87.5-229.3-87.5-316.8 0s-87.5 229.3 0 316.8s229.3 87.5 316.8 0c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0c-62.5 62.5-163.8 62.5-226.3 0s-62.5-163.8 0-226.3s163.8-62.5 226.3 0L386.3 160z"
					/></svg
				>
				<span class="ml-2 flex">Password Generator</span>
			</a>
		</div>
	{/if}
</div>
