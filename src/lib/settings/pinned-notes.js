function getFilenameFromPath(path) {
    return (
        path.replace(/\\/g, "/").split("/").pop()?.replace(/\.md$/i, "") || path
    );
}

export function normalizePinnedNotes(pinnedNotes = []) {
    return pinnedNotes
        .map((entry) => {
            if (typeof entry === "string") {
                return {
                    path: entry,
                    label: getFilenameFromPath(entry),
                    icon: "",
                };
            }

            return {
                path: entry?.path ?? "",
                label: entry?.label ?? "",
                icon: entry?.icon ?? "",
            };
        })
        .filter((entry) => entry.path.trim() !== "");
}
