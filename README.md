# Todo app
This is Todo api by using rust language.  
The framework that this app using is Axum.

# Run
```
cargo run
```

## Hello world
You can check server running normally.  
```
http://localhost:3000
```

## Json-Response
I recomend you to using **Insomnia**.  https://github.com/Kong/insomnia  

### Post Json　　
uri is ```http://localhost:3000/todo```  
choosing HTTP Request, post method and write this.

```
{
  "text": "todo name"
}
```
You cannot send json when you try to empty or more than 100 letter.  

### Get Json　　
choosing HTTP Request, get method and write this.  
if you want to get json that id is 3, you input this. 
```http://localhost:3000/todo/3```

You can get all json by this way.  
choosing HTTP Request, get method and write this.  
```http://localhost:3000/todo```

### Put Json
If you want to update todo, you can do it by this way.
choosing HTTP Request, put method and wirte json body.
```
{
  "text": "change todo name"
}
```
or
```
{
  "completed": true
}
```
or
```
{
  "text": "change todo name",
  "completed: "true"
}
```

It is because the logic code of update_todo is written by using Option<>, So you can select change only text, only completed or both.　　

### Delete Json
If you want to Delete todo, you can do it by this way.
choosing HTTP Request, delete method.  
if you want to delete todo that id is 3, you should input uri.  
```http://localhost:3000/todo/3```


