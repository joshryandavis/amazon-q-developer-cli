#!/usr/bin/env python3
"""
Simple example of using Q Developer with LangChain.

Make sure you're logged in first: q login
Run with: uv run examples/simple.py
"""

import sys
sys.path.insert(0, "src")

from q_developer_langchain import QDeveloperLLM


def main():
    print("Amazon Q Developer - LangChain Python Example\n")

    # Create Q Developer LLM
    print("Initializing Q Developer...")
    llm = QDeveloperLLM()

    # Simple question
    prompt = "Write a hello world program in Python"
    print(f"\nPrompt: {prompt}\n")

    print("Generating response...\n")
    try:
        response = llm.invoke(prompt)
        print("Response:")
        print(response)
        print("\n✓ Success!")
    except FileNotFoundError:
        print("❌ Q CLI not found. Please install from https://aws.amazon.com/q/")
        return 1
    except ValueError as e:
        if "not logged in" in str(e):
            print("❌ Not logged in. Please run: q login")
        else:
            print(f"❌ Error: {e}")
        return 1
    except Exception as e:
        print(f"❌ Unexpected error: {e}")
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
