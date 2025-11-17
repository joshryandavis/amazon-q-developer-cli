#!/usr/bin/env python3
"""
Example using Q Developer with LangChain chains.

Run with: uv run examples/chain.py
"""

import sys
sys.path.insert(0, "src")

from langchain.chains import LLMChain
from langchain.prompts import PromptTemplate
from q_developer_langchain import QDeveloperLLM


def main():
    print("Amazon Q Developer - LangChain Chains Example\n")

    # Create prompt template
    prompt = PromptTemplate(
        input_variables=["topic", "level"],
        template="Explain {topic} to someone at {level} level with a practical example"
    )

    # Create Q Developer LLM
    llm = QDeveloperLLM()

    # Create chain
    chain = LLMChain(llm=llm, prompt=prompt)

    # Run chain with different parameters
    topics = [
        {"topic": "recursion in programming", "level": "beginner"},
        {"topic": "async/await in Python", "level": "intermediate"},
    ]

    for params in topics:
        print(f"\n{'='*60}")
        print(f"Topic: {params['topic']} (Level: {params['level']})")
        print(f"{'='*60}\n")

        try:
            result = chain.run(**params)
            print(result)
        except Exception as e:
            print(f"Error: {e}")
            return 1

    print("\n✓ Success!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
