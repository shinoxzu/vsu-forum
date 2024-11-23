<script setup>
import Button from "primevue/button";
import { ref, onMounted } from "vue";

const topics = ref([]);

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

onMounted(() => {
    fetchTopics();
});
</script>

<template>
    <div class="main-page">
        <div class="top-page">
            <h2>Топики</h2>
            <Button
                as="router-link"
                label="Создать топик"
                to="/create-topic"
                replace
                style="text-decoration: none"
            />
        </div>
        <div class="topics-container">
            <router-link
                style="text-decoration: none; color: white"
                :to="`/topics/${topic.id}`"
                v-for="topic in topics"
                :key="topic.id"
            >
                <div class="topic">
                    <div>{{ topic.name }}</div>
                </div>
            </router-link>
        </div>
    </div>
</template>

<style scoped>
.topics-container {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.topic {
    background-color: #222222;
    border-radius: 7px;
    padding-left: 10px;
    padding-top: 20px;
    padding-bottom: 20px;
}

.main-page {
    padding: 20px;
}

.top-page {
    display: flex;
    justify-content: space-between;
    align-items: center;
}
</style>
