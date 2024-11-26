<script setup>
import { ref, onMounted } from "vue";

const username = ref("");

async function fetchProfile() {
    const token = localStorage.getItem("token");
    if (!token) {
        console.error("Токен не найден, авторизация не выполнена.");
        return;
    }

    try {
        const response = await fetch("http://localhost:3000/users/me", {
            headers: {
                Authorization: `Bearer ${token}`,
            },
        });

        if (response.ok) {
            const data = await response.json();
            username.value = data.login;
        } else {
            console.error("Ошибка при загрузке профиля");
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

onMounted(fetchProfile);
</script>

<template>
    <div class="profile">
        <h2>Профиль</h2>
        <p v-if="username">Логин: {{ username }}</p>
        <p v-else>Загрузка профиля...</p>
    </div>
</template>
