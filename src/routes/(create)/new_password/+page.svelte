<script lang="ts">
	import { afterNavigate } from '$app/navigation';
	import { base } from '$app/paths';
	import { goto } from '$app/navigation';
	import { generatePassword } from '$lib/utils';
	import SettingStore from '$lib/stores/SettingStore';

	let title: string = '';
	let websiteUrl: string = '';
	let usernameOrEmail: string = '';
	let password: string = '';

	let cTitle: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cErrorMsg: string = 'invisible';
	let cCreateBtn: string = 'bg-primary-700 border-primary-700';
	let showPassword = false;

	let previousPage: string = base;
	afterNavigate(({ from }) => {
		previousPage = from?.url.pathname || previousPage;
	});

	const validateTitle = () => {
		if (title.length == 0) {
			cErrorMsg = 'text-error';
			cTitle =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			cCreateBtn = 'bg-primary-700 border-primary-700';
			return true;
		} else {
			cErrorMsg = 'invisible';
			cTitle = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			cCreateBtn = 'bg-primary-600 border-primary-700 active:scale-90';
			return false;
		}
	};

	const createPassword = () => {
		if (!validateTitle()) {
			let passwordItem = {
				title,
				websiteUrl,
				usernameOrEmail,
				password
			};
			console.log(passwordItem);
		}
	};

	const cancelBtn = () => {
		goto(previousPage);
	};

	const callGenerate = async () => {
		password = await generatePassword($SettingStore);
	};
</script>

<div class="flex justify-center h-full">
	<div class="w-[90%]">
		<div class="flex mt-20 h-[5%] items-center justify-center">
			<h1 class="flex text-3xl font-bold">New Password</h1>
		</div>
		<div class="w-full mt-10 drop-shadow-xl h-[70%] rounded-md">
			<form
				class="flex flex-col text-lg w-full items-center"
				on:submit|preventDefault={createPassword}
			>
				<div class="flex flex-col mt-10 w-[60%]">
					<span class="text-hover text-xs mb-2">Required</span>
					<input
						on:keyup={validateTitle}
						class={`${cTitle} outline-none border rounded-md h-fit p-2`}
						type="text"
						placeholder="Title"
						bind:value={title}
					/>
					<span class={`${cErrorMsg} mt-2`}>Title cannot be empty!</span>

					<input
						class="bg-secondary-800 outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit mt-6 p-2"
						type="text"
						placeholder="Website Link"
						bind:value={websiteUrl}
					/>
				</div>

				<div class="flex flex-col mt-10 w-[60%]">
					<h1 class="mb-4 text-xl">Login Fields</h1>
					<input
						class="bg-secondary-800 outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit p-2"
						type="text"
						placeholder="Username or Email"
						bind:value={usernameOrEmail}
					/>
					{#if showPassword}
						<input
							class="bg-secondary-800 outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit mt-6 p-2"
							type="text"
							placeholder="Password"
							bind:value={password}
						/>
						<button>Generate Password</button>
					{:else}
						<div class="flex flex-col">
							<input
								class="bg-secondary-800 outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit mt-6 p-2"
								type="password"
								placeholder="Password"
								bind:value={password}
							/>
							<button
								on:click={callGenerate}
								class="mr-auto mt-2 text-sm text-hover hover:text-accent">Generate Password</button
							>
						</div>
					{/if}
				</div>

				<div class="flex mt-10 w-[25%] justify-evenly">
					<button
						on:click={cancelBtn}
						type="button"
						class="hover:bg-primary-700 w-1/3 border duration-200 border-primary-600 rounded-md p-2 active:scale-90 flex justify-center items-center bg-primary-500"
						>Cancel</button
					>
					<button
						type="submit"
						class={`${cCreateBtn} hover:bg-primary-700 w-1/3 border duration-200 border-primary-600 rounded-md p-2 flex justify-center items-center bg-primary-500`}
						>Create</button
					>
				</div>
			</form>
		</div>
	</div>
</div>

<style>
	div::-webkit-scrollbar::hover {
		width: 12px; /* width of the entire scrollbar */
	}

	div::-webkit-scrollbar-track {
		background: none; /* color of the tracking area */
	}

	div::-webkit-scrollbar-thumb {
		background-color: #3c3d5d;
		border-radius: 20px; /* roundness of the scroll thumb */
		border: 3px solid #2f1e1e; /* creates padding around scroll thumb */
	}
</style>
