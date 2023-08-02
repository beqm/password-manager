<script lang="ts">
	import TableBtn from './TableBtn.svelte';
	import type { Item } from '$lib/types/types';
	export let data: Item;

	let formatModified = new Date(data.last_modified).toLocaleDateString();
	let formatCreated = new Date(data.created_at).toLocaleDateString();
</script>

<button class=" flex w-full items-center justify-evenly hover:bg-primary-800 duration-200">
	<div class="w-[20%] ml-2 p-4 overflow-hidden">{data.title}</div>
	<div class="w-[20%] ml-2 p-4">{formatModified}</div>
	<div class="w-[20%] ml-2 p-4">{formatCreated}</div>
	<div class="w-[15%] ml-2 p-4 justify-end flex">
		{#if data.link}
			<TableBtn message="Launch" content={data.link} isLaunch={true}>
				<svg
					class="hover:text-hover"
					fill="currentColor"
					xmlns="http://www.w3.org/2000/svg"
					height="1.2em"
					viewBox="0 0 512 512"
					><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
						d="M320 0c-17.7 0-32 14.3-32 32s14.3 32 32 32h82.7L201.4 265.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L448 109.3V192c0 17.7 14.3 32 32 32s32-14.3 32-32V32c0-17.7-14.3-32-32-32H320zM80 32C35.8 32 0 67.8 0 112V432c0 44.2 35.8 80 80 80H400c44.2 0 80-35.8 80-80V320c0-17.7-14.3-32-32-32s-32 14.3-32 32V432c0 8.8-7.2 16-16 16H80c-8.8 0-16-7.2-16-16V112c0-8.8 7.2-16 16-16H192c17.7 0 32-14.3 32-32s-14.3-32-32-32H80z"
					/></svg
				>
			</TableBtn>
		{/if}
		<TableBtn message="Copy" content={'google'}>
			<svg
				class="hover:text-hover"
				fill="currentColor"
				xmlns="http://www.w3.org/2000/svg"
				height="1.2em"
				viewBox="0 0 512 512"
				><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
					d="M272 0H396.1c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9V336c0 26.5-21.5 48-48 48H272c-26.5 0-48-21.5-48-48V48c0-26.5 21.5-48 48-48zM48 128H192v64H64V448H256V416h64v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V176c0-26.5 21.5-48 48-48z"
				/></svg
			>
		</TableBtn>
	</div>
</button>
