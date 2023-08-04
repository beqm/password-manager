<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api';
	import { slide } from 'svelte/transition';
	import PasswordInput from '$lib/components/PasswordInput.svelte';
	import TableBtn from '$lib/components/ItemLaunch.svelte';
	import { onMount } from 'svelte';

	let username: string = '';
	let password: string = '';
	let confirmPassword: string = '';
	let registered: boolean = false;
	let timer: number = 20;
	let recoveryCode: string;

	$: {
		if (timer == 0) {
			goto('login');
		}
	}

	let cUser: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cUserError: string = 'invisible';
	let cUserMsg: string = 'This field cannot be empty!';

	let cPassword: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cPasswordError: string = 'invisible';
	let cPasswordMsg: string = 'This field cannot be empty!';

	let cConfirm: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cConfirmError: string = 'invisible';
	let cConfirmMsg: string = 'This field cannot be empty!';

	const validateUserEmpty = () => {
		if (username.length == 0) {
			cUserError = 'text-error';
			cUser =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			return false;
		} else {
			cUserError = 'invisible';
			cUser = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			return true;
		}
	};

	const validatePasswordEmpty = () => {
		if (password.length == 0) {
			cPasswordError = 'text-error';
			cPassword =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			cPasswordMsg = 'This field cannot be empty!';
			return false;
		} else {
			cPasswordError = 'invisible';
			cPassword = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			return true;
		}
	};

	const validateConfirmEmpty = () => {
		if (confirmPassword.length == 0) {
			cConfirmError = 'text-error';
			cConfirm =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			cConfirmMsg = 'This field cannot be empty!';
			return false;
		} else {
			cConfirmError = 'invisible';
			cConfirm = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			return true;
		}
	};

	const validatePasswordMatch = () => {
		if ((password == '' && confirmPassword == '') || password !== confirmPassword) {
			cPasswordError = 'text-error';
			cConfirmError = 'text-error';
			cPassword =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			cConfirm =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			if (password == '' && confirmPassword == '') {
				cPasswordMsg = 'This field cannot be empty!';
				cConfirmMsg = 'This field cannot be empty!';
			} else {
				cPasswordMsg = 'Passwords Dont Match!';
				cConfirmMsg = 'Passwords Dont Match!';
			}
			return false;
		} else {
			cPasswordError = 'invisible';
			cConfirmError = 'invisible';
			cPassword = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			cConfirm = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';

			cPasswordMsg = 'This field cannot be empty!';
			cConfirmMsg = 'This field cannot be empty!';
			return true;
		}
	};

	const handleRegister = async () => {
		let canRegister: boolean = false;

		if (validateUserEmpty()) {
			canRegister = true;
		} else {
			canRegister = false;
		}

		if (validatePasswordMatch()) {
			canRegister = true;
		} else {
			canRegister = false;
		}

		if (canRegister) {
			let result: string = await invoke('register', { username, masterPassword: password });
			let data: TauriResponse = JSON.parse(result);
			console.log(result);
			if (data.status == 400) {
				cUserError = 'text-error';
				cUser =
					'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
				cUserMsg = 'Username already taken!';
			} else if (data.status == 200) {
				registered = true;
				recoveryCode = data.data;
				setInterval(() => {
					timer -= 1;
				}, 1000);
			}
		}
	};
</script>

<div
	class="absolute top-0 left-0 w-full h-full bg-primary-1000 overflow-hidden flex justify-center"
>
	{#if registered}
		<div transition:slide class="flex flex-col w-[50%] lg:w-[30%] h-[70%] mt-40">
			<div class="flex flex-col items-center justify-center w-full mb-10">
				<h1 class="flex text-3xl font-bold mt-10">Success!</h1>
				<span class="text-2xl mt-2">{timer}</span>
			</div>

			<div class=" w-full bg-primary-900 font-bold text-xl">
				<div class="p-8 xl:p-20 break-words text-center">
					{recoveryCode}
				</div>
				<div class="flex justify-end text-sm">
					<TableBtn message="Copy" content={recoveryCode}>
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
			</div>
			<span class="mt-2 text-hover text-sm"
				>This is your recovery code in case you forget master password</span
			>
		</div>
	{:else}
		<form
			class="flex flex-col w-[50%] lg:w-[30%] h-[70%] mt-40"
			on:submit|preventDefault={handleRegister}
		>
			<div class="flex items-center justify-center w-full mb-10">
				<h1 class="flex text-3xl font-bold mt-10">Register</h1>
			</div>

			<div class="w-full">
				<input
					class={`${cUser} w-full outline-none border rounded-md mb-2 h-fit p-2`}
					type="text"
					placeholder="Username"
					on:keyup={validateUserEmpty}
					bind:value={username}
				/>
				<span class={cUserError}>{cUserMsg}</span>

				<PasswordInput
					on:keyup={validatePasswordEmpty}
					bind:input={password}
					msg={cPasswordMsg}
					placeholder={'Master Password'}
					bind:cInput={cPassword}
					bind:cError={cPasswordError}
				/>
				<PasswordInput
					on:keyup={validateConfirmEmpty}
					bind:input={confirmPassword}
					msg={cConfirmMsg}
					placeholder={'Confirm Master Password'}
					bind:cInput={cConfirm}
					bind:cError={cConfirmError}
				/>
			</div>
			<div class="w-full flex justify-end">
				<button
					type="submit"
					class="bg-primary-600 w-[25%] max-w-[100px] text-center border-primary-700 active:scale-90 hover:bg-primary-700 border duration-200 rounded-md p-2"
					>Create</button
				>
			</div>

			<div class="text-center text-lg mt-4">
				Already a user? <a href="login" class="text-hover">Login</a>
			</div>
		</form>
	{/if}
</div>
