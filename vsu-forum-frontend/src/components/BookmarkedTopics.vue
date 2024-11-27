<script setup>
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "../stores/auth";
import Button from "primevue/button";
import Message from "primevue/message";

const router = useRouter();
const authStore = useAuthStore();
const bookmarks = ref([]);
const topics = ref([]);
const errorMessages = ref([]);
const errorId = ref(0);

async function fetchBookmarks() {
    try {
        if (!authStore.token) {
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch("http://localhost:3000/bookmarks", {
            headers: {
                Authorization: `Bearer ${authStore.token}`,
            },
        });

        if (response.ok) {
            bookmarks.value = await response.json();
            await fetchTopicDetails();
        } else {
            const error = await response.json();
            errorMessages.value.push({
                content: error.err || "Ошибка при загрузке закладок",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка:", error);
        errorMessages.value.push({
            content: "Произошла ошибка при загрузке закладок",
            id: errorId.value++,
        });
    }
}

async function fetchTopicDetails() {
    try {
        const topicsData = await Promise.all(
            bookmarks.value.map(async (bookmark) => {
                const response = await fetch(
                    `http://localhost:3000/topics/${bookmark.topic_id}`,
                );
                if (response.ok) {
                    return await response.json();
                }
                return null;
            }),
        );
        topics.value = topicsData.filter((topic) => topic !== null);
    } catch (error) {
        console.error("Ошибка при загрузке информации о топиках:", error);
    }
}

async function removeBookmark(topicId) {
    try {
        if (!authStore.token) {
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch(
            `http://localhost:3000/topics/${topicId}/bookmark`,
            {
                method: "DELETE",
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
            },
        );

        if (response.ok) {
            // Remove the topic from the local list
            topics.value = topics.value.filter((topic) => topic.id !== topicId);
        } else {
            const error = await response.json();
            errorMessages.value.push({
                content: error.err || "Ошибка при удалении закладки",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка:", error);
        errorMessages.value.push({
            content: "Произошла ошибка при удалении закладки",
            id: errorId.value++,
        });
    }
}

function navigateToTopic(topicId) {
    router.push(`/topics/${topicId}`);
}

onMounted(async () => {
    await fetchBookmarks();
});
</script>

<template>
    <div class="page-header">
        <h2>Закладки</h2>
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

    <div v-if="topics.length === 0" class="no-bookmarks">
        <p>У вас пока нет закладок</p>
    </div>

    <div v-else class="bookmarks-list">
        <div v-for="topic in topics" :key="topic.id" class="bookmark-item">
            <div class="bookmark-content" @click="navigateToTopic(topic.id)">
                <h3>{{ topic.name }}</h3>
                <p>Категория: {{ topic.category.name }}</p>
                <p>Количество постов: {{ topic.posts_count }}</p>
            </div>
            <div class="bookmark-actions">
                <Button
                    icon="pi pi-trash"
                    severity="danger"
                    @click="removeBookmark(topic.id)"
                    tooltip="Удалить из закладок"
                />
            </div>
        </div>
    </div>
</template>

<style scoped>
.bookmarks-page {
    padding: 20px;
}

.page-header {
    margin-bottom: 20px;
}

.bookmarks-list {
    display: flex;
    flex-direction: column;
    gap: 15px;
}

.bookmark-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 15px;
    background-color: #222222;
    border-radius: 8px;
    transition: background-color 0.2s;
}

.bookmark-item:hover {
    background-color: #2a2a2a;
    cursor: pointer;
}

.bookmark-content {
    flex-grow: 1;
}

.bookmark-content h3 {
    margin: 0 0 10px 0;
    color: #ffffff;
}

.bookmark-content p {
    margin: 5px 0;
    color: #cccccc;
}

.bookmark-actions {
    display: flex;
    gap: 10px;
}

.no-bookmarks {
    text-align: center;
    padding: 40px;
    color: #cccccc;
}

.errors-container {
    margin-bottom: 20px;
}
</style>
