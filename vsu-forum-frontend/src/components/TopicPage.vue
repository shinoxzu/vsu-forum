<script setup>
import { ref, onMounted } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const topic = ref({});
const posts = ref([]);
const newPostText = ref("");

async function fetchTopic() {
    try {
        const response = await fetch(
            `http://localhost:3000/topics/${route.params.id}`,
        );
        if (response.ok) {
            topic.value = await response.json();
        } else {
            console.error("Ошибка при загрузке топика");
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

async function fetchPosts() {
    try {
        const response = await fetch(
            `http://localhost:3000/posts?topic_id=${route.params.id}`,
        );
        if (response.ok) {
            posts.value = await response.json();
        } else {
            console.error("Ошибка при загрузке постов");
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

async function createPost() {
    if (!newPostText.value) return; // Проверка, что текст поста не пустой
    try {
        const token = localStorage.getItem("token");
        if (!token) {
            console.error("Токен не найден, авторизация не выполнена.");
            return;
        }
        const response = await fetch("http://localhost:3000/posts", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                Authorization: `Bearer ${token}`,
            },
            body: JSON.stringify({
                topic_id: parseInt(route.params.id, 10),
                text: newPostText.value,
            }),
        });
        if (response.ok) {
            newPostText.value = ""; // Очистить текстовое поле
            fetchPosts(); // Обновить список постов
        } else {
            console.error("Ошибка при создании поста");
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

onMounted(() => {
    fetchTopic();
    fetchPosts();
});
</script>

<template>
    <div class="topic-page">
        <h2>{{ topic.name }}</h2>
        <p>Автор ID: {{ topic.author_id }}</p>
        <p>Категория ID: {{ topic.category_id }}</p>

        <h3>Посты в этом топике</h3>
        <ul v-if="posts.length">
            <li v-for="post in posts" :key="post.id">
                {{ post.text }}
            </li>
        </ul>
        <p v-else>Постов пока нет.</p>

        <h3>Создать новый пост</h3>
        <textarea
            v-model="newPostText"
            placeholder="Введите текст поста"
        ></textarea>
        <button @click="createPost">Добавить пост</button>
    </div>
</template>

<style scoped>
.topic-page {
    padding: 20px;
}

textarea {
    display: block;
    width: 100%;
    height: 100px;
    margin-top: 10px;
    margin-bottom: 10px;
}

button {
    padding: 10px 20px;
}
</style>
