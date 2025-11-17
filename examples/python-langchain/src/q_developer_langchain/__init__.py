"""
LangChain integration for Amazon Q Developer.

This package provides a LangChain-compatible LLM that uses Amazon Q Developer
via the Q CLI. Make sure you're logged in with `q login` before using.
"""

from .llm import QDeveloperLLM

__all__ = ["QDeveloperLLM"]
__version__ = "0.1.0"
