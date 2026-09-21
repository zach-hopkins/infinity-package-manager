<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	type GameDetection = { game: 'bgee' | 'bg2ee'; label: string; path: string };
	type StartupState = {
		libraryLocation: string | null;
		weiduOverride: string | null;
		detectedGames: GameDetection[];
		defaultWeiduVersion: string;
	};
	type BuildProgress = { stage: string; message: string };
	type BuildPreview = {
		plan: string;
		verification: string;
		warnings: string[];
		packageCount: number;
		environments: string[];
		weiduVersion: string;
	};
	type BuildReport = {
		sealed_build: string;
		lockfile: string;
		log_dir: string;
		actions: number;
		warnings: string[];
		weidu_warnings: { action: number; package: string; log: string; details: string }[];
	};

	let startup = $state<StartupState | null>(null);
	let preview = $state<BuildPreview | null>(null);
	let progress = $state<BuildProgress | null>(null);
	let error = $state('');
	let busy = $state(false);
	let result = $state<BuildReport | null>(null);
	let settingsOpen = $state(false);

	let libraryLocation = $state('');
	let weiduOverride = $state('');
	let manifest = $state('');
	let experienceName = $state('');
	let bgeeDirectory = $state('');
	let bg2eeDirectory = $state('');
	let showInstallWarnings = $state(false);

	const stages = [
		['1', 'Choose', 'Games and mod list'],
		['2', 'Review', 'Exact plan and warnings'],
		['3', 'Install', 'Build a separate game copy']
	];

	function request() {
		return {
			manifest,
			experienceName: experienceName.trim() || null,
			bgeeDirectory: bgeeDirectory.trim() || null,
			bg2eeDirectory: bg2eeDirectory.trim() || null,
		};
	}

	async function pickDirectory(current: string): Promise<string | null> {
		const selected = await open({
			directory: true,
			multiple: false,
			defaultPath: current || undefined,
			title: 'Choose a folder'
		});
		return typeof selected === 'string' ? selected : null;
	}

	async function pickLibrary() {
		const selected = await pickDirectory(libraryLocation);
		if (selected) libraryLocation = selected;
	}

	async function pickGame(kind: 'bgee' | 'bg2ee') {
		const selected = await pickDirectory(kind === 'bgee' ? bgeeDirectory : bg2eeDirectory);
		if (!selected) return;
		if (kind === 'bgee') bgeeDirectory = selected;
		else bg2eeDirectory = selected;
	}

	async function pickManifest() {
		const selected = await open({
			multiple: false,
			filters: [{ name: 'IEPM mod lists', extensions: ['yaml', 'yml'] }],
			defaultPath: manifest || undefined,
			title: 'Choose a mod list'
		});
		if (typeof selected === 'string') manifest = selected;
	}

	async function pickWeiduOverride() {
		const selected = await open({
			multiple: false,
			filters: [{ name: 'WeiDU executable', extensions: ['exe'] }],
			defaultPath: weiduOverride || undefined,
			title: 'Choose a local WeiDU v251 executable'
		});
		if (typeof selected === 'string') weiduOverride = selected;
	}

	function applyDetections(state: StartupState) {
		for (const game of state.detectedGames) {
			if (game.game === 'bgee' && !bgeeDirectory) bgeeDirectory = game.path;
			if (game.game === 'bg2ee' && !bg2eeDirectory) bg2eeDirectory = game.path;
		}
	}

	async function loadStartup() {
		const state = await invoke<StartupState>('startup_state');
		startup = state;
		libraryLocation = state.libraryLocation ?? libraryLocation;
		weiduOverride = state.weiduOverride ?? '';
		applyDetections(state);
	}

	async function saveWeiduOverride() {
		busy = true;
		error = '';
		try {
			const state = await invoke<StartupState>('save_weidu_override', { path: weiduOverride.trim() || null });
			startup = state;
			weiduOverride = state.weiduOverride ?? '';
		} catch (reason) {
			error = String(reason);
		} finally {
			busy = false;
		}
	}

	async function saveLibrary() {
		busy = true;
		error = '';
		try {
			const state = await invoke<StartupState>('save_library_location', { path: libraryLocation });
			startup = state;
			libraryLocation = state.libraryLocation ?? libraryLocation;
			applyDetections(state);
			settingsOpen = false;
		} catch (reason) {
			error = String(reason);
		} finally {
			busy = false;
		}
	}

	async function reviewPlan() {
		busy = true;
		error = '';
		result = null;
		try {
			preview = await invoke<BuildPreview>('preview_build', { request: request() });
		} catch (reason) {
			error = String(reason);
			preview = null;
		} finally {
			busy = false;
		}
	}

	async function installExperience() {
		busy = true;
		error = '';
		result = null;
		try {
			result = await invoke<BuildReport>('start_build', { request: request() });
		} catch (reason) {
			error = String(reason);
		} finally {
			busy = false;
		}
	}

	onMount(() => {
		let stop: UnlistenFn | undefined;
		void loadStartup().catch((reason) => (error = String(reason)));
		void listen<BuildProgress>('iepm://build-progress', (event) => {
			progress = event.payload;
		}).then((unlisten) => (stop = unlisten));
		return () => stop?.();
	});
