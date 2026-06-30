from __future__ import annotations

import json
import threading
from dataclasses import asdict, dataclass, field
from datetime import UTC, date, datetime
from pathlib import Path

# Rough USD-per-1M-token pricing snapshot (input, output). Update as needed.
# Sourced from each provider's public pricing pages; treat as ceiling.
PRICING: dict[str, tuple[float, float]] = {
    "MiniMax-M2": (0.30, 1.20),
    "MiniMax-M2.5": (0.30, 1.20),
    "MiniMax-M2.7": (0.30, 1.20),
    "kimi-k2-turbo-preview": (0.60, 2.50),
    "accounts/fireworks/models/kimi-k2-instruct": (1.00, 3.00),
    "_default": (1.00, 3.00),
}


def estimate_cost_usd(model: str, input_tokens: int, output_tokens: int) -> float:
    in_rate, out_rate = PRICING.get(model, PRICING["_default"])
    return (input_tokens * in_rate + output_tokens * out_rate) / 1_000_000


class LedgerCapError(Exception):
    """Raised when a monthly cost cap is exceeded."""


@dataclass
class LedgerEntry:
    ts: str
    provider: str
    model: str
    input_tokens: int
    output_tokens: int
    cost_usd: float


@dataclass
class MonthAggregate:
    month: str  # YYYY-MM
    total_usd: float = 0.0
    calls: int = 0
    by_provider: dict[str, float] = field(default_factory=dict)


class Ledger:
    """Append-only JSONL ledger with monthly cap enforcement.

    Thread-safe: both *record()* and *check_cap()* serialise through an
    internal lock so that the cap check and append happen atomically.
    """

    def __init__(self, path: Path, cap_usd: float | None = None):
        self.path = path
        self.cap_usd = cap_usd
        self._lock = threading.Lock()
        self.path.parent.mkdir(parents=True, exist_ok=True)

    def record(
        self, provider: str, model: str, input_tokens: int, output_tokens: int
    ) -> LedgerEntry:
        entry = LedgerEntry(
            ts=datetime.now(UTC).isoformat(timespec="seconds"),
            provider=provider,
            model=model,
            input_tokens=input_tokens,
            output_tokens=output_tokens,
            cost_usd=round(estimate_cost_usd(model, input_tokens, output_tokens), 6),
        )
        with self._lock:
            # Atomic cap check under the same lock as the write.
            # This prevents TOCTOU races where concurrent requests both
            # pass check_cap() before either calls record().
            if self.cap_usd is not None:
                agg = self._read_month_total_unlocked()
                if agg.total_usd + entry.cost_usd > self.cap_usd:
                    raise LedgerCapError(
                        f"monthly cap ${self.cap_usd:.2f} would be exceeded "
                        f"(${agg.total_usd:.4f} spent + ${entry.cost_usd:.6f} new)"
                    )
            with self.path.open("a") as f:
                f.write(json.dumps(asdict(entry)) + "\n")
        return entry

    def month_total(self, month: str | None = None) -> MonthAggregate:
        """Return aggregate for *month* (YYYY-MM). Thread-safe."""
        with self._lock:
            return self._read_month_total_unlocked(month)

    def _read_month_total_unlocked(self, month: str | None = None) -> MonthAggregate:
        """Read month total without acquiring the lock (caller must hold _lock)."""
        if month is None:
            month = date.today().strftime("%Y-%m")
        agg = MonthAggregate(month=month)
        if not self.path.exists():
            return agg
        with self.path.open() as f:
            for line in f:
                try:
                    e = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if not e.get("ts", "").startswith(month):
                    continue
                agg.total_usd += e["cost_usd"]
                agg.calls += 1
                prov = e["provider"]
                agg.by_provider[prov] = agg.by_provider.get(prov, 0.0) + e["cost_usd"]
        agg.total_usd = round(agg.total_usd, 4)
        return agg

    def check_cap(self) -> None:
        """Raise LedgerCapError if monthly cap is exceeded. Thread-safe."""
        if self.cap_usd is None:
            return
        with self._lock:
            agg = self._read_month_total_unlocked()
        if agg.total_usd >= self.cap_usd:
            raise LedgerCapError(
                f"monthly cap ${self.cap_usd:.2f} reached "
                f"(spent ${agg.total_usd:.2f} across {agg.calls} calls)"
            )
