from ollama import chat
from pydantic import BaseModel

response = chat(
    model="qwen3.5:9b",
    messages=[{"role": "user", "content": "Say hello from Ollama!"}]
)

print(response.message.content)