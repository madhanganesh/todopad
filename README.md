# 📝 todopad

**todopad** is a clean and focused task management app that helps you capture, track, and complete the todos you intend to finish.

It is built with **Rust** on the backend (Axum) and **HTMX** on the frontend for a fast, server-driven UI.

---

## ✨ Features

- ✅ Simple and intuitive interface to manage your todos  
- 🧠 Automatically generates **AI-powered tags** from todo text  
- 📊 Visualize **insights** based on tags (e.g., time spent on Project A vs B)  
- ⚡ Quickly act on todos — mark complete, reschedule, adjust effort  
- 📱 Fully mobile-compatible design  

> 🔍 [Explore the app with screenshots & demo](https://www.todopad.in/static/pages/explore.html)

---

## 🚀 How to Run Locally

### 1. Clone the repository
```bash
git clone https://github.com/madhanganesh/todopad.git
cd todopad
```

### 2. Create a .env file with the following:
```bash
ENV=development
DATABASE_URL=sqlite://todopad.db
SQLX_OFFLINE=false
RUST_LOG=debug
OPENAI_API_KEY=<your_openai_key>
```

### 3. In a seperate terminal build UI assets
```bash
cd UI
npm i
npm run build
```
there is also `npm run watch` if you want to watch for changes and build

### 4. Start the backend server
```bash
cargo run
```

---

## 🐳 Run with Docker
```bash
docker build -t todopad .
docker run --name todopad \
  -e ENV=release \
  -e DATABASE_URL=sqlite://todopad.db \
  -e SQLX_OFFLINE=true \
  -e RUST_LOG=error \
  -e OPENAI_API_KEY=<your_openai_key> \
  -p 8080:8080 todopad
```

---

## 🌍 Deployment

The app is deployed on Fly.io and is live at:

👉 https://www.todopad.in

---

## 📄 License

MIT License

---

## 💬 Feedback

For feedback, ideas, or bugs, feel free to reach out:

📧 madhanganesh@gmail.com

