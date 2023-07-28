<script lang="ts">
	import { generatePassword } from '$lib/utils';
	import { onMount } from 'svelte';
	import TableBtn from '$lib/components/TableBtn.svelte';

	let password: string = '';
	let length: number = 16;
	let upper: boolean = false;
	let number: boolean = false;
	let symbol: boolean = false;

	const update = async () => {
		password = await generatePassword(length, upper, number, symbol);
	};

	onMount(async () => {
		await update();
	});
</script>

<div class="flex justify-center h-full">
	<div class="w-[90%]">
		<div class="flex mt-20 h-[5%] items-center justify-center">
			<h1 class="flex text-3xl font-bold">Password Generator</h1>
		</div>
		<div class="w-full mt-10 drop-shadow-xl h-[70%] rounded-md">
			<div class="flex flex-col text-lg w-full h-full items-center">
				<div
					class="flex flex-col justify-center items-center mt-10 h-[40%] w-[60%] text-2xl bg-secondary-900"
				>
					<div class="p-2 w-full h-full xl:flex xl:justify-center xl:items-center break-words">
						{password}
					</div>
				</div>
				<div class="flex justify-end text-sm w-[60%]">
					<TableBtn message="Copy" content={password}>
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

				<div class="flex flex-col mt-10 w-[60%]">
					<div class="flex items-center">
						<span>Password Length</span>
						<input
							min="8"
							max="40"
							class="bg-secondary-800 ml-auto outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit p-2"
							type="range"
							on:change={update}
							bind:value={length}
						/>
						<div class="ml-4 w-[11px]">{length}</div>
					</div>
					<div class="flex items-center">
						<span>Use capital letters</span>
						<input
							class="bg-secondary-800 ml-auto outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit p-2"
							type="checkbox"
							on:click={update}
							bind:checked={upper}
						/>
					</div>

					<div class="flex items-center">
						<span>Use digits</span>
						<input
							class="bg-secondary-800 ml-auto outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit p-2"
							type="checkbox"
							on:click={update}
							bind:checked={number}
						/>
					</div>

					<div class="flex items-center">
						<span>Use symbols</span>
						<input
							class="bg-secondary-800 ml-auto outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit p-2"
							type="checkbox"
							on:click={update}
							bind:checked={symbol}
						/>
					</div>
				</div>

				<div class="flex mt-10 w-[25%] justify-evenly">
					<button
						on:click={update}
						class="hover:bg-primary-700 w-1/3 min-w-[90px] active:scale-90 border duration-200 border-primary-600 rounded-md p-2 flex justify-center items-center bg-primary-500"
						>Generate</button
					>
				</div>
			</div>
		</div>
	</div>
</div>
