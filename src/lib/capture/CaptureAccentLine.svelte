<div class="accent-line" role="presentation"></div>

<style>
  .accent-line {
    height: 2px;
    background: linear-gradient(
      90deg,
      color-mix(in srgb, var(--accent-color, #8b5cf6) 70%, transparent),
      color-mix(in srgb, var(--accent-color, #8b5cf6) 35%, transparent),
      color-mix(in srgb, var(--accent-color, #8b5cf6) 70%, transparent)
    );
    background-size: 200% 100%;
    animation: shimmer 3s linear infinite;
    transition:
      filter 0.12s ease,
      opacity 0.12s ease,
      transform 0.12s ease;
  }

  @keyframes shimmer {
    0% {
      background-position: 200% 0;
    }
    100% {
      background-position: -200% 0;
    }
  }
</style>
