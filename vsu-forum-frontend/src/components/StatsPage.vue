<script setup>
import { ref, onMounted } from "vue";
import Message from "primevue/message";

const stats = ref({
    posts_count: 0,
    users_count: 0,
    topics_count: 0,
});
const errorMessages = ref([]);
const errorId = ref(0);

onMounted(() => {
    fetchStats();
});

async function fetchStats() {
    try {
        const response = await fetch("http://localhost:3000/stats");

        if (response.ok) {
            stats.value = await response.json();
        } else {
            const error = await response.json();
            errorMessages.value.push({
                content: error.err || "Ошибка при загрузке статистики",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка:", error);
        errorMessages.value.push({
            content: "Ошибка при загрузке статистики",
            id: errorId.value++,
        });
    }
}
</script>

<template>
    <div class="page-header">
        <h2>Статистика форума</h2>
    </div>

    <div v-if="errorMessages.length > 0" class="errors-container">
        <Message
            v-for="msg of errorMessages"
            :key="msg.id"
            severity="error"
            :life="3000"
        >
            {{ msg.content }}
        </Message>
    </div>

    <h4>Blazingly FAST 🚀🚀🚀 VSU FORUM written in Rust 🚀🚀🌰 + Axum + Vue</h4>

    <div class="stats-container">
        <div class="stat-card">
            <div class="stat-icon">
                <i class="pi pi-comments"></i>
            </div>
            <div class="stat-info">
                <h3>Сообщения</h3>
                <span class="stat-value">{{ stats.posts_count }}</span>
            </div>
        </div>

        <div class="stat-card">
            <div class="stat-icon">
                <i class="pi pi-users"></i>
            </div>
            <div class="stat-info">
                <h3>Пользователи</h3>
                <span class="stat-value">{{ stats.users_count }}</span>
            </div>
        </div>

        <div class="stat-card">
            <div class="stat-icon">
                <i class="pi pi-list"></i>
            </div>
            <div class="stat-info">
                <h3>Темы</h3>
                <span class="stat-value">{{ stats.topics_count }}</span>
            </div>
        </div>
    </div>
</template>

<style scoped>
.page-header {
    margin-bottom: 2rem;
}

.stats-container {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 2rem;
    padding: 1rem;
}

.stat-card {
    background-color: #222222;
    border-radius: 8px;
    padding: 1.5rem;
    display: flex;
    align-items: center;
    gap: 1.5rem;
    transition: transform 0.2s ease;
}

.stat-card:hover {
    transform: translateY(-5px);
}

.stat-icon {
    background-color: #1e1e1e;
    border-radius: 50%;
    width: 60px;
    height: 60px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.stat-icon i {
    font-size: 1.5rem;
    color: #64b5f6;
}

.stat-info {
    display: flex;
    flex-direction: column;
}

.stat-info h3 {
    margin: 0;
    font-size: 1.1rem;
    color: #888;
}

.stat-value {
    font-size: 1.5rem;
    font-weight: bold;
    color: #fff;
}

.errors-container {
    margin-bottom: 15px;
}
</style>
