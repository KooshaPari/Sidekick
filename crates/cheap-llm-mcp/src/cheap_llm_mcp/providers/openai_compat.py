from __future__ import annotations

import json
from collections.abc import AsyncIterator

import httpx

from ..config import ProviderConfig
from ..retry import with_retry
from .base import Completion, Message


def _sanitize_provider_error(exc: Exception) -> str:
    """Return a safe error label without leaking credentials.

    Strips any lines from the exception message that look like they
    contain API keys, tokens, or secrets.
    """
    type_name = type(exc).__name__
    msg = str(exc)
    # HTTP responses may contain upstream error bodies with credentials
    if hasattr(exc, "response"):
        try:
            status = exc.response.status_code  # type: ignore[union-attr]
            return f"HTTP {status} ({type_name})"
        except Exception:
            pass
    if len(msg) > 200:
        msg = msg[:197] + "..."
    lines = msg.split("\n")
    clean: list[str] = []
    for line in lines:
        low = line.lower()
        if any(kw in low for kw in ("api_key", "api-key", "apikey", "bearer ", "secret", "token")):
            clean.append(f"[{type_name}: redacted]")
        else:
            clean.append(line)
    return f"{type_name}: {'; '.join(clean)}"


class OpenAICompatProvider:
    """OpenAI-compatible chat-completions client shared by Minimax, Kimi, Fireworks."""

    def __init__(self, cfg: ProviderConfig):
        self.cfg = cfg
        self.name = cfg.name
        self._client = httpx.AsyncClient(
            base_url=cfg.base_url,
            timeout=cfg.timeout_s,
            headers={
                "Authorization": f"Bearer {cfg.api_key}",
                "Content-Type": "application/json",
            },
        )

    def _resolve_model(self, model: str | None, variant: str | None = None) -> str:
        if model:
            return model
        if variant and variant in self.cfg.variants:
            return self.cfg.variants[variant]
        return self.cfg.default_model

    async def complete(
        self,
        messages: list[Message],
        *,
        model: str | None = None,
        variant: str | None = None,
        max_tokens: int = 4096,
        temperature: float = 0.2,
    ) -> Completion:
        resolved = self._resolve_model(model, variant)
        payload = {
            "model": resolved,
            "messages": [{"role": m.role, "content": m.content} for m in messages],
            "max_tokens": max_tokens,
            "temperature": temperature,
        }

        async def _post():
            resp = await self._client.post("/chat/completions", json=payload)
            resp.raise_for_status()
            return resp

        r = await with_retry(_post)
        data = r.json()
        choice = data["choices"][0]
        usage = data.get("usage", {})
        return Completion(
            text=choice["message"]["content"] or "",
            model=data.get("model", resolved),
            provider=self.name,
            input_tokens=usage.get("prompt_tokens", 0),
            output_tokens=usage.get("completion_tokens", 0),
            finish_reason=choice.get("finish_reason"),
        )

    async def stream(
        self,
        messages: list[Message],
        *,
        model: str | None = None,
        variant: str | None = None,
        max_tokens: int = 4096,
        temperature: float = 0.2,
    ) -> AsyncIterator[str]:
        resolved = self._resolve_model(model, variant)
        payload = {
            "model": resolved,
            "messages": [{"role": m.role, "content": m.content} for m in messages],
            "max_tokens": max_tokens,
            "temperature": temperature,
            "stream": True,
        }
        async with self._client.stream("POST", "/chat/completions", json=payload) as r:
            r.raise_for_status()
            async for line in r.aiter_lines():
                if not line or not line.startswith("data: "):
                    continue
                chunk = line[6:]
                if chunk.strip() == "[DONE]":
                    break
                try:
                    obj = json.loads(chunk)
                except json.JSONDecodeError:
                    continue
                delta = obj.get("choices", [{}])[0].get("delta", {}).get("content")
                if delta:
                    yield delta

    async def list_models(self) -> list[dict]:
        r = await self._client.get("/models")
        r.raise_for_status()
        return r.json().get("data", [])

    async def health(self) -> dict:
        """Lightweight health probe. Returns {status, latency_ms, error?}."""
        import time

        t0 = time.perf_counter()
        try:
            r = await self._client.get("/models", timeout=5.0)
            r.raise_for_status()
            return {
                "status": "ok",
                "latency_ms": round((time.perf_counter() - t0) * 1000, 1),
                "provider": self.name,
            }
        except Exception as e:
            safe = _sanitize_provider_error(e)
            return {
                "status": "error",
                "latency_ms": round((time.perf_counter() - t0) * 1000, 1),
                "provider": self.name,
                "error": safe,
            }

    async def aclose(self) -> None:
        await self._client.aclose()
