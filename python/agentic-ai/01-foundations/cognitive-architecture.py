import os
from pathlib import Path
from dotenv import load_dotenv
from google import genai
from google.genai import types

# -----------------------------
# Load environment variables
# -----------------------------
env_path = Path(__file__).parent.parent.parent / ".env"
load_dotenv(dotenv_path=env_path)

api_key = os.getenv("GEMINI_API_KEY")
if not api_key:
    raise ValueError("GEMINI_API_KEY not found in environment variables.")

client = genai.Client(api_key=api_key)

# -----------------------------
# Hidden game world
# -----------------------------
SECRET_NUMBER = 12


# -----------------------------
# Tools
# -----------------------------
def greater_than(n: int) -> str:
    """Return whether the secret number is greater than n."""
    return f"Is the secret number greater than {n}? {'yes' if SECRET_NUMBER > n else 'no'}"


def is_divisible_by(n: int) -> str:
    """Return whether the secret number is divisible by n."""
    if n == 0:
        return "Error: division by zero is not allowed."
    return f"Is the secret number divisible by {n}? {'yes' if SECRET_NUMBER % n == 0 else 'no'}"


def guess_number(n: int) -> str:
    """Make a final guess for the secret number."""
    if SECRET_NUMBER == n:
        return f"Correct! The secret number is {n}."
    return f"Incorrect guess: {n} is not the secret number."


# -----------------------------
# Agent loop
# -----------------------------
def run_hidden_number_agent(goal: str, max_iterations: int = 8) -> str:
    chat = client.chats.create(
        model="gemini-2.5-flash",
        config=types.GenerateContentConfig(
            tools=[greater_than, is_divisible_by, guess_number],
            temperature=0.1,
            system_instruction=(
                "You are an autonomous puzzle-solving agent.\n"
                "A secret integer exists between 1 and 20.\n"
                "Your job is to discover it by calling tools strategically.\n"
                "Do not guess too early.\n"
                "Use previous observations to narrow the possibilities.\n"
                "When you are confident, call guess_number.\n"
                "Think step by step and solve the task carefully."
            ),
        ),
    )

    history = []
    current_prompt = (
        f"Goal: {goal}\n"
        f"The secret number is between 1 and 20.\n"
        "Start by choosing the best first tool call."
    )

    for iteration in range(1, max_iterations + 1):
        print(f"\n=== Iteration {iteration} ===")
        print("Prompt to agent:")
        print(current_prompt)

        response = chat.send_message(current_prompt)

        text = response.text if response.text else ""
        print("\nAgent response:")
        print(text)

        history.append(f"Iteration {iteration}: {text}")

        # Stop if the model clearly found the answer
        if "Correct! The secret number is" in text:
            return text

        # Build the next prompt using accumulated observations
        observation_block = "\n".join(history)
        current_prompt = (
            f"Goal: {goal}\n"
            f"Iteration: {iteration}/{max_iterations}\n"
            "Here are the observations so far:\n"
            f"{observation_block}\n\n"
            "Based on these observations, choose the single best next step.\n"
            "If you are confident, use guess_number.\n"
            "Otherwise, call one of the clue tools."
        )

    return "Maximum iterations reached before the agent found the secret number."


if __name__ == "__main__":
    goal = "Find the secret number."
    answer = run_hidden_number_agent(goal)

    print("\nFinal answer:\n")
    print(answer)