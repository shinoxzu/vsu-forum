<script setup>
import Button from "primevue/button";
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import { useAuthStore } from "../stores/auth";

const topics = ref([]);
const router = useRouter();
const authStore = useAuthStore();
const showEditDialog = ref(false);
const selectedTopicId = ref(null);
const newTopicName = ref("");
const errorMessages = ref([]);
const errorId = ref(0);

async function fetchTopics() {
    try {
        const response = await fetch("http://localhost:3000/topics");

        if (response.ok) {
            topics.value = await response.json();
        } else {
            topics.value = [];
            errorMessages.value.push({
                content: "Ошибка при загрузке топиков",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

async function removeTopic(id) {
    try {
        const response = await fetch("http://localhost:3000/topics/" + id, {
            method: "DELETE",
            headers: {
                Authorization: `Bearer ${authStore.token}`,
            },
        });

        if (response.ok) {
            await fetchTopics();
        } else {
            errorMessages.value.push({
                content: "Ошибка при удалении топика",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

async function updateTopic() {
    try {
        const response = await fetch(
            `http://localhost:3000/topics/${selectedTopicId.value}`,
            {
                method: "PATCH",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: `Bearer ${authStore.token}`,
                },
                body: JSON.stringify({
                    name: newTopicName.value,
                }),
            },
        );

        if (response.ok) {
            showEditDialog.value = false;
            await fetchTopics();
        } else {
            errorMessages.value.push({
                content: "Ошибка при обновлении топика",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

async function bookmarkTopic(id) {
    try {
        const response = await fetch(
            `http://localhost:3000/topics/${id}/bookmark`,
            {
                method: "POST",
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
            },
        );

        if (!response.ok) {
            errorMessages.value.push({
                content: "Ошибка при добавлении в закладки",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

function openEditDialog(topic) {
    selectedTopicId.value = topic.id;
    newTopicName.value = topic.name;
    showEditDialog.value = true;
}

onMounted(() => {
    fetchTopics();
});
</script>

<template>
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

    <div class="topics-container">
        <div
            style="text-decoration: none; color: white"
            v-for="topic in topics"
            :key="topic.id"
            @click="router.push(`/topics/${topic.id}`)"
        >
            <div class="topic">
                <div class="topic-name">{{ topic.name }}</div>
                <div class="topic-action-buttons">
                    <Button
                        icon="pi pi-pencil"
                        @click.stop="openEditDialog(topic)"
                    />
                    <Button
                        icon="pi pi-trash"
                        severity="danger"
                        @click.stop="removeTopic(topic.id)"
                    />
                    <Button
                        icon="pi pi-bookmark"
                        @click.stop="bookmarkTopic(topic.id)"
                    />
                </div>
            </div>
        </div>
    </div>

    <Dialog v-model:visible="showEditDialog" modal header="Редактировать топик">
        <div class="edit-form">
            <InputText v-model="newTopicName" placeholder="Новое название" />
            <Button label="Сохранить" @click="updateTopic" />
        </div>
    </Dialog>
</template>

<style scoped>
.topics-container {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.topic {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    gap: 10px;
    background-color: #222222;
    border-radius: 7px;
    padding: 20px;
}

.topic-name {
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.topic-action-buttons {
    display: flex;
    gap: 10px;
}

.main-page {
    padding: 20px;
}

.top-page {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.edit-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
}

.errors-container {
    margin-bottom: 1rem;
}
</style>
