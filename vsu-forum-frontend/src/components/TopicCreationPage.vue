<script setup>
import { ref, onMounted } from "vue";
import { Form } from "@primevue/forms";
import InputText from "primevue/inputtext";
import AutoComplete from "primevue/autocomplete";
import Button from "primevue/button";
import Message from "primevue/message";
import { useRouter } from "vue-router";

const router = useRouter();
const newTopicName = ref(null);
const selectedTopicCategory = ref(null);
const topicCategories = ref([]);
const filteredTopicCategories = ref([]);
const errorMessages = ref([]);
const errorId = ref(0);

function search(event) {
    setTimeout(() => {
        if (!event.query.trim().length) {
            filteredTopicCategories.value = [...topicCategories.value];
        } else {
            filteredTopicCategories.value = topicCategories.value.filter(
                (category) => {
                    return category.name
                        .toLowerCase()
                        .startsWith(event.query.toLowerCase());
                },
            );
        }
    }, 250);
}

async function fetchCategories() {
    try {
        const response = await fetch("http://localhost:3000/topics-categories");

        if (response.ok) {
            topicCategories.value = await response.json();
            filteredTopicCategories.value = [...topicCategories.value];
        } else {
            console.error("Ошибка при загрузке категорий");
            topicCategories.value = [];
            filteredTopicCategories.value = [];
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

async function createCategory() {
    errorMessages.value = [];

    if (!selectedTopicCategory.value) return;

    try {
        const token = localStorage.getItem("token");
        if (!token) {
            console.error("Токен не найден, авторизация не выполнена.");
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch(
            "http://localhost:3000/topics-categories",
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: `Bearer ${token}`,
                },
                body: JSON.stringify({ name: selectedTopicCategory.value }),
            },
        );

        if (response.ok) {
            await fetchCategories();
            const data = await response.json();
            selectedTopicCategory.value = topicCategories.value.find(
                (cat) => cat.id === data.id,
            );
            search({ query: "" });
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
        console.error("Ошибка при создании категории:", error);
    }
}

async function createTopic() {
    errorMessages.value = [];

    if (!newTopicName.value || !selectedTopicCategory.value) return;

    try {
        const token = localStorage.getItem("token");
        if (!token) {
            console.error("Токен не найден, авторизация не выполнена.");
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch("http://localhost:3000/topics", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                Authorization: `Bearer ${token}`,
            },
            body: JSON.stringify({
                name: newTopicName.value,
                category_id: selectedTopicCategory.value.id,
            }),
        });

        if (response.ok) {
            const data = await response.json();
            router.push(`/topics/${data.id}`);
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
        console.error("Ошибка при создании топика:", error);
    }
}

onMounted(() => {
    fetchCategories();
});
</script>

<template>
    <div class="center-div">
        <h2>Создание топика</h2>

        <div v-if="errorMessages.length != 0" class="errors-container">
            <transition-group>
                <Message
                    icon="pi pi-times-circle"
                    v-for="msg of errorMessages"
                    :key="msg.id"
                    severity="error"
                    >{{ msg.content }}</Message
                >
            </transition-group>
        </div>

        <Form @submit="createTopic" class="simple-form">
            <InputText
                v-model="newTopicName"
                placeholder="Название топика"
                style="width: 100%"
                required
            />

            <AutoComplete
                v-model="selectedTopicCategory"
                placeholder="Категория"
                optionLabel="name"
                :suggestions="filteredTopicCategories"
                @complete="search"
                style="width: 100%"
                :input-style="{ width: '100%' }"
                required
            >
                <template #option="slotProps">
                    <div class="flex items-center">
                        <div>{{ slotProps.option.name }}</div>
                    </div>
                </template>
                <template #footer>
                    <div class="px-3 py-3">
                        <Button
                            label="Создать"
                            fluid
                            severity="secondary"
                            text
                            size="small"
                            icon="pi pi-plus"
                            @click="createCategory"
                        />
                    </div>
                </template>
            </AutoComplete>

            <Button type="submit" label="Создать топик" icon="pi pi-check" />
        </Form>
    </div>
</template>
