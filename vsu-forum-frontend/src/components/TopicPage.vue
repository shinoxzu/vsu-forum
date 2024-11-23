<script setup>
import { ref, onMounted, watch } from "vue";
import { useRoute } from "vue-router";
import { Form } from "@primevue/forms";
import InputText from "primevue/inputtext";
import Button from "primevue/button";
import { Select } from "primevue";
import {Dialog} from "primevue";
import Textarea from 'primevue/textarea';

const route = useRoute();
const topic = ref({});
const posts = ref([]);
const newPostText = ref("");
const err = ref(false)
const newTopicName = ref(topic.value?.name)
const selectedCategory = ref(null)
const categories = ref([])
const edited = ref(0)

watch([topic, selectedCategory], () => {
    edited.value++
}, {deep: true} )


onMounted(() => {
    fetchTopic();
    fetchPosts();
    fetchCategories()
});

async function fetchTopic() {
    edited.value++
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

const  fetchPosts = async () => {
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
        err.value = true
    }
}

const updateTopic = async () => {
    console.log(newTopicName.value)
    console.log(selectedCategory.value)
    const token = localStorage.getItem("token");
    const resp = await fetch("http://localhost:3000/topics/" + topic.value.id, {
        method: "PUT",
        headers: {
            "Authorization" : `Bearer ${token}`,
            "Content-Type": "application/json",
        },
        body: JSON.stringify({category_id: selectedCategory.value.id, name: topic.value.name})
    })
    if (!resp.ok) {
        console.error(resp.status)
        err.value = true
    } else {
        edited.value = 3
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

const fetchCategories = async () => {
    edited.value++
    try {
        const response = await fetch("http://localhost:3000/topics-categories");
        if (!response.ok) {
            console.error(response.status, await response.json())
        } else {
            categories.value = await response.json();
            selectedCategory.value = categories.value.find(c => topic.value.category_id == c.id)
            console.log(categories.value)
        }

    } catch (e) {
        console.error(e)
    }
}


</script>

<template>
    <div class="topic-page">
        <div class="topic-info">
            <InputText type="text" v-model="topic.name" ></InputText>
            <p>Автор ID: {{ topic.author_id }}</p>
            <Select placeholder="Категория:" :options="categories" optionLabel="name" v-model="selectedCategory" style="width: 160px;"/>
            <Button v-show="edited > 3" @click="updateTopic">Обновить</Button>
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
    <Dialog v-model:visible="err" closable position="top"><p>Упс! Что-то пошло не так</p></Dialog>
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
