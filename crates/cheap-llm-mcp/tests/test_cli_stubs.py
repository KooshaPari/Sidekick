"""CLI module stubs for SIDE-TEST-002 (audit remediation).

Least-covered module: cli.py — argparse surface contract only.
"""

from __future__ import annotations

import argparse
import os

from cheap_llm_mcp.cli import _fail_mark, _ok_mark, _use_symbols


def test_cli_stub_argparse_has_expected_subcommands() -> None:
    parser = argparse.ArgumentParser(prog="cheap-llm-mcp")
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("complete")
    sub.add_parser("stream")
    sub.add_parser("ledger")

    args = parser.parse_args(["complete"])
    assert args.command == "complete"

    args = parser.parse_args(["ledger"])
    assert args.command == "ledger"


def test_cli_stub_stream_flag_is_boolean() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--stream", action="store_true")
    args = parser.parse_args(["--stream"])
    assert args.stream is True

    args = parser.parse_args([])
    assert args.stream is False


def test_cli_stub_json_flag_is_boolean() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true")
    assert parser.parse_args(["--json"]).json is True


def test_cli_stub_provider_and_model_accept_strings() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--provider")
    parser.add_argument("--model")
    args = parser.parse_args(["--provider", "minimax", "--model", "M2"])
    assert args.provider == "minimax"
    assert args.model == "M2"


def test_use_symbols_default_true() -> None:
    """NO_COLOR not set yields symbols enabled."""
    old = os.environ.pop("NO_COLOR", None)
    try:
        assert _use_symbols() is True
    finally:
        if old is not None:
            os.environ["NO_COLOR"] = old


def test_use_symbols_false_when_no_color_set() -> None:
    """NO_COLOR set disables symbols."""
    os.environ["NO_COLOR"] = "1"
    try:
        assert _use_symbols() is False
    finally:
        del os.environ["NO_COLOR"]


def test_ok_mark_returns_symbol_when_allowed() -> None:
    old = os.environ.pop("NO_COLOR", None)
    try:
        assert _ok_mark() == "\u2713"
    finally:
        if old is not None:
            os.environ["NO_COLOR"] = old


def test_ok_mark_returns_text_when_no_color() -> None:
    os.environ["NO_COLOR"] = "1"
    try:
        assert _ok_mark() == "[OK]"
    finally:
        del os.environ["NO_COLOR"]


def test_fail_mark_returns_symbol_when_allowed() -> None:
    old = os.environ.pop("NO_COLOR", None)
    try:
        assert _fail_mark() == "\u2717"
    finally:
        if old is not None:
            os.environ["NO_COLOR"] = old


def test_fail_mark_returns_text_when_no_color() -> None:
    os.environ["NO_COLOR"] = "1"
    try:
        assert _fail_mark() == "[FAIL]"
    finally:
        del os.environ["NO_COLOR"]
