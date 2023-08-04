<script lang="ts">
	import { afterNavigate } from '$app/navigation';
	import { base } from '$app/paths';
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api';

	let title: string = '';
	let description: string = '';

	let cTitle: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cErrorMsg: string = 'invisible';
	let cCreateBtn: string = 'bg-primary-700 border-primary-700';

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
			return false;
		} else {
			cErrorMsg = 'invisible';
			cTitle = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			cCreateBtn = 'bg-primary-600 border-primary-700 active:scale-90';
			return true;
		}
	};

	const createNote = async () => {
		let canRegister: boolean = false;

		if (validateTitle()) {
			canRegister = true;
		} else {
			canRegister = false;
		}

		if (canRegister) {
			let user: Client = JSON.parse(localStorage.getItem('client') || '');
			await invoke('add_item', {
				username: user.username,
				title,
				identify: '',
				pass: '',
				desc: description,
				link: '',
				type: 'note'
			});
			// TODO: Add item viewer and redirect to that later.
			goto(previousPage);
		}
	};

	const cancelBtn = () => {
		goto(previousPage);
	};
</script>

<div class="flex justify-center items-center h-full overflow-y-scroll">
	<div class="w-full drop-shadow-xl xl:mt-20 h-[70%] rounded-md">
		<div class="flex items-center justify-center">
			<h1 class="flex text-3xl font-bold">New Note</h1>
		</div>
		<form
			class="flex flex-col text-lg w-full h-full items-center"
			on:submit|preventDefault={createNote}
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
			</div>

			<textarea
				class="flex w-[60%] bg-secondary-800 h-[40%] outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md mt-6 p-2 resize-none"
				placeholder="Description"
				bind:value={description}
			/>

			<div class="flex mt-10 mb-10 justify-center">
				<button
					on:click={cancelBtn}
					type="button"
					class="hover:bg-primary-700 mr-10 border duration-200 border-primary-600 rounded-md p-2 active:scale-90 flex justify-center items-center bg-primary-500"
					>Cancel</button
				>
				<button
					type="submit"
					class={`${cCreateBtn} hover:bg-primary-700   border duration-200 border-primary-600 rounded-md p-2 flex justify-center items-center bg-primary-500`}
					>Create</button
				>
			</div>
		</form>
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
