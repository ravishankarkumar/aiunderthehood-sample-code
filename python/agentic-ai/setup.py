import os
from dotenv import load_dotenv
from google import genai
from pathlib import Path

env_path = Path(__file__).parent.parent.parent / ".env"
# Replace the env_path with the actual path to your .env file if it's located elsewhere
load_dotenv(dotenv_path=env_path)

api_key = os.getenv("GEMIN_API_KEY")
if not api_key:
    raise ValueError("GEMIN_API_KEY not found in environment variables. Please set it in the .env file.")

try:
    client = genai.Client(api_key=api_key)

    response = client.models.generate_content(
        model="gemini-2.5-flash", contents="Explain how AI works in a few words"
    )
    print(response.text)
except Exception as e:
    print(f"An error occurred: {e}")