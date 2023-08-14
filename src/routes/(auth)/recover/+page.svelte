<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api';

	let username: string = '';
	let cUser: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cUserError: string = 'invisible';
	let cUserMsg: string = 'This field cannot be empty!';

	let recoveryCode: string = '';
	let cRecover: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cRecoverError: string = 'invisible';
	let cRecoverMsg: string = 'This field cannot be empty!';

	const validateRecoverEmpty = () => {
		if (recoveryCode.length == 0) {
			cRecoverError = 'text-error';
			cRecover =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';

			return false;
		} else {
			cRecoverError = 'invisible';
			cRecover = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';

			return true;
		}
	};

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

	const handleLogin = async () => {
		let validRecoveryCode: boolean = false;

		if (validateUserEmpty()) {
			validRecoveryCode = true;
		} else {
			validRecoveryCode = false;
		}

		if (validateRecoverEmpty()) {
			validRecoveryCode = true;
		} else {
			validRecoveryCode = false;
		}

		if (validRecoveryCode) {
			let result: string = await invoke('verify_recovery_code', { username, recoveryCode });
			let data: TauriResponse = JSON.parse(result);
			if (data.status == 200) {
				console.log(data);
				await localStorage.setItem('temp_value', data.data);
				goto('/recover/change_password');
			}

			if (data.status == 400) {
				cRecoverError = 'text-error';
				cRecover =
					'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';

				cRecoverMsg = 'Incorrect Recovery Code!';
			}
		}
	};
</script>

<div
	class="absolute top-0 left-0 w-full h-full bg-primary-1000 overflow-hidden flex justify-center"
>
	<form
		class="flex flex-col w-[50%] lg:w-[30%] h-[70%] mt-40"
		on:submit|preventDefault={handleLogin}
	>
		<div class="flex items-center justify-center w-full mb-10">
			<h1 class="flex text-3xl font-bold mt-10">Enter Recovery Code</h1>
		</div>

		<div class="w-full relative">
			<a class="text-hover text-xs mb-2 hover:text-dark" href="login">Remembered Master password?</a
			>
			<input
				on:keyup={validateUserEmpty}
				class={`${cUser} w-full mt-2 outline-none border mb-2 rounded-md h-fit p-2`}
				type="text"
				placeholder="Username"
				bind:value={username}
			/>
			<span class={`${cUserError}`}>{cUserMsg}</span>
			<input
				on:keyup={validateRecoverEmpty}
				class={`${cRecover} w-full outline-none border mt-2 rounded-md mb-2 h-fit p-2`}
				type="text"
				placeholder="Recovery Code"
				bind:value={recoveryCode}
			/>
			<span class={`${cRecoverError}`}>{cRecoverMsg}</span>
		</div>
		<div class="w-full flex justify-end mt-2">
			<a
				href="/login"
				class="bg-primary-600 w-[25%] max-w-[100px] mr-4 text-center border-primary-700 active:scale-90 hover:bg-primary-700 border duration-200 rounded-md p-2"
				>Back</a
			>
			<button
				type="submit"
				class="bg-primary-600 w-[25%] max-w-[100px] text-center border-primary-700 active:scale-90 hover:bg-primary-700 border duration-200 rounded-md p-2"
				>Enter</button
			>
		</div>
	</form>
</div>
