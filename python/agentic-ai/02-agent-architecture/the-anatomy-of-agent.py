from typing import Any, Callable
# import json
from ollama import chat
from pydantic import BaseModel

MODEL = "qwen3.5:9b"
MAX_STEPS = 6


class Action(BaseModel):
    type: str  # "tool" | "final"
    name: str | None = None
    args: dict[str, Any] = {}
    answer: str | None = None


class Agent:
    def __init__(self, tools: dict[str, Callable[..., str]]):
        self.tools = tools
        self.history: list[dict[str, str]] = []

    def build_messages(self, observation: str) -> list[dict[str, str]]:
        system_prompt = """
You are an agent runtime planner.
Return ONLY JSON with schema:
{
  "type": "tool" | "final",
  "name": string | null,
  "args": object,
  "answer": string | null
}
Use "tool" when you need external data. Use "final" only when done.
"""
        return [{"role": "system", "content": system_prompt}, *self.history, {"role": "user", "content": observation}]

    def step(self, observation: str) -> tuple[str, bool]:
        response = chat(model=MODEL, messages=self.build_messages(observation), options={"temperature": 0.2})
        raw = response.message.content
        action = Action.model_validate_json(raw)
        self.history.append({"role": "assistant", "content": raw})

        if action.type == "final":
            return action.answer or "No answer provided.", True

        if action.type == "tool":
            if not action.name or action.name not in self.tools:
                tool_result = f"Tool '{action.name}' not available."
            else:
                try:
                    tool_result = self.tools[action.name](**action.args)
                except Exception as exc:
                    tool_result = f"Tool error: {exc}"

            self.history.append({"role": "tool", "content": tool_result})
            return tool_result, False

        return f"Unknown action type: {action.type}", False

    def run(self, goal: str) -> str:
        observation = goal
        for step_no in range(1, MAX_STEPS + 1):
            observation, done = self.step(observation)
            print(f"[step {step_no}/{MAX_STEPS}] completed")
            if done:
                return observation
        return "Stopped: reached max iterations without final answer."


def web_search(query: str) -> str:
    return f"[mock] top search results for: {query}"


agent = Agent(tools={"web_search": web_search})
final_answer = agent.run("Research the current state of MCP and summarize key updates.")
print(final_answer)