<script lang="ts">
	let cFocus = '';
	let cFocusInput = '';

	const addFocusEffect = () => {
		cFocus = 'border-primary-200 bg-primary-700';
		cFocusInput = 'bg-primary-700';
	};

	const removeFocusEffect = () => {
		cFocus = '';
		cFocusInput = '';
	};
</script>

<div
	class={`${cFocus} border duration-200 border-primary-500 rounded-md p-2 w-[60%] mb-4 flex justify-center items-center bg-primary-500`}
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
		on:focusin={addFocusEffect}
		on:focusout={removeFocusEffect}
		type="text"
		placeholder="Ctrl+K"
		class={`${cFocusInput} duration-200 w-[70%] outline-none bg-primary-500`}
	/>
</div>
