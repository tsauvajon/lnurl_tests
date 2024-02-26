<script lang="ts">
	import ZapIcon from './ZapIcon.svelte';
	import Card from './Card.svelte';

	let qr_code: String | null = null;
	let invoice: String | null = null;
	let error: String | null = null;

	let qty: number = 1;
	const unit_price: number = 200_000;
	$: total = unit_price * qty;

	let zap_amount: number = 2000;
	$: zap_amount_usd = (zap_amount / 1930).toFixed(2);

	async function buy() {
		get_invoice(total);
	}

	async function zap() {
		get_invoice(zap_amount);
	}

	async function get_invoice(sats: number) {
		error = null;
		try {
			const res = await fetch('http://127.0.0.1:3987/invoice', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					sats
				})
			});
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
</script>

<h2>Self-generate invoices</h2>

<table>
	<tr>
		<th style="background: grey">Self-checkout demo</th>
		<th style="background: lightgrey">Zap demo</th>
	</tr>

	<tr>
		<td style="background: grey">
			<p>
				<br />
				<span>
					Price: {unit_price} sats
				</span>
				<span>
					Quantity: {qty} item
					<input type="range" bind:value={qty} min="0" max="10" />
				</span>
				<span>
					Total: {total} sats
				</span>
				<br /><br />
				<span>
					<button on:click={buy}>Buy</button>
				</span>
			</p>
		</td>

		<td style="background: lightgrey">
			<p>
				<span>
					<button on:click={() => (zap_amount = 50)}>50</button>&nbsp;
					<button on:click={() => (zap_amount = 1000)}>1k</button>&nbsp;
					<button on:click={() => (zap_amount = 5000)}>5k</button>&nbsp;
					<button on:click={() => (zap_amount = 100_000)}>100k</button>
				</span>
				<br />
				<!-- TODO: modal? -->
				<span>
					Custom:
					<input type="number" bind:value={zap_amount} min="20" max="40000" /><br />
					<input type="range" bind:value={zap_amount} min="20" max="40000" /><br />
					($ {zap_amount_usd})
				</span>
				<br />
				<span>
					<button class="pulse" on:click={zap}><ZapIcon />Zap {zap_amount} sats!</button>
				</span>
			</p>
		</td>
	</tr>
</table>

{#if error}
	<p>Error: {error}</p>
{:else if qr_code}
	<p class="qr_code">{@html qr_code}</p>
	<br />
	<p>
		{#if invoice}
			<Card
				header="Raw invoice"
				content={invoice}
				button="Copy to clipboard"
				on:buttonClicked={() => navigator.clipboard.writeText(invoice)}
			/>
		{/if}
	</p>
{:else if invoice}
	<p>Invoice: {invoice}</p>
{/if}

<style>
	input,
	p {
		margin: 6px;
	}

	button,
	span {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.pulse:hover {
		animation: pulse-animation 3s infinite;
	}

	@keyframes pulse-animation {
		0% {
			box-shadow: 0 0 0 0px #3c4fe0;
		}
		100% {
			box-shadow: 0 0 0 20px rgba(0, 0, 0, 0);
		}
	}

	:global(.qr_code svg rect) {
		animation: contour 6s ease-in-out infinite;
	}

	@keyframes contour {
		0% {
			fill: #ef5455;
		}
		50% {
			fill: #f77f00;
		}
		100% {
			fill: #ef5455;
		}
	}

	:global(.qr_code svg path) {
		animation: interieur 6s ease-in-out infinite;
	}

	@keyframes interieur {
		0% {
			fill: #2b3252;
		}
		50% {
			fill: #5e412f;
		}
		100% {
			fill: #2b3252;
		}
	}

	:global(.qr_code svg) {
		animation: rotate 10s ease-in-out infinite;
	}

	@keyframes rotate {
		0% {
			transform: rotate3d(0);
		}
		25% {
			transform: rotate3d(1, 1, 1, 5deg);
		}
		50% {
			transform: rotate3d(0);
		}
		75% {
			transform: rotate3d(1, 1, 1, -5deg);
		}
		100% {
			transform: rotate3d(0);
		}
	}
</style>
