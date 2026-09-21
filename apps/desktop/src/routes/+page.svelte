<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	type CoreStatus = { version: string; architecture: string };
	type BuildProgress = { stage: string; message: string };
	type BuildPreview = {
		plan: string;
		verification: string;
		warnings: string[];
		packageCount: number;
		environments: string[];
	};
	type BuildReport = {
		sealed_build: string;
		lockfile: string;
		log_dir: string;
		actions: number;
		warnings: string[];
	};

	let status = $state<CoreStatus | null>(null);
	let preview = $state<BuildPreview | null>(null);
	let progress = $state<BuildProgress | null>(null);
	let error = $state('');
	let busy = $state(false);
	let result = $state<BuildReport | null>(null);

	let registry = $state('registry');
	let manifest = $state('examples\\bg2ee-tweaks-starter\\modpack.yaml');
	let store = $state('C:\\Games\\IEPM');
	let buildName = $state('my-first-build');
	let sourcesText = $state('target=C:\\Games\\BG2EE-clean');
	let weidu = $state('C:\\Games\\WeiDU-Windows\\weidu.exe');
	let weiduVersion = $state('25100');
	let confirmDisposable = $state(false);
	let allowWarnings = $state(false);

	const stages = [
		['1', 'Choose', 'Manifest and clean game'],
		['2', 'Review', 'Exact plan and warnings'],
		['3', 'Build', 'Copy, install, and seal']
	];

	function sources(): Record<string, string> {
		const entries = sourcesText
			.split(/\r?\n/)
			.map((line) => line.trim())
			.filter(Boolean)
			.map((line) => {
				const separator = line.indexOf('=');
				if (separator <= 0 || separator === line.length - 1) {
					throw new Error(`Source must be NAME=PATH: ${line}`);
				}
				return [line.slice(0, separator), line.slice(separator + 1)];
			});
		return Object.fromEntries(entries);
	}

	function request(confirm: boolean) {
		return {
			registry,
			manifest,
			store,
			build: buildName,
			sources: sources(),
			weidu,
			weiduVersion,
			registryRevision: 'working-tree',
			confirmDisposable: confirm,
			allowWeiduWarnings: allowWarnings
		};
	}

	async function reviewBuild() {
		busy = true;
		error = '';
		result = null;
		try {
			preview = await invoke<BuildPreview>('preview_build', { request: request(false) });
		} catch (reason) {
			error = String(reason);
			preview = null;
		} finally {
			busy = false;
		}
	}

	async function runBuild() {
		busy = true;
		error = '';
		result = null;
		try {
			result = await invoke<BuildReport>('start_build', {
				request: request(confirmDisposable)
			});
		} catch (reason) {
			error = String(reason);
		} finally {
			busy = false;
		}
	}

	onMount(() => {
		let stop: UnlistenFn | undefined;
		void invoke<CoreStatus>('core_status')
			.then((value) => (status = value))
			.catch((reason) => (error = String(reason)));
		void listen<BuildProgress>('iepm://build-progress', (event) => {
			progress = event.payload;
		}).then((unlisten) => (stop = unlisten));
		return () => stop?.();
	});
</script>

<svelte:head>
	<title>Infinity Package Manager</title>
	<meta
		name="description"
		content="Build reproducible Infinity Engine mod installations from a clean game copy."
	/>
</svelte:head>

