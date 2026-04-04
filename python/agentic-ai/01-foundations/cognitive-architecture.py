import json
from pathlib import Path
import ollama

# -----------------------------
# Hidden game world
# -----------------------------
SECRET_NUMBER = 12


# -----------------------------
# Tools
# -----------------------------
def greater_than(n: int) -> str:
    return f"Is the secret number greater than {n}? {'yes' if SECRET_NUMBER > n else 'no'}"


def is_divisible_by(n: int) -> str:
    if n == 0:
        return "Error: division by zero is not allowed."
    return f"Is the secret number divisible by {n}? {'yes' if SECRET_NUMBER % n == 0 else 'no'}"


def guess_number(n: int) -> str:
    if SECRET_NUMBER == n:
        return f"Correct! The secret number is {n}."
    return f"Incorrect guess: {n} is not the secret number."


# -----------------------------
# Agent loop (Ollama version)
# -----------------------------
def run_hidden_number_agent(goal: str, max_iterations: int = 8) -> str:
    model = "qwen3.5:9b"

    system_prompt = """
You are an autonomous puzzle-solving agent.

A secret integer exists between 1 and 20.
Your job is to discover it using tools strategically.

Rules:
- Do NOT guess too early
- Use previous observations
- Think step by step
- Call ONE tool per step

Available tools:
1. greater_than(n)
2. is_divisible_by(n)
3. guess_number(n)

To call a tool, respond ONLY in JSON:

{
  "tool": "tool_name",
  "arguments": { "n": number }
}

After receiving tool results, continue reasoning.
"""

    history = []

    messages = [
        {"role": "system", "content": system_prompt},
    ]

    current_prompt = (
        f"Goal: {goal}\n"
        "The secret number is between 1 and 20.\n"
        "Start by choosing the best first tool call."
    )

    for iteration in range(1, max_iterations + 1):
        print(f"\n=== Iteration {iteration} ===")
        print("Prompt to agent:\n", current_prompt)

        messages.append({"role": "user", "content": current_prompt})

        response = ollama.chat(
            model=model,
            messages=messages,
        )

        content = response["message"]["content"]
        print("\nAgent response:\n", content)

        # -----------------------------
        # Try parsing tool call
        # -----------------------------
        try:
            parsed = json.loads(content)

            tool = parsed.get("tool")
            n = parsed.get("arguments", {}).get("n")

            if tool and n is not None:
                if tool == "greater_than":
                    result = greater_than(int(n))
                elif tool == "is_divisible_by":
                    result = is_divisible_by(int(n))
                elif tool == "guess_number":
                    result = guess_number(int(n))
                else:
                    result = "Error: unknown tool"

                print("\n📦 Tool result:", result)

                history.append(f"Iteration {iteration}: {result}")

                # Add assistant + tool result
                messages.append({"role": "assistant", "content": content})
                messages.append({
                    "role": "user",
                    "content": f"Tool result: {result}"
                })

                # Stop if solved
                if "Correct! The secret number is" in result:
                    return result

                continue

        except json.JSONDecodeError:
            pass

        # If model didn't call tool properly → treat as final answer
        history.append(f"Iteration {iteration}: {content}")

        if "Correct! The secret number is" in content:
            return content

        # Build next prompt
        observation_block = "\n".join(history)

        current_prompt = (
            f"Goal: {goal}\n"
            f"Iteration: {iteration}/{max_iterations}\n"
            "Observations so far:\n"
            f"{observation_block}\n\n"
            "Choose the best next tool call.\n"
            "If confident, use guess_number."
        )

    return "Maximum iterations reached before solving."


# -----------------------------
# Main
# -----------------------------
if __name__ == "__main__":
    goal = "Find the secret number."
    answer = run_hidden_number_agent(goal)

    print("\nFinal answer:\n")
    print(answer)