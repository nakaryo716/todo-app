import {FC, useEffect, useState} from "react";
import {Box, createTheme, Stack, ThemeProvider, Typography} from "@mui/material";
import {NewTodoPayload, Todo} from "./types/todo";
import TodoForm from "./components/TodoForm.tsx";
import TodoList from "./components/TodoList.tsx";
import {addTodoItem, getTodoItem, updateTodoItem} from "./lib/api/todo.ts";

const TodoApp: FC = () => {
    const [todos, setTodos] = useState<Todo[]>([]);


    const onSubmit = async (payload: NewTodoPayload) => {
        if (!payload) return

        await addTodoItem(payload);
        const todos = await getTodoItem();

        setTodos(todos);
    }

   const onUpdate = async (updateTodo: Todo) => {
        await updateTodoItem(updateTodo);
        const todos = await getTodoItem(); 

        setTodos(todos);
    }

    useEffect(() => {
        (async () => {
            const todos = await getTodoItem()
            setTodos(todos)
        })()
    }, [])

    return (
        <>
            <Box
                sx={{
                    backgroundColor: 'white',
                    borderBottom: '1px solid gray',
                    display: 'flex',
                    alignItems: 'center',
                    position: 'fixed',
                    top: 0,
                    p: 2,
                    width: '100%',
                    height: 80,
                    zIndex: 3,
                }}
            >
                <Typography variant={"h1"}>Todo App</Typography>
            </Box>
            <Box
                sx={{
                    display: 'flex',
                    justifyContent: 'center',
                    p: 5,
                    mt: 10,
                    backgroundColor: 'white',
                }}
            >
                <Box maxWidth={700} width={"100%"}>
                    <Stack spacing={5}>
                        <TodoForm onSubmit={onSubmit} />
                        <TodoList todos={todos} onUpdate={onUpdate} onDelete={() => {}} />
                    </Stack>
                </Box>
            </Box>
        </>
    )
}

const theme = createTheme({
    typography: {
        h1: {
            fontSize: 30,
        },
        h2: {
            fontSize: 20,
        },
    },
})

const App: FC = () => {
    return (
        <ThemeProvider theme={theme}>
            <TodoApp />
        </ThemeProvider>
    )
}

export default App