</script>

<svelte:head>
	<title>Infinity Package Manager</title>
	<meta name="description" content="Build a separate, reproducible Infinity Engine mod experience." />
</svelte:head>

{#if startup && !startup.libraryLocation}
	<div class="grid min-h-screen place-items-center bg-paper-100 p-6">
		<section class="w-full max-w-xl rounded-2xl border border-black/10 bg-paper-50 p-8 shadow-[0_28px_80px_rgba(37,34,27,0.12)]">
			<div class="flex h-11 w-11 items-center justify-center rounded-xl bg-moss-600 text-lg font-semibold text-white">I</div>
			<p class="mt-6 text-xs font-semibold tracking-[0.17em] text-moss-600 uppercase">First-time setup</p>
			<h1 class="mt-2 text-3xl font-semibold tracking-[-0.03em]">Where should IEPM keep its library?</h1>
			<p class="mt-3 max-w-lg text-sm leading-6 text-ink-800/65">
				IEPM keeps downloaded mods, clean snapshots, temporary game copies, logs, and finished mod experiences here. Choose a drive with room for several full game copies. You can change it later.
			</p>
			<label class="mt-7 block">
				<span class="text-xs font-semibold text-ink-800/65">IEPM library folder</span>
				<div class="mt-2 flex gap-2">
					<input bind:value={libraryLocation} placeholder="For example: D:\Games\IEPM" class="min-w-0 flex-1 rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10" />
					<button onclick={pickLibrary} class="rounded-lg border border-black/12 bg-white px-4 text-sm font-semibold hover:bg-paper-50">Browse</button>
				</div>
			</label>
			{#if error}<p class="mt-4 rounded-lg bg-red-50 px-3 py-2 text-sm text-red-800">{error}</p>{/if}
			<div class="mt-7 flex items-center justify-end">
				<button onclick={saveLibrary} disabled={busy || !libraryLocation.trim()} class="rounded-lg bg-moss-600 px-5 py-2.5 text-sm font-semibold text-white hover:bg-moss-500 disabled:opacity-40">{busy ? 'Saving…' : 'Continue'}</button>
			</div>
		</section>
	</div>
{:else if startup}
	<div class="min-h-screen bg-paper-100 p-4 lg:p-6">
		<div class="mx-auto grid min-h-[calc(100vh-3rem)] max-w-[1500px] grid-cols-[250px_1fr] overflow-hidden rounded-2xl border border-black/10 bg-paper-50 shadow-[0_28px_80px_rgba(37,34,27,0.12)]">
			<aside class="flex flex-col border-r border-white/10 bg-ink-950 px-5 py-6 text-white">
				<div class="mb-10">
					<div class="mb-3 flex h-10 w-10 items-center justify-center rounded-xl bg-moss-500 text-lg font-semibold">I</div>
					<p class="text-xs font-semibold tracking-[0.18em] text-white/45 uppercase">IEPM</p>
					<h1 class="mt-1 text-xl font-semibold tracking-tight">Your mod experience</h1>
				</div>
				<nav class="space-y-2" aria-label="Install steps">
					{#each stages as stage, index}
						<div class="flex gap-3 rounded-xl px-3 py-3 {index === 0 ? 'bg-white/8' : ''}">
							<div class="flex h-7 w-7 shrink-0 items-center justify-center rounded-full border border-white/15 text-xs text-white/70">{stage[0]}</div>
							<div><p class="text-sm font-medium">{stage[1]}</p><p class="mt-0.5 text-xs leading-5 text-white/45">{stage[2]}</p></div>
						</div>
					{/each}
				</nav>
				<div class="mt-auto rounded-xl border border-white/10 bg-white/5 p-3">
					<div class="flex items-center justify-between gap-2 text-xs text-white/60"><span><span class="mr-2 inline-block h-2 w-2 rounded-full bg-emerald-400"></span>Ready</span><button onclick={() => (settingsOpen = true)} class="text-white/65 underline hover:text-white">Settings</button></div>
					<p class="mt-2 truncate text-[11px] text-white/35" title={libraryLocation}>{libraryLocation}</p>
				</div>
			</aside>

			<main class="min-w-0 px-7 py-7 lg:px-10">
				<header class="flex items-start justify-between gap-6 border-b border-black/8 pb-6">
					<div>
						<p class="text-xs font-semibold tracking-[0.15em] text-moss-600 uppercase">New mod experience</p>
						<h2 class="mt-2 text-3xl font-semibold tracking-[-0.03em] text-ink-950">Choose your games and mod list</h2>
						<p class="mt-2 max-w-2xl text-sm leading-6 text-ink-800/65">Your original game folders stay untouched. IEPM creates and installs into separate, disposable copies.</p>
					</div>
					<div class="rounded-full border border-moss-500/25 bg-moss-500/10 px-3 py-1.5 text-xs font-medium text-moss-600">WeiDU {startup.defaultWeiduVersion} · {startup.weiduOverride ? 'local override' : 'automatic'}</div>
				</header>

				<div class="mt-7 grid grid-cols-[minmax(0,1fr)_330px] gap-7">
					<section class="space-y-5">
						<label class="block"><span class="text-xs font-semibold text-ink-800/65">Mod Experience Name <span class="font-normal text-ink-800/40">optional</span></span><input bind:value={experienceName} placeholder="For example: My first BG2 adventure" class="mt-2 w-full rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10" /><span class="mt-1.5 block text-[11px] text-ink-800/45">Leave blank and IEPM will give this experience a safe name.</span></label>

						<label class="block"><span class="text-xs font-semibold text-ink-800/65">BG2:EE install folder</span><div class="mt-2 flex gap-2"><input bind:value={bg2eeDirectory} placeholder="Choose your clean BG2:EE game folder" class="min-w-0 flex-1 rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10" /><button onclick={() => pickGame('bg2ee')} class="rounded-lg border border-black/12 bg-white px-4 text-sm font-semibold hover:bg-paper-50">Browse</button></div></label>

						<label class="block"><span class="text-xs font-semibold text-ink-800/65">BG:EE / Siege of Dragonspear install folder <span class="font-normal text-ink-800/40">only needed for EET</span></span><div class="mt-2 flex gap-2"><input bind:value={bgeeDirectory} placeholder="Choose only when your mod list needs BG:EE" class="min-w-0 flex-1 rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10" /><button onclick={() => pickGame('bgee')} class="rounded-lg border border-black/12 bg-white px-4 text-sm font-semibold hover:bg-paper-50">Browse</button></div></label>

						{#if startup.detectedGames.length}
							<div class="rounded-xl border border-moss-500/15 bg-moss-500/7 p-4"><p class="text-xs font-semibold text-moss-600">Games found on this computer</p><div class="mt-2 space-y-1.5">{#each startup.detectedGames as game}<button onclick={() => game.game === 'bgee' ? (bgeeDirectory = game.path) : (bg2eeDirectory = game.path)} class="block w-full truncate text-left text-xs text-ink-800/70 hover:text-moss-600">Use {game.label}: {game.path}</button>{/each}</div></div>
						{/if}

						<label class="block"><span class="text-xs font-semibold text-ink-800/65">Mod list file</span><div class="mt-2 flex gap-2"><input bind:value={manifest} placeholder="Choose an IEPM .yaml mod list" class="min-w-0 flex-1 rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm outline-none focus:border-moss-500 focus:ring-3 focus:ring-moss-500/10" /><button onclick={pickManifest} class="rounded-lg border border-black/12 bg-white px-4 text-sm font-semibold hover:bg-paper-50">Browse</button></div><span class="mt-1.5 block text-[11px] text-ink-800/45">The app brings its own mod registry. You only choose the list you want to install.</span></label>

						{#if error}<div class="rounded-xl border border-red-500/20 bg-red-50 px-4 py-3 text-sm leading-5 text-red-800">{error}</div>{/if}
						{#if result}<div class="rounded-xl border border-emerald-500/25 bg-emerald-50 px-4 py-3 text-sm text-emerald-900"><p class="font-semibold">Your mod experience is ready</p><p class="mt-1 break-all text-xs">{result.sealed_build}</p>{#if result.weidu_warnings.length}<button onclick={() => (showInstallWarnings = !showInstallWarnings)} class="mt-3 text-xs font-semibold text-emerald-800 underline">{showInstallWarnings ? 'Hide' : 'Show'} {result.weidu_warnings.length} WeiDU warning {result.weidu_warnings.length === 1 ? 'receipt' : 'receipts'}</button>{#if showInstallWarnings}<div class="mt-2 space-y-2 rounded-lg border border-emerald-500/20 bg-white/60 p-3 text-xs text-ink-800/75">{#each result.weidu_warnings as warning}<div><p class="font-semibold">Action {warning.action}: {warning.package}</p><pre class="mt-1 whitespace-pre-wrap font-mono text-[10px] leading-4">{warning.details}</pre><p class="mt-1 break-all text-[10px] text-ink-800/50">Saved receipt: {warning.log}</p></div>{/each}</div>{/if}{/if}</div>{/if}

						<div class="flex items-center gap-3 pt-1"><button onclick={reviewPlan} disabled={busy || !manifest.trim() || (!bgeeDirectory.trim() && !bg2eeDirectory.trim())} class="rounded-lg border border-black/12 bg-white px-5 py-2.5 text-sm font-semibold text-ink-800 shadow-sm hover:bg-paper-50 disabled:opacity-50">Review plan</button><button onclick={installExperience} disabled={busy || !preview} class="rounded-lg bg-moss-600 px-5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-moss-500 disabled:cursor-not-allowed disabled:opacity-40">{busy ? 'Working…' : 'Install experience'}</button>{#if progress}<span class="text-xs text-ink-800/55"><strong class="font-semibold capitalize">{progress.stage}:</strong> {progress.message}</span>{/if}</div>
					</section>

					<aside class="min-w-0 rounded-2xl border border-black/8 bg-white p-5 shadow-sm"><div class="flex items-center justify-between"><h3 class="text-sm font-semibold">Installation review</h3><span class="rounded-full bg-paper-100 px-2 py-1 text-[10px] font-semibold tracking-wide text-ink-800/55 uppercase">{preview ? 'Ready' : 'Not checked'}</span></div>{#if preview}<div class="mt-5 grid grid-cols-2 gap-3"><div class="rounded-xl bg-paper-100 p-3"><p class="text-2xl font-semibold">{preview.packageCount}</p><p class="mt-1 text-[11px] text-ink-800/50">packages</p></div><div class="rounded-xl bg-paper-100 p-3"><p class="text-2xl font-semibold">{preview.weiduVersion}</p><p class="mt-1 text-[11px] text-ink-800/50">WeiDU engine</p></div></div>{#if preview.warnings.length}<div class="mt-4 rounded-xl border border-amber-500/20 bg-amber-500/8 p-3"><p class="text-xs font-semibold text-amber-500">Allowed warnings</p><ul class="mt-2 space-y-1 text-[11px] leading-4 text-ink-800/65">{#each preview.warnings as warning}<li>• {warning}</li>{/each}</ul></div>{/if}<pre class="mt-4 max-h-[390px] overflow-auto whitespace-pre-wrap rounded-xl bg-ink-950 p-4 font-mono text-[10px] leading-4 text-white/70">{preview.plan}</pre>{:else}<div class="mt-16 text-center"><div class="mx-auto flex h-11 w-11 items-center justify-center rounded-full bg-paper-100 text-lg text-ink-800/40">✓</div><p class="mt-4 text-sm font-medium">Review before installing</p><p class="mx-auto mt-2 max-w-[230px] text-xs leading-5 text-ink-800/45">IEPM will prepare its verified installation engine, resolve exact releases, and show the full plan.</p></div>{/if}</aside>
				</div>
			</main>
		</div>
	</div>

	{#if settingsOpen}<div class="fixed inset-0 grid place-items-center bg-black/35 p-6"><section class="w-full max-w-lg rounded-2xl bg-paper-50 p-6 shadow-2xl"><div class="flex items-center justify-between"><h2 class="text-xl font-semibold">IEPM settings</h2><button onclick={() => (settingsOpen = false)} class="text-sm text-ink-800/55 hover:text-ink-950">Close</button></div><p class="mt-2 text-sm leading-6 text-ink-800/60">Move the IEPM library if you need more room. Existing experiences remain where they are.</p><label class="mt-5 block"><span class="text-xs font-semibold text-ink-800/65">IEPM library folder</span><div class="mt-2 flex gap-2"><input bind:value={libraryLocation} class="min-w-0 flex-1 rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm" /><button onclick={pickLibrary} class="rounded-lg border border-black/12 bg-white px-4 text-sm font-semibold">Browse</button></div></label><div class="mt-5 border-t border-black/8 pt-5"><p class="text-xs font-semibold text-ink-800/65">Advanced: local WeiDU override</p><p class="mt-1 text-xs leading-5 text-ink-800/50">Only use this if Windows Security blocks IEPM’s SHA-verified download. IEPM checks that the selected executable reports WeiDU {startup.defaultWeiduVersion}; a loose file cannot receive the downloaded archive’s integrity badge.</p><div class="mt-3 flex gap-2"><input bind:value={weiduOverride} placeholder="Usually leave blank" class="min-w-0 flex-1 rounded-lg border border-black/10 bg-white px-3 py-2.5 text-sm" /><button onclick={pickWeiduOverride} class="rounded-lg border border-black/12 bg-white px-4 text-sm font-semibold">Browse</button></div><div class="mt-3 flex justify-end"><button onclick={saveWeiduOverride} disabled={busy} class="rounded-lg border border-black/12 bg-white px-4 py-2 text-sm font-semibold text-ink-800 disabled:opacity-40">Save WeiDU override</button></div></div><div class="mt-6 flex justify-end"><button onclick={saveLibrary} disabled={busy || !libraryLocation.trim()} class="rounded-lg bg-moss-600 px-5 py-2.5 text-sm font-semibold text-white disabled:opacity-40">Save location</button></div></section></div>{/if}
{/if}
