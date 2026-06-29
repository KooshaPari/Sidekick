"""Tests for thread-safe singleton initialization in server.py."""

from __future__ import annotations

import threading
from unittest.mock import patch

import pytest

from cheap_llm_mcp.server import _ledger_get, _router_get


def test_router_get_returns_same_instance():
    """_router_get() should return the same Router instance on repeated calls."""
    r1 = _router_get()
    r2 = _router_get()
    assert r1 is r2


def test_ledger_get_returns_same_instance():
    """_ledger_get() should return the same Ledger instance on repeated calls."""
    l1 = _ledger_get()
    l2 = _ledger_get()
    assert l1 is l2


def test_concurrent_router_init_returns_same_instance():
    """Simulate concurrent calls to _router_get to verify lock safety."""
    results: list = []
    errors: list = []
    lock = threading.Lock()

    def call_router():
        try:
            r = _router_get()
            with lock:
                results.append(id(r))
        except Exception as e:
            with lock:
                errors.append(e)

    threads = [threading.Thread(target=call_router) for _ in range(10)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()

    assert len(errors) == 0
    # All threads should get the same instance
    assert len(set(results)) == 1


def test_concurrent_ledger_init_returns_same_instance():
    """Simulate concurrent calls to _ledger_get to verify lock safety."""
    results: list = []
    errors: list = []
    lock = threading.Lock()

    def call_ledger():
        try:
            l = _ledger_get()
            with lock:
                results.append(id(l))
        except Exception as e:
            with lock:
                errors.append(e)

    threads = [threading.Thread(target=call_ledger) for _ in range(10)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()

    assert len(errors) == 0
    # All threads should get the same instance
    assert len(set(results)) == 1


def test_router_get_with_locks_prevents_race():
    """Verify that the lock prevents duplicate initialization under contention."""
    from cheap_llm_mcp.router import Router

    # Reset state, then verify the lock + double-check pattern works
    import cheap_llm_mcp.server as srv

    srv._router = None
    srv._ledger = None

    init_count = {"n": 0}
    original_router_init = Router.__init__

    def counting_init(self, cfg, cache_ttl=0.0):
        init_count["n"] += 1
        return original_router_init(self, cfg, cache_ttl=cache_ttl)

    with patch.object(Router, "__init__", counting_init):
        threads = [threading.Thread(target=_router_get) for _ in range(5)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

    # Despite 5 concurrent calls, __init__ should only be called once
    assert init_count["n"] == 1
