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
# Tool: Count regular files
# -----------------------------
def count_files(directory: str = ".") -> str:
    try:
        path = Path(directory)
        if not path.exists():
            return f"Error: directory '{directory}' does not exist."
        if not path.is_dir():
            return f"Error: '{directory}' is not a directory."
        count = sum(1 for item in path.iterdir() if item.is_file() and not item.name.startswith("."))
        return str(count)
    except Exception as e:
        return f"Error counting files: {e}"


def run_agent(user_query: str):
    chat = client.chats.create(
        model="gemini-2.5-flash",
        config=types.GenerateContentConfig(
            tools=[count_files], # Just pass the function itself!
            temperature=0.1,
            system_instruction=(
              "You are a helpful assistant. "
              "When the user asks about the number of files in a folder, "
              "use the count_files tool instead of guessing."
            ),
        )
    )
    response = chat.send_message(user_query)
    return response.text


if __name__ == "__main__":
    query = "Count the number of files in the current folder, and tell me one interesting mathematical fact about that number."
    answer = run_agent(query)

    print("\nFinal answer:\n")
    print(answer)