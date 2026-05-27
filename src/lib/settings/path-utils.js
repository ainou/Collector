export function normalizeComparablePath(path = "") {
    return path.replace(/\\/g, "/").replace(/\/+$/, "");
}
