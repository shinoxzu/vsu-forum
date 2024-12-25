<script setup>
import { ref, onMounted } from "vue";
import { useRoute } from "vue-router";
import { useAuthStore } from "../stores/auth";
import InputText from "primevue/inputtext";
import Button from "primevue/button";
import Select from "primevue/select";
import Dialog from "primevue/dialog";
import Textarea from "primevue/textarea";
import Message from "primevue/message";
import { formatDate, formatRelativeTime } from "../utils/date";
import Paginator from "primevue/paginator";

const route = useRoute();
const authStore = useAuthStore();
const topic = ref({});
const posts = ref([]);
const reactions = ref([]);
const availableReactions = ref([]);
const newPostText = ref("");
const editedPostId = ref(null);
const editedPostText = ref("");
const showEditDialog = ref(false);
const editedTopicName = ref("");
const selectedCategory = ref(null);
const topicCategories = ref([]);
const errorMessages = ref([]);
const errorId = ref(0);

const first = ref(0);
const rowsPerPage = ref(10);
const totalPosts = ref(0);
const displayedPosts = ref([]);

onMounted(() => {
    fetchTopic();
    fetchPosts();
    fetchCategories();
    fetchAvailableReactions();
});

async function fetchTopic() {
    try {
        const response = await fetch(
            `http://localhost:3000/topics/${route.params.id}`,
        );

        if (response.ok) {
            topic.value = await response.json();
            editedTopicName.value = topic.value.name;
            selectedCategory.value = topic.value.category;
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
            console.log(posts.value);
            totalPosts.value = posts.value.length;
            posts.value.sort(
                (a, b) => new Date(a.created_at) - new Date(b.created_at),
            );
            for (let post of posts.value) {
                fetchPostReactions(post.id);
            }
            updateDisplayedPosts();
        } else {
            console.error("Ошибка при загрузке постов");
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

async function fetchAvailableReactions() {
    try {
        const response = await fetch(
            "http://localhost:3000/available-reactions",
        );
        if (response.ok) {
            availableReactions.value = await response.json();
        }
    } catch (error) {
        console.error("Ошибка при загрузке реакций:", error);
    }
}

async function fetchPostReactions(postId) {
    try {
        const response = await fetch(
            `http://localhost:3000/posts/${postId}/reactions`,
        );
        if (response.ok) {
            const postReactions = await response.json();
            reactions.value[postId] = postReactions;
        }
    } catch (error) {
        console.error("Ошибка при загрузке реакций:", error);
    }
}

async function updateTopic() {
    if (!authStore.isAuthorized) {
        errorMessages.value.push({
            content: "Необходимо войти в аккаунт",
            id: errorId.value++,
        });
        return;
    }

    const response = await fetch(
        "http://localhost:3000/topics/" + topic.value.id,
        {
            method: "PATCH",
            headers: {
                Authorization: `Bearer ${authStore.token}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                category_id: selectedCategory.value.id,
                name: editedTopicName.value,
            }),
        },
    );

    if (response.ok) {
        await fetchTopic();
    } else {
        switch (response.status) {
            case 400:
                errorMessages.value.push({
                    content: "Введены некорректные данные",
                    id: errorId.value++,
                });
                break;
            case 500:
                errorMessages.value.push({
                    content: "Произошла ошибка",
                    id: errorId.value++,
                });
                break;
        }
    }
}

async function createPost() {
    try {
        if (!authStore.isAuthorized) {
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch("http://localhost:3000/posts", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                Authorization: `Bearer ${authStore.token}`,
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
            switch (response.status) {
                case 400:
                    errorMessages.value.push({
                        content: "Введены некорректные данные",
                        id: errorId.value++,
                    });
                    break;
                case 500:
                    errorMessages.value.push({
                        content: "Произошла ошибка",
                        id: errorId.value++,
                    });
                    break;
            }
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

async function deletePost(postId) {
    try {
        if (!authStore.isAuthorized) {
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch(`http://localhost:3000/posts/${postId}`, {
            method: "DELETE",
            headers: {
                Authorization: `Bearer ${authStore.token}`,
            },
        });

        if (response.ok) {
            first.value = 0;
            await fetchPosts();
        } else {
            errorMessages.value.push({
                content: "Ошибка при удалении поста",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка при удалении поста:", error);
    }
}

function openEditPostDialog(post) {
    editedPostId.value = post.id;
    editedPostText.value = post.text;
    showEditDialog.value = true;
}

async function updatePost() {
    try {
        if (!authStore.isAuthorized) {
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch(
            `http://localhost:3000/posts/${editedPostId.value}`,
            {
                method: "PATCH",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: `Bearer ${authStore.token}`,
                },
                body: JSON.stringify({
                    text: editedPostText.value,
                }),
            },
        );

        if (response.ok) {
            first.value = 0;
            showEditDialog.value = false;
            await fetchPosts();
        } else {
            errorMessages.value.push({
                content: "Ошибка при обновлении поста",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка при обновлении поста:", error);
    }
}

async function addReaction(postId, reactionId) {
    try {
        if (!authStore.isAuthorized) {
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch(
            `http://localhost:3000/posts/${postId}/reactions/${reactionId}`,
            {
                method: "POST",
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
            },
        );

        if (response.ok) {
            await fetchPostReactions(postId);
        } else {
            const error = await response.json();
            errorMessages.value.push({
                content: error.err || "Ошибка при добавлении реакции",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка при добавлении реакции:", error);
        errorMessages.value.push({
            content: "Произошла ошибка при добавлении реакции",
            id: errorId.value++,
        });
    }
}

async function removeReaction(postId, reactionId) {
    try {
        if (!authStore.isAuthorized) {
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch(
            `http://localhost:3000/posts/${postId}/reactions/${reactionId}`,
            {
                method: "DELETE",
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
            },
        );

        if (response.ok) {
            await fetchPostReactions(postId);
        } else {
            const error = await response.json();
            errorMessages.value.push({
                content: error.err || "Ошибка при удалении реакции",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка при удалении реакции:", error);
        errorMessages.value.push({
            content: "Произошла ошибка при удалении реакции",
            id: errorId.value++,
        });
    }
}

function hasUserReaction(postId, reactionId) {
    if (!reactions.value[postId]) return false;
    return reactions.value[postId].some(
        (r) =>
            r.reaction_id === reactionId && r.author_id === getCurrentUserId(),
    );
}

function getCurrentUserId() {
    if (!authStore.isAuthorized) return null;
    try {
        const payload = JSON.parse(atob(authStore.token.split(".")[1]));
        return payload.user_id;
    } catch (e) {
        return null;
    }
}

function toggleReaction(postId, reactionId) {
    if (hasUserReaction(postId, reactionId)) {
        removeReaction(postId, reactionId);
    } else {
        addReaction(postId, reactionId);
    }
}

async function fetchCategories() {
    try {
        const response = await fetch("http://localhost:3000/topics-categories");

        if (response.ok) {
            topicCategories.value = await response.json();
        } else {
            console.error(response.status, await response.json());
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

function updateDisplayedPosts() {
    const startIndex = first.value;
    const endIndex = Math.min(
        startIndex + rowsPerPage.value,
        posts.value.length,
    );
    displayedPosts.value = posts.value.slice(startIndex, endIndex);
}

function onPage(event) {
    first.value = event.first;
    rowsPerPage.value = event.rows;
    updateDisplayedPosts();
}
</script>

<template>
    <div class="page-wrapper">
        <div class="topic-info">
            <div class="topic-header">
                <InputText type="text" v-model="editedTopicName" />
                <div class="topic-details">
                    <p>Создатель: {{ topic.creator?.login }}</p>
                    <p v-if="topic.created_at" class="topic-date">
                        Создано: {{ formatRelativeTime(topic.created_at) }}
                    </p>
                    <Select
                        v-model="selectedCategory"
                        :options="topicCategories"
                        optionLabel="name"
                        placeholder="Выберите категорию"
                    />
                    <Button @click="updateTopic" label="Обновить топик" />
                </div>
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

        <div class="posts-container" v-if="posts.length">
            <div class="post" v-for="post in displayedPosts" :key="post.id">
                <div class="post-header">
                    <div class="post-header-info">
                        <span class="author">{{ post.sender.login }}</span>
                        <span
                            class="post-date"
                            :title="formatDate(post.created_at)"
                        >
                            {{ formatRelativeTime(post.created_at) }}
                        </span>
                    </div>
                    <div class="post-actions">
                        <Button
                            icon="pi pi-pencil"
                            @click="openEditPostDialog(post)"
                            text
                            rounded
                        />
                        <Button
                            icon="pi pi-trash"
                            @click="deletePost(post.id)"
                            text
                            rounded
                            severity="danger"
                        />
                    </div>
                </div>
                <div class="post-content">{{ post.text }}</div>
                <div class="reactions">
                    <div
                        v-for="reaction in availableReactions"
                        :key="reaction.id"
                    >
                        <Button
                            :label="reaction.reaction"
                            :severity="
                                hasUserReaction(post.id, reaction.id)
                                    ? 'success'
                                    : 'secondary'
                            "
                            text
                            rounded
                            @click="toggleReaction(post.id, reaction.id)"
                        />
                        <span v-if="reactions[post.id]">
                            {{
                                reactions[post.id].filter(
                                    (r) => r.reaction_id === reaction.id,
                                ).length
                            }}
                        </span>
                    </div>
                </div>
            </div>
        </div>

        <div class="send-message-container">
            <div class="send-post-form">
                <Textarea
                    placeholder="Текст поста"
                    v-model="newPostText"
                    autoResize
                    rows="5"
                />
                <Button @click="createPost" label="Отправить" />
            </div>
        </div>

        <Paginator
            v-model:first="first"
            v-model:rows="rowsPerPage"
            :totalRecords="totalPosts"
            :rowsPerPageOptions="[5, 10, 20, 30]"
            @page="onPage"
            template="FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink RowsPerPageDropdown"
            class="paginator"
        />

        <Dialog
            v-model:visible="showEditDialog"
            modal
            header="Редактировать пост"
        >
            <div class="edit-post-form">
                <Textarea
                    v-model="editedPostText"
                    autoResize
                    rows="5"
                    style="width: 100%"
                />
                <Button label="Сохранить" @click="updatePost" />
            </div>
        </Dialog>
    </div>
</template>

<style scoped>
.page-wrapper {
    width: 100%;
}

.topic-info {
    background-color: #222222;
    border-radius: 7px;
    padding: 20px;
    margin-bottom: 10px;
}

.topic-header {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.topic-details {
    display: flex;
    gap: 10px;
    align-items: center;
}

.topic-date {
    color: #888;
    font-size: 0.9em;
}

.posts-container {
    display: flex;
    flex-direction: column;
    gap: 13px;
    width: 100%;
}

.post {
    background-color: #222222;
    border-radius: 7px;
    padding: 15px;
}

.post-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
}

.post-header-info {
    display: flex;
    flex-direction: column;
    gap: 5px;
}

.author {
    font-weight: bold;
}

.post-date {
    color: #888;
    font-size: 0.9em;
}

.post-actions {
    display: flex;
    gap: 5px;
}

.post-content {
    margin-bottom: 10px;
}

.reactions {
    display: flex;
    gap: 10px;
    margin-top: 10px;
}

.send-post-form {
    display: flex;
    flex-direction: column;
    gap: 13px;
    width: 100%;
}

.send-message-container {
    background-color: #222222;
    border-radius: 7px;
    padding: 20px;
    margin-top: 10px;
}

.edit-post-form {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 20px;
}

.errors-container {
    margin-bottom: 15px;
}
</style>
