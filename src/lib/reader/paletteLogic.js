export function filterPaletteNotes(vaultNotes = [], query = "") {
  const normalizedQuery = query.trim().toLowerCase();
  if (!normalizedQuery) return vaultNotes;

  return vaultNotes.filter(
    (note) =>
      note.name.toLowerCase().includes(normalizedQuery) ||
      note.relative_path.toLowerCase().includes(normalizedQuery),
  );
}

export function getOpenVaultNoteIntent(note, tabs = []) {
  const notePath = note.relative_path ?? note.path;
  const existingIndex = tabs.findIndex((tab) => tab.path === notePath);
  if (existingIndex >= 0) {
    return { action: "activateExisting", index: existingIndex };
  }

  return { action: "newTab", note };
}
