# This code sample is not tested, and is provided just as reference
# Only Gemini and Ollama based code on this portal are tested
from openai import OpenAI

client = OpenAI()

response = client.chat.completions.create(
    model="o1",
    messages=[
        {
            "role": "user",
            "content": "Design a fault-tolerant multi-agent system."
        }
    ],
    reasoning_effort="high",  # 🔥 Increase internal reasoning depth
)

print(response.choices[0].message.content)

# Includes hidden reasoning tokens
print(response.usage)