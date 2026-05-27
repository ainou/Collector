export function normalizeDelayValue(value, fallback = 1000) {
    const parsed = Number(value);

    if (!Number.isFinite(parsed)) {
        return fallback;
    }

    return Math.min(10000, Math.max(50, Math.round(parsed)));
}
