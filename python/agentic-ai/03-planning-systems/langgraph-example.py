from typing import Literal, Annotated
from typing_extensions import TypedDict
from langchain_ollama import ChatOllama
from langchain_core.messages import HumanMessage, AIMessage, SystemMessage
from langgraph.graph import StateGraph, START, END
from langgraph.graph.message import add_messages
from langgraph.checkpoint.memory import MemorySaver

MODEL = "qwen3.5:9b"

llm = ChatOllama(model=MODEL, temperature=0.2)

# 1. Define the state
class AgentState(TypedDict):
    goal: str
    messages: Annotated[list, add_messages]
    next: Literal["tools", "final_answer"]
    final_report: str | None

# 2. Define nodes
def reasoning_node(state: AgentState) -> dict:
    prompt = (
        f"Goal: {state['goal']}\n\n"
        "Decide the next action. If you need to look something up, say 'use tool'. "
        "Otherwise provide a final_answer."
    )
    messages = [SystemMessage(content="You are a GPU expert.")] + list(state["messages"]) + [HumanMessage(content=prompt)]
    response: AIMessage = llm.invoke(messages)
    next_action = "tools" if "tool" in response.content.lower() else "final_answer"
    return {
        "messages": [response],
        "next": next_action,
    }

def tool_node(state: AgentState) -> dict:
    # Simulated tool result
    result = "H100 outperforms RTX 4090 in training throughput by 2-3x."
    return {
        "messages": [HumanMessage(content=f"Tool result: {result}")],
    }

def final_answer_node(state: AgentState) -> dict:
    history = "\n".join(m.content for m in state["messages"])
    messages = [
        SystemMessage(content="You are a GPU expert."),
        HumanMessage(content=f"Summarize a clear final answer based on:\n{history}"),
    ]
    response: AIMessage = llm.invoke(messages)
    return {
        "messages": [response],
        "final_report": response.content,
    }

# 3. Build the graph
workflow = StateGraph(AgentState)
workflow.add_node("reasoning", reasoning_node)
workflow.add_node("tools", tool_node)
workflow.add_node("final_answer", final_answer_node)

# 4. Define edges
workflow.add_edge(START, "reasoning")

def route_after_reasoning(state: AgentState) -> Literal["tools", "final_answer"]:
    return state["next"]

workflow.add_conditional_edges(
    "reasoning",
    route_after_reasoning,
    {"tools": "tools", "final_answer": "final_answer"},
)
workflow.add_edge("tools", "reasoning")   # ReAct cycle
workflow.add_edge("final_answer", END)

# 5. Compile with checkpointing
graph = workflow.compile(checkpointer=MemorySaver())

# 6. Run
initial_state: AgentState = {
    "goal": "Compare RTX 4090 and H100 for machine learning workloads",
    "messages": [],
    "next": "final_answer",
    "final_report": None,
}

result = graph.invoke(initial_state, config={"configurable": {"thread_id": "1"}})

print("\n=== FINAL REPORT ===")
print(result["final_report"])
