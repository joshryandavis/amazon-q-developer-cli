"""
LangChain LLM implementation for Amazon Q Developer.

Uses the Q CLI for authentication and API access.
"""

import json
import subprocess
from typing import Any, Dict, List, Optional

from langchain_core.callbacks import CallbackManagerForLLMRun
from langchain_core.language_models.llms import LLM
from langchain_core.outputs import Generation, LLMResult


class QDeveloperLLM(LLM):
    """
    LangChain LLM that uses Amazon Q Developer via the Q CLI.

    This LLM uses the existing Q CLI authentication (via `q login`),
    so no additional API keys or configuration is needed.

    Example:
        ```python
        from q_developer_langchain import QDeveloperLLM

        llm = QDeveloperLLM()
        response = llm.invoke("Write a hello world in Python")
        print(response)
        ```

    Attributes:
        model_id: Optional model ID to use (uses default if None)
        temperature: Sampling temperature (0.0 to 1.0)
        max_tokens: Maximum tokens to generate
        q_cli_path: Path to q CLI executable (default: "q")
    """

    model_id: Optional[str] = None
    temperature: Optional[float] = None
    max_tokens: Optional[int] = None
    q_cli_path: str = "q"
    conversation_id: Optional[str] = None

    @property
    def _llm_type(self) -> str:
        """Return identifier for this LLM."""
        return "amazon-q-developer"

    def _call(
        self,
        prompt: str,
        stop: Optional[List[str]] = None,
        run_manager: Optional[CallbackManagerForLLMRun] = None,
        **kwargs: Any,
    ) -> str:
        """
        Run the LLM on the given prompt.

        Args:
            prompt: The prompt to generate from
            stop: Stop words to use when generating
            run_manager: Callback manager for the run
            **kwargs: Additional arguments

        Returns:
            The generated text
        """
        # Build command to invoke Q CLI
        # We'll use a JSON format for communication
        cmd = [
            self.q_cli_path,
            "chat",
            "--no-tui",  # Disable TUI mode
            prompt,
        ]

        # Add optional parameters
        if self.model_id:
            cmd.extend(["--model", self.model_id])

        try:
            # Run Q CLI command
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                check=True,
                timeout=300,  # 5 minute timeout
            )

            # Parse response
            response_text = result.stdout.strip()

            # Extract metadata if available (conversation ID, etc.)
            # For now, we just return the text

            return response_text

        except subprocess.CalledProcessError as e:
            error_msg = f"Q CLI error: {e.stderr}"
            if "not logged in" in e.stderr.lower():
                error_msg += "\n\nPlease run: q login"
            raise ValueError(error_msg) from e
        except subprocess.TimeoutExpired as e:
            raise TimeoutError("Q CLI request timed out after 5 minutes") from e
        except FileNotFoundError as e:
            raise FileNotFoundError(
                f"Q CLI not found at '{self.q_cli_path}'. "
                "Please install it from: https://aws.amazon.com/q/"
            ) from e

    def _generate(
        self,
        prompts: List[str],
        stop: Optional[List[str]] = None,
        run_manager: Optional[CallbackManagerForLLMRun] = None,
        **kwargs: Any,
    ) -> LLMResult:
        """
        Generate responses for multiple prompts.

        Args:
            prompts: List of prompts to generate from
            stop: Stop words to use when generating
            run_manager: Callback manager for the run
            **kwargs: Additional arguments

        Returns:
            LLM result with generations
        """
        generations = []
        for prompt in prompts:
            text = self._call(prompt, stop=stop, run_manager=run_manager, **kwargs)
            generations.append([Generation(text=text)])

        return LLMResult(generations=generations)

    @property
    def _identifying_params(self) -> Dict[str, Any]:
        """Get the identifying parameters."""
        return {
            "model_id": self.model_id,
            "temperature": self.temperature,
            "max_tokens": self.max_tokens,
            "q_cli_path": self.q_cli_path,
        }
