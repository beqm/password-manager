<script lang="ts">
	import { goto } from '$app/navigation';

	let cFocus = '';
	let cFocusInput = '';

	let isCtrlPressed = false;
	let isKPressed = false;

	const addFocusEffect = () => {
		cFocus = 'border-primary-200 bg-primary-700';
		cFocusInput = 'bg-primary-700';
	};

	const removeFocusEffect = () => {
		cFocus = '';
		cFocusInput = '';
	};

	const onBind = () => {
		const searchBar = document.getElementById('search-bar');
		if (searchBar) {
			searchBar.focus();
		}
	};

	const onEnter = (event: KeyboardEvent) => {
		if (event.repeat) return;

		switch (event.key) {
			case 'Enter':
				const searchBar = document.getElementById('search-bar') as HTMLInputElement;
				if (searchBar.value == '') {
					goto('/');
				} else {
					goto('/results');
				}
		}
	};

	const onKeyDown = (event: KeyboardEvent) => {
		if (event.repeat) return;
		switch (event.key) {
			case 'Control':
				isCtrlPressed = true;
				event.preventDefault();
				break;

			case 'k':
				isKPressed = true;
				event.preventDefault();
				break;
		}

		if (isCtrlPressed && isKPressed) {
			onBind();
		}
	};

	const onKeyUp = (event: KeyboardEvent) => {
		switch (event.key) {
			case 'Control':
				isCtrlPressed = false;
				event.preventDefault();
				break;

			case 'k':
				isKPressed = false;
				event.preventDefault();
				break;
		}
	};
</script>

<svelte:window on:keydown={onKeyDown} on:keyup={onKeyUp} />

<div
	class={`${cFocus} border duration-200 border-primary-500 rounded-md p-2 w-[60%] max-w-[150px] mb-4 flex justify-center items-center bg-primary-500`}
>
	<svg
		class="mr-2 w-1/3"
		fill="currentColor"
		xmlns="http://www.w3.org/2000/svg"
		height="1em"
		viewBox="0 0 512 512"
		><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
			d="M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z"
		/></svg
	>

	<input
		on:keydown={onEnter}
		id="search-bar"
		on:focusin={addFocusEffect}
		on:focusout={removeFocusEffect}
		type="text"
		placeholder="Ctrl+K"
		class={`${cFocusInput} duration-200 w-[70%] outline-none bg-primary-500`}
	/>
</div>
