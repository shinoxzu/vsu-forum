<script setup>
import AutoComplete from 'primevue/autocomplete';
import Button from 'primevue/button';
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
        <AutoComplete
            v-model="newCategoryName"
            optionLabel="name"
            :suggestions="categories"
            @complete="fetchCategories"
            placeholder="Название категории"
        >
            <template #option="slotProps">
                <div class="flex items-center">
                    <img
                        :alt="slotProps.option.name"
                        src="https://primefaces.org/cdn/primevue/images/flag/flag_placeholder.png"
                        :class="`flag flag-${slotProps.option.code.toLowerCase()} mr-2`"
                        style="width: 18px"
                    />
                    <div>{{ slotProps.option.name }}</div>
                </div>
            </template>
            <template #header>
                <div class="font-medium px-3 py-2">Доступные категории</div>
            </template>
            <template #footer>
                <div class="px-3 py-3">
                    <Button
                        @click="createCategory"
                        type="button"
                        label="Создать категорию"
                        fluid
                        severity="secondary"
                        text
                        size="small"
                        icon="pi pi-plus"
                    />
                </div>
            </template>
        </AutoComplete>

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
