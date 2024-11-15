<script setup>
import { ref, onMounted } from "vue";

const topics = ref([]);
const categories = ref([]);
const newCategoryName = ref("");
const newTopicName = ref("");
const selectedCategory = ref(null);

async function fetchTopics() {
    try {
        const response = await fetch("http://localhost:3000/topics");
        if (response.ok) {
            topics.value = await response.json();
        } else {
            topics.value = [];
            console.error("Ошибка при загрузке топиков");
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

async function fetchCategories() {
    try {
        const response = await fetch("http://localhost:3000/topics-categories");
        if (response.ok) {
            categories.value = await response.json();
        } else {
            console.error("Ошибка при загрузке категорий");
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

async function createCategory() {
    if (!newCategoryName.value) return;
    try {
        const token = localStorage.getItem("token");
        if (!token) {
            console.error("Токен не найден, авторизация не выполнена.");
            return;
        }

        await fetch("http://localhost:3000/topics-categories", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                Authorization: `Bearer ${token}`,
            },
            body: JSON.stringify({ name: newCategoryName.value }),
        });
        newCategoryName.value = "";
        fetchCategories(); // Обновить список категорий
    } catch (error) {
        console.error("Ошибка при создании категории:", error);
    }
}

async function createTopic() {
    if (!newTopicName.value || !selectedCategory.value) return;
    try {
        const token = localStorage.getItem("token");
        if (!token) {
            console.error("Токен не найден, авторизация не выполнена.");
            return;
        }
        await fetch("http://localhost:3000/topics", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                Authorization: `Bearer ${token}`,
            },
            body: JSON.stringify({
                name: newTopicName.value,
                category_id: selectedCategory.value,
            }),
        });
        newTopicName.value = "";
        fetchTopics(); // Обновить список топиков
    } catch (error) {
        console.error("Ошибка при создании топика:", error);
    }
}

onMounted(() => {
    fetchTopics();
    fetchCategories();
});
</script>

<template>
    <div class="main-page">
        <h2>Топики форума</h2>
        <ul v-if="topics.length">
            <li v-for="topic in topics" :key="topic.id">
                <router-link :to="`/topics/${topic.id}`">{{
                    topic.name
                }}</router-link>
            </li>
        </ul>
        <p v-else>Топики пока не добавлены.</p>

        <h3>Создать новую категорию</h3>
        <input v-model="newCategoryName" placeholder="Название категории" />

        <button @click="createCategory">Создать категорию</button>

        <h3>Создать новый топик</h3>
        <select v-model="selectedCategory">
            <option
                v-for="category in categories"
                :key="category.id"
                :value="category.id"
            >
                {{ category.name }}
            </option>
        </select>
        <input v-model="newTopicName" placeholder="Название топика" />
        <button @click="createTopic">Создать топик</button>
    </div>
</template>

<style scoped>
.main-page {
    padding: 20px;
}
</style>
