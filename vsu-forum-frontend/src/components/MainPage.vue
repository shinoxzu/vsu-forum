<script setup>
import Button from "primevue/button";
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import { useAuthStore } from "../stores/auth";
import { formatRelativeTime } from "../utils/date";
import Paginator from "primevue/paginator";

const topics = ref([]);
const router = useRouter();
const authStore = useAuthStore();
const showEditDialog = ref(false);
const selectedTopicId = ref(null);
const newTopicName = ref("");
const errorMessages = ref([]);
const errorId = ref(0);
const sortOrder = ref("newest");

const first = ref(0);
const rowsPerPage = ref(10);
const totalTopics = ref(0);
const displayedTopics = ref([]);

function toggleSortOrder() {
    sortOrder.value = sortOrder.value === "newest" ? "oldest" : "newest";
    first.value = 0;
    sortTopics();
}

async function fetchTopics() {
    try {
        const response = await fetch("http://localhost:3000/topics");

        if (response.ok) {
            topics.value = await response.json();
            totalTopics.value = topics.value.length;
            sortTopics();
            updateDisplayedTopics();
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
            first.value = 0;
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
            first.value = 0;
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

function updateDisplayedTopics() {
    const startIndex = first.value;
    const endIndex = Math.min(
        startIndex + rowsPerPage.value,
        topics.value.length,
    );
    displayedTopics.value = topics.value.slice(startIndex, endIndex);
}

function sortTopics() {
    topics.value.sort((a, b) => {
        const dateA = new Date(a.created_at).getTime();
        const dateB = new Date(b.created_at).getTime();
        return sortOrder.value === "newest" ? dateB - dateA : dateA - dateB;
    });
    updateDisplayedTopics();
}

function onPage(event) {
    first.value = event.first;
    rowsPerPage.value = event.rows;
    updateDisplayedTopics();
}

onMounted(() => {
    fetchTopics();
});
</script>

<template>
    <div class="top-page">
        <div class="header-container">
            <h2>Топики</h2>
            <Button
                icon="pi pi-sort"
                @click="toggleSortOrder"
                :label="
                    sortOrder === 'newest' ? 'Сначала новые' : 'Сначала старые'
                "
            />
            <Button
                as="router-link"
                label="Создать топик"
                to="/create-topic"
                replace
                style="text-decoration: none"
            />
        </div>
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
            v-for="topic in displayedTopics"
            :key="topic.id"
            @click="router.push(`/topics/${topic.id}`)"
        >
            <div class="topic">
                <div class="topic-info">
                    <div class="topic-name">{{ topic.name }}</div>
                    <div class="topic-date">
                        {{ formatRelativeTime(topic.created_at) }}
                    </div>
                </div>
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

    <Paginator
        v-model:first="first"
        v-model:rows="rowsPerPage"
        :totalRecords="totalTopics"
        :rowsPerPageOptions="[5, 10, 20, 30]"
        @page="onPage"
        template="FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink RowsPerPageDropdown"
        class="paginator"
    />

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

.topic-info {
    display: flex;
    flex-direction: column;
    gap: 5px;
}

.topic-name {
    font-size: 1.1em;
}

.topic-date {
    font-size: 0.9em;
    color: #888;
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
    margin-bottom: 20px;
}

.header-container {
    display: flex;
    gap: 15px;
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
