#!/usr/bin/env python3
"""
Example demonstrating conversation with Q Developer.

Note: Each invoke is currently independent. Conversation context
management is planned for a future version.

Run with: uv run examples/conversation.py
"""

import sys
sys.path.insert(0, "src")

from q_developer_langchain import QDeveloperLLM


def main():
    print("Amazon Q Developer - Conversation Example\n")

    # Create Q Developer LLM
    llm = QDeveloperLLM()

    # Series of questions
    questions = [
        "What is Rust programming language?",
        "What are its main advantages?",
        "Show me a simple 'Hello World' example in Rust",
    ]

    for i, question in enumerate(questions, 1):
        print(f"\n{'='*60}")
        print(f"Question {i}: {question}")
        print(f"{'='*60}\n")

        try:
            response = llm.invoke(question)
            print(response)
        except Exception as e:
            print(f"Error: {e}")
            return 1

    print("\n✓ Success!")
    print("\nNote: Currently each question is independent.")
    print("Conversation context management coming soon!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
