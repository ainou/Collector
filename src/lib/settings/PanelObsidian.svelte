<script>
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";

    export let settings;
    export let showStatus;

    let isIndexing = false;
    let indexStatus = "";

    $: void showStatus;

    async function pickVaultPath() {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: settings.vault_path || undefined,
        });
        if (selected) {
            settings.vault_path = selected;
            indexStatus = "";
        }
    }

    async function handleReindex() {
        isIndexing = true;
        indexStatus = "";

        try {
            const count = await invoke("reindex_vault");
            indexStatus = `✓ Index built (${count} files)`;
        } catch (e) {
            indexStatus = `Failed to index vault: ${e}`;
        } finally {
            isIndexing = false;
        }
    }
</script>

<div class="settings-panel">
    <section>
        <h2>Vault</h2>
        <div class="field">
            <label for="vault_name">Vault Name</label>
            <input
                type="text"
                id="vault_name"
                bind:value={settings.vault_name}
                placeholder="Vault"
            />
            <small>Name of your Obsidian vault</small>
        </div>
        <div class="field">
            <label for="vault_path">Vault Path</label>
            <div class="path-picker">
                <input
                    type="text"
                    id="vault_path"
                    bind:value={settings.vault_path}
                    placeholder="/Users/username/Vault"
                    readonly
                />
                <button class="secondary" on:click={pickVaultPath}
                    >Choose...</button
                >
            </div>
            <div class="reindex-row">
                <button
                    class="secondary"
                    type="button"
                    on:click={handleReindex}
                    disabled={isIndexing}
                >
                    {isIndexing ? "Indexing..." : "↻ Re-index Vault"}
                </button>
                {#if indexStatus}
                    <span class="index-status">{indexStatus}</span>
                {/if}
            </div>
            <small>Full path to your Obsidian vault</small>
            <small>
                Re-index if images or notes added outside Collector are not
                appearing correctly.
            </small>
        </div>
    </section>

    <section>
        <h2>Daily Notes</h2>
        <div class="field">
            <label for="daily_note_folder">Daily Note Path</label>
            <input
                type="text"
                id="daily_note_folder"
                bind:value={settings.daily_note_folder}
                placeholder="Journal/Notes/"
            />
            <small>Relative path in vault for daily notes</small>
        </div>
        <div class="field">
            <label for="daily_note_format">Daily Note Format</label>
            <input
                type="text"
                id="daily_note_format"
                bind:value={settings.daily_note_format}
                placeholder="YYYY-MM-DD"
            />
            <small>
                Filename format (e.g. YYYY-MM-DD). Supports: YYYY, MM, DD
            </small>
        </div>
    </section>

    <section>
        <h2>Entry Header</h2>
        <div class="field">
            <label for="entry_header">Entry Header</label>
            <input
                type="text"
                id="entry_header"
                bind:value={settings.entry_header}
                placeholder="#### HH:mm"
            />
            <small>
                Supported: HH (24h), hh / h (12h), mm, ss, a / A (am/pm) · e.g.
                #### HH:mm or #### h:mm a
            </small>
        </div>
    </section>

    <section>
        <h2>Daily Note Capture</h2>
        <div class="field">
            <label for="daily_note_target_heading">Target Heading</label>
            <input
                type="text"
                id="daily_note_target_heading"
                bind:value={settings.daily_note_target_heading}
                placeholder="### Note"
            />
            <small>Leave empty to append to the end of the daily note.</small>
        </div>

        <div class="field">
            <label for="daily_note_insert_position">Insert Position</label>
            <select
                id="daily_note_insert_position"
                bind:value={settings.daily_note_insert_position}
            >
                <option value="bottom">Bottom of section</option>
                <option value="top">Top of section</option>
            </select>
        </div>

        <div class="field">
            <label class="checkbox" for="daily_note_create_heading_if_missing">
                <input
                    type="checkbox"
                    id="daily_note_create_heading_if_missing"
                    bind:checked={settings.daily_note_create_heading_if_missing}
                />
                <span>Create heading if it does not exist</span>
            </label>
        </div>

        <div class="field">
            <label class="checkbox" for="daily_note_create_if_missing">
                <input
                    type="checkbox"
                    id="daily_note_create_if_missing"
                    bind:checked={settings.daily_note_create_if_missing}
                />
                <span>Create daily note if it does not exist</span>
            </label>
            {#if settings.daily_note_create_if_missing}
                <small>
                    Requires the Obsidian Advanced URI plugin. Collector opens
                    Obsidian to create the daily note, then appends the capture
                    to the configured section.
                </small>
            {/if}
        </div>

        <div class="field">
            <label for="daily_note_create_timeout_ms"
                >Wait timeout (ms)</label
            >
            <input
                type="number"
                id="daily_note_create_timeout_ms"
                min="1000"
                max="60000"
                step="100"
                bind:value={settings.daily_note_create_timeout_ms}
            />
            <small
                >How long Collector waits for Obsidian to create the daily note
                before giving up (1000–60000 ms).</small
            >
        </div>
    </section>
</div>

<style>
    .reindex-row {
        display: flex;
        gap: 8px;
        align-items: center;
        margin-top: 8px;
    }

    .index-status {
        color: #666;
        font-size: 11px;
    }
</style>
