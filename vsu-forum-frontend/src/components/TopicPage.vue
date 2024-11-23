<script setup>
import { ref, onMounted } from "vue";
import { useRoute } from "vue-router";
import { Form } from "@primevue/forms";
import InputText from "primevue/inputtext";
import AutoComplete from "primevue/autocomplete";
import Button from "primevue/button";
import Message from "primevue/message";
import Textarea from 'primevue/textarea';

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
    if (!newPostText.value) return;

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
            newPostText.value = "";
            fetchPosts();
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
        <div class="topic-info">
            <h2>{{ topic.name }}</h2>
            <p>Автор ID: {{ topic.author_id }}</p>
            <p>Категория ID: {{ topic.category_id }}</p>
        </div>

        <div class="posts-container" v-if="posts.length">
            <div class="post" v-for="post in posts" :key="post.id">
                {{ post.text }}
            </div>
        </div>
        
        <div class="send-message-container">
            <Form @submit="createPost" class="send-post-form">
                <Textarea placeholder="Текст поста" v-model="newPostText" autoResize rows="5" />
                <Button
                    type="submit"
                    label="Отправить"
                />
            </Form>
        </div>
    </div>
</template>

<style scoped>
.topic-page {
    padding: 20px;
}

.topic-info {
    background-color: #222222;
    border-radius: 7px;
    padding-left: 10px;
    padding-top: 20px;
    padding-bottom: 20px;
    margin-bottom: 10px;
}

.posts-container {
    display: flex;
    flex-direction: column;
    align-items: start;
    gap: 13px;
    min-width: 220px;
    max-width: 30%;
    width: 20%;
}

.post {
    background-color: #222222;
    border-radius: 7px;
    padding: 10px;
}

.send-post-form {
    display: flex;
    flex-direction: column;
    align-items: start;
    gap: 13px;
    min-width: 220px;
    max-width: 30%;
    width: 20%;
}

.send-message-container {
    background-color: #222222;
    border-radius: 7px;
    padding-left: 10px;
    padding-top: 20px;
    padding-bottom: 20px;
    margin-top: 10px;
}
</style>
