from __future__ import annotations


class CheapLMSError(Exception):
    """Base exception for all cheap-llm errors."""


class ConfigError(CheapLMSError):
    """Raised when configuration is invalid or incomplete."""


class RouterError(CheapLMSError):
    """Raised when all providers fail to complete a request."""


class LedgerCapExceeded(CheapLMSError):
    """Raised when the monthly cost cap has been exceeded."""
