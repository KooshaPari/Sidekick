from __future__ import annotations

import pytest

from cheap_llm_mcp.config import ProviderConfig, load


@pytest.mark.requirement("FR-LLM-041")
def test_provider_config_api_key_short(monkeypatch):
    monkeypatch.setenv("TEST_KEY", "short")
    cfg = ProviderConfig(
        name="test",
        base_url="http://localhost",
        api_key_env="TEST_KEY",
        default_model="m",
    )
    with pytest.raises(RuntimeError, match="too short"):
        _ = cfg.api_key


@pytest.mark.requirement("FR-LLM-042")
def test_provider_config_api_key_valid(monkeypatch):
    monkeypatch.setenv("TEST_KEY", "sk-valid-key-12345")
    cfg = ProviderConfig(
        name="test",
        base_url="http://localhost",
        api_key_env="TEST_KEY",
        default_model="m",
    )
    assert cfg.api_key == "sk-valid-key-12345"


@pytest.mark.requirement("FR-LLM-011")
def test_load_defaults_when_no_file(tmp_path, monkeypatch):
    monkeypatch.chdir(tmp_path)
    cfg = load()
    assert "minimax" in cfg.providers
    assert cfg.providers["minimax"].default_model == "MiniMax-M2.7-highspeed"


@pytest.mark.requirement("FR-LLM-012")
def test_load_missing_required_keys(tmp_path):
    p = tmp_path / "bad.toml"
    p.write_text('[providers.broken]\nbase_url = "x"\n')
    with pytest.raises(ValueError, match="missing required keys"):
        load(p)


@pytest.mark.requirement("FR-LLM-013")
def test_load_unknown_keys_rejected(tmp_path):
    p = tmp_path / "bad.toml"
    p.write_text(
        "[providers.minimax]\n"
        'base_url = "x"\n'
        'api_key_env = "X"\n'
        'default_model = "m"\n'
        'totally_unknown_key = "y"\n'
    )
    with pytest.raises(ValueError, match="unexpected keyword"):
        load(p)


@pytest.mark.requirement("FR-LLM-014")
def test_load_providers_not_table(tmp_path):
    p = tmp_path / "bad.toml"
    p.write_text('providers = "not a table"\n')
    with pytest.raises(ValueError, match="must be a table"):
        load(p)


@pytest.mark.requirement("FR-LLM-010")
def test_load_valid_file(tmp_path):
    p = tmp_path / "good.toml"
    p.write_text(
        'default_provider = "minimax"\n'
        "monthly_cost_cap_usd = 25.0\n"
        "[providers.minimax]\n"
        'base_url = "https://api.minimax.io/v1"\n'
        'api_key_env = "MINIMAX_API_KEY"\n'
        'default_model = "MiniMax-M2.7"\n'
    )
    cfg = load(p)
    assert cfg.default_provider == "minimax"
    assert cfg.monthly_cost_cap_usd == 25.0
    assert cfg.providers["minimax"].default_model == "MiniMax-M2.7"