<div class="min-h-screen bg-paper-100 p-4 lg:p-6">
	<div class="mx-auto grid min-h-[calc(100vh-3rem)] max-w-[1500px] grid-cols-[250px_1fr] overflow-hidden rounded-2xl border border-black/10 bg-paper-50 shadow-[0_28px_80px_rgba(37,34,27,0.12)]">
		<aside class="flex flex-col border-r border-white/10 bg-ink-950 px-5 py-6 text-white">
			<div class="mb-10">
				<div class="mb-3 flex h-10 w-10 items-center justify-center rounded-xl bg-moss-500 text-lg font-semibold">I</div>
				<p class="text-xs font-semibold tracking-[0.18em] text-white/45 uppercase">IEPM desktop</p>
				<h1 class="mt-1 text-xl font-semibold tracking-tight">Build a modded game</h1>
			</div>

			<nav class="space-y-2" aria-label="Build steps">
				{#each stages as stage, index}
					<div class="flex gap-3 rounded-xl px-3 py-3 {index === 0 ? 'bg-white/8' : ''}">
						<div class="flex h-7 w-7 shrink-0 items-center justify-center rounded-full border border-white/15 text-xs text-white/70">{stage[0]}</div>
						<div>
							<p class="text-sm font-medium">{stage[1]}</p>
							<p class="mt-0.5 text-xs leading-5 text-white/45">{stage[2]}</p>
						</div>
					</div>
				{/each}
			</nav>

			<div class="mt-auto rounded-xl border border-white/10 bg-white/5 p-3">
				<div class="flex items-center gap-2 text-xs text-white/60">
					<span class="h-2 w-2 rounded-full bg-emerald-400"></span>
					Rust core connected
				</div>
				<p class="mt-2 text-[11px] leading-4 text-white/35">{status ? `Core ${status.version}` : 'Connecting…'}</p>
			</div>
		</aside>

		<main class="min-w-0 px-7 py-7 lg:px-10">
			<header class="flex items-start justify-between gap-6 border-b border-black/8 pb-6">
				<div>
					<p class="text-xs font-semibold tracking-[0.15em] text-moss-600 uppercase">New build</p>
					<h2 class="mt-2 text-3xl font-semibold tracking-[-0.03em] text-ink-950">Choose your inputs</h2>
					<p class="mt-2 max-w-2xl text-sm leading-6 text-ink-800/65">IEPM keeps the clean game untouched, installs into a disposable copy, and seals only a successful build.</p>
				</div>
				<div class="rounded-full border border-moss-500/25 bg-moss-500/10 px-3 py-1.5 text-xs font-medium text-moss-600">A7 shell</div>
			</header>

			<div class="mt-7 grid grid-cols-[minmax(0,1fr)_330px] gap-7">
				<section class="space-y-5">
					<div class="grid grid-cols-2 gap-4">
						<label class="block">
							<span class="text-xs font-semibold text-ink-800/65">Build name</span>
							<input bind:value={buildName} class="mt-2 w-full rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10" />
						</label>
						<label class="block">
							<span class="text-xs font-semibold text-ink-800/65">IEPM store</span>
							<input bind:value={store} class="mt-2 w-full rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10" />
						</label>
					</div>

					<label class="block">
						<span class="text-xs font-semibold text-ink-800/65">Mod list manifest</span>
						<input bind:value={manifest} class="mt-2 w-full rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10" />
					</label>

					<div class="grid grid-cols-2 gap-4">
						<label class="block">
							<span class="text-xs font-semibold text-ink-800/65">Registry directory</span>
							<input bind:value={registry} class="mt-2 w-full rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10" />
						</label>
						<label class="block">
							<span class="text-xs font-semibold text-ink-800/65">WeiDU version</span>
							<input bind:value={weiduVersion} class="mt-2 w-full rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10" />
						</label>
					</div>

					<label class="block">
						<span class="text-xs font-semibold text-ink-800/65">Clean game sources</span>
						<textarea bind:value={sourcesText} rows="3" class="mt-2 w-full resize-none rounded-lg border border-black/10 bg-white px-3 py-2.5 font-mono text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10"></textarea>
						<span class="mt-1.5 block text-[11px] text-ink-800/45">One NAME=PATH binding per line. EET builds can bind more than one game.</span>
					</label>

					<label class="block">
						<span class="text-xs font-semibold text-ink-800/65">WeiDU executable</span>
						<input bind:value={weidu} class="mt-2 w-full rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10" />
					</label>

					<div class="flex flex-wrap items-center gap-x-6 gap-y-3 rounded-xl border border-black/8 bg-white/60 p-4 text-xs text-ink-800/70">
						<label class="flex items-center gap-2">
							<input type="checkbox" bind:checked={confirmDisposable} class="accent-moss-600" />
							I confirm IEPM may create disposable copies
						</label>
						<label class="flex items-center gap-2">
							<input type="checkbox" bind:checked={allowWarnings} class="accent-moss-600" />
							Accept receipted WeiDU warnings
						</label>
					</div>

					{#if error}<div class="rounded-xl border border-red-500/20 bg-red-50 px-4 py-3 text-sm leading-5 text-red-800">{error}</div>{/if}
					{#if result}
						<div class="rounded-xl border border-emerald-500/25 bg-emerald-50 px-4 py-3 text-sm text-emerald-900">
							<p class="font-semibold">Build complete</p>
							<p class="mt-1 break-all text-xs">{result.sealed_build}</p>
						</div>
					{/if}

					<div class="flex items-center gap-3 pt-1">
						<button onclick={reviewBuild} disabled={busy} class="rounded-lg border border-black/12 bg-white px-5 py-2.5 text-sm font-semibold text-ink-800 shadow-sm hover:bg-paper-50 disabled:opacity-50">Review plan</button>
						<button onclick={runBuild} disabled={busy || !preview || !confirmDisposable} class="rounded-lg bg-moss-600 px-5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-moss-500 disabled:cursor-not-allowed disabled:opacity-40">{busy ? 'Working…' : 'Build and seal'}</button>
						{#if progress}<span class="text-xs text-ink-800/55"><strong class="font-semibold capitalize">{progress.stage}:</strong> {progress.message}</span>{/if}
					</div>
				</section>

				<aside class="min-w-0 rounded-2xl border border-black/8 bg-white p-5 shadow-sm">
					<div class="flex items-center justify-between">
						<h3 class="text-sm font-semibold">Build review</h3>
						<span class="rounded-full bg-paper-100 px-2 py-1 text-[10px] font-semibold tracking-wide text-ink-800/55 uppercase">{preview ? 'Ready' : 'Not checked'}</span>
					</div>

					{#if preview}
						<div class="mt-5 grid grid-cols-2 gap-3">
							<div class="rounded-xl bg-paper-100 p-3"><p class="text-2xl font-semibold">{preview.packageCount}</p><p class="mt-1 text-[11px] text-ink-800/50">packages</p></div>
							<div class="rounded-xl bg-paper-100 p-3"><p class="text-2xl font-semibold">{preview.environments.length}</p><p class="mt-1 text-[11px] text-ink-800/50">game environments</p></div>
						</div>
						{#if preview.warnings.length}
							<div class="mt-4 rounded-xl border border-amber-500/20 bg-amber-500/8 p-3">
								<p class="text-xs font-semibold text-amber-500">Allowed warnings</p>
								<ul class="mt-2 space-y-1 text-[11px] leading-4 text-ink-800/65">{#each preview.warnings as warning}<li>• {warning}</li>{/each}</ul>
							</div>
						{/if}
						<pre class="mt-4 max-h-[390px] overflow-auto whitespace-pre-wrap rounded-xl bg-ink-950 p-4 font-mono text-[10px] leading-4 text-white/70">{preview.plan}</pre>
					{:else}
						<div class="mt-16 text-center">
							<div class="mx-auto flex h-11 w-11 items-center justify-center rounded-full bg-paper-100 text-lg text-ink-800/40">✓</div>
							<p class="mt-4 text-sm font-medium">Review before building</p>
							<p class="mx-auto mt-2 max-w-[230px] text-xs leading-5 text-ink-800/45">The Rust core will resolve exact releases, check fingerprints, and show the full plan.</p>
						</div>
					{/if}
				</aside>
			</div>
		</main>
	</div>
</div>
