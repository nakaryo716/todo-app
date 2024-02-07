import type {NewTodoPayload, Todo} from "../../types/todo";

export const addTodoItem = async (payload: NewTodoPayload) => {
    const  res = await fetch('http://localhost:8080/todos', {
        method: 'post',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
    })

    if (!res.ok){
        throw new Error('add todo request failed')
    }

    const json: Todo = await res.json();
    return json;
}