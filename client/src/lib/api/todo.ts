import type {NewTodoPayload, Todo} from "../../types/todo";

export const addTodoItem = async (payload: NewTodoPayload) => {
    const  res = await fetch('http://localhost:3000/todo', {
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

export const getTodoItem = async () => {
    const res = await fetch('http://localhost:3000/todo');

    if (!res.ok) {
        throw new Error('get todo request failed');
    }

    const json = await res.json();
    return json;
}

export const updateTodoItem = async (todo: Todo) => {
    const {id, ...updateTodo} = todo;

    const res = await fetch(`http://localhost:3000/todo/${id}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(updateTodo),
    })

    if (!res.ok) {
        throw new Error('update todo request failed')
    }

    const json: Todo = await res.json();
    return json;
}