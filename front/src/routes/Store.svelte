<script lang="ts">
	let qty = 1;
	const unit_price = 200_000;
	let qr_code: String | null = null;
	let invoice: String | null = null;
	let error: String | null = null;
	$: total = unit_price * qty;

	async function buy() {
		try {
			const sats = total;
			const res = await fetch('http://127.0.0.1:3987/invoice', {
				method: 'POST',
				headers: {
				"Content-Type": "application/json",
				},
				body: JSON.stringify({
					sats
				})
			})
			if (!res.ok) {
				error = await res.text();
			}
			let resp = await res.json();
			qr_code = resp['qr_code'];
			invoice = resp['data'];
		} catch (e: any) {
			error = e;
		}
	}

	// TODO: animate QR Code https://svelte.dev/tutorial/bind-this
</script>

<p>
	<!-- <label> -->
	Price: {unit_price}<br />
	Quantity: {qty}
	<input type="range" bind:value={qty} min="0" max="10" /><br />
	Total: {total}<br />
	<!-- </label> -->
	<br />
	<button on:click={buy}> Buy </button>
</p>

{#if error}
	<p>Error: {error}</p>
{:else if qr_code}
	<p>QR Code: {@html qr_code}</p>
{:else if invoice}
	<p>Invoice: {invoice}</p>
{/if}


<style>
	input,
	p {
		margin: 6px;
	}

	:global(svg rect) {
		animation: contour 6s ease-in-out infinite;
	}

	@keyframes contour {
		0% {fill:#ef5455}
		50% {fill:#f77f00}
		100% {fill:#ef5455}
	}

	:global(svg path) {
		animation: interieur 6s ease-in-out infinite;
	}

	@keyframes interieur {
		0% {fill:#2b3252}
		50% {fill:#5e412f}
		100% {fill:#2b3252}
	}
</style>
