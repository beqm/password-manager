<script lang="ts">
	import ItemLaunch from './ItemLaunch.svelte';
	import ItemBtn from './ItemBtn.svelte';
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
			<ItemLaunch message="Launch" content={data.link} isLaunch={true}>
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
			</ItemLaunch>
		{/if}
		<ItemBtn content={data}>
			<svg
				class="hover:text-hover"
				fill="currentColor"
				xmlns="http://www.w3.org/2000/svg"
				height="1.2em"
				viewBox="0 0 448 512"
				><!--! Font Awesome Free 6.4.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
					d="M0 96C0 78.3 14.3 64 32 64H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 128 0 113.7 0 96zM0 256c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 416c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z"
				/></svg
			>
		</ItemBtn>
	</div>
</button>
