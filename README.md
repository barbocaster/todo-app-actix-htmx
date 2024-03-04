# Todo App Using HTMX + TailwindCSS + Actix Web + MongoDB

This is a simple CRUD todo app that I did, you can add, remove and check tasks. It uses HTMX + TailwindCSS on the frontend and Actix Web + MongoDB on the backend with Rust. That's my first full stack app that I did so the code might be a little messy.

![Todo app index page](no_image)

# How To Run it

Step 1. Make sure you have a MongoDB database environment setup locally or online.
        You should also have an environment variable on your system called `URI_CONN_STR` with the connection string of your database.

Step 2. Then, make sure you have cargo and git installed on your computer and run these commands.

```bash
git clone https://github.com/barbocaster/todo-app-actix-htmx.git
cd todo-app-actix-htmx/backend/actix
cargo run
```

After that, simply open your browser on `127.0.0.1:8080` and you're ready to organize your life!